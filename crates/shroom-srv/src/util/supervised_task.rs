use std::{
    sync::{atomic::AtomicUsize, Arc, Mutex},
    time::Duration,
};

use futures::Future;

pub trait SupervisedTask {
    type Context: Send + 'static;
    fn run(
        &mut self,
        ctx: &mut Self::Context,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

#[derive(Default)]
struct Shared {
    fails: AtomicUsize,
    last_error: Mutex<Option<anyhow::Error>>,
}

pub struct SupervisedTaskHandle {
    task: tokio::task::JoinHandle<()>,
    shared: Arc<Shared>,
}

impl SupervisedTaskHandle {
    #[must_use]
    pub fn fails(&self) -> usize {
        self.shared.fails.load(std::sync::atomic::Ordering::Relaxed)
    }

    #[must_use]
    pub fn last_error(&mut self) -> Option<anyhow::Error> {
        self.shared.last_error.lock().unwrap().take()
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.task.is_finished()
    }
}

impl Drop for SupervisedTaskHandle {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl SupervisedTaskHandle {
    pub fn spawn<T>(mut task: T, mut ctx: T::Context, delay: Duration) -> Self
    where
        T: SupervisedTask + Send + 'static,
    {
        let shared: Arc<Shared> = Arc::default();

        let s = shared.clone();
        let task = tokio::spawn(async move {
            loop {
                match task.run(&mut ctx).await {
                    Ok(()) => break,
                    Err(e) => {
                        log::error!("task failed: {}", e);
                        s.fails.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        s.last_error.lock().unwrap().replace(e);

                        // TODO backoff + quit after too many fails
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        });

        Self { task, shared }
    }
}

/*
    Macro to create a super vised task from a function like:
    `supervised_task!(TestTask, mpsc::Receiver<u32>, test_task);`

    For now this helper is required due to limitations of lifetimes
*/
#[macro_export]
macro_rules! supervised_task {
    ($task:ident, $ctx:ty, $f:ident) => {
        pub struct $task;

        impl $crate::util::supervised_task::SupervisedTask for $task {
            type Context = $ctx;

            fn run(
                &mut self,
                ctx: &mut Self::Context,
            ) -> impl Future<Output = Result<(), anyhow::Error>> + Send {
                $f(ctx)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use super::*;

    async fn test_task(rx: &mut mpsc::Receiver<u32>) -> anyhow::Result<()> {
        let val = rx.recv().await.unwrap();
        anyhow::bail!("Failed with: {}", val);
    }
    supervised_task!(TestTask, mpsc::Receiver<u32>, test_task);

    #[tokio::test]
    async fn supervisor() {
        let restart_delay = Duration::from_millis(10);
        let (tx, rx) = mpsc::channel::<u32>(1);
        let mut task = SupervisedTaskHandle::spawn(TestTask, rx, restart_delay);

        assert_eq!(task.fails(), 0);
        assert!(task.last_error().is_none());

        tx.try_send(0).unwrap();

        // Wait for restart
        tokio::time::sleep(restart_delay).await;
        tx.try_send(0).unwrap();
        assert_eq!(task.fails(), 1);
    }
}
