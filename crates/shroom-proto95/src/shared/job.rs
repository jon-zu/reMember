use shroom_pkt::ShroomPacket;

use shroom_meta::id::job_id::{JobId, SubJob};

#[derive(ShroomPacket, Debug)]
pub struct Job {
    pub job_id: JobId,
    pub sub_job: SubJob,
}

impl Job {
    pub fn new(job_id: JobId, sub_job: SubJob) -> Self {
        Self { job_id, sub_job }
    }
}
