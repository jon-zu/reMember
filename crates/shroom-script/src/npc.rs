use std::pin::Pin;

use futures::Future;
use shroom_meta::{
    fmt::{ShroomDisplay, ShroomMenuItem, ShroomMenuList},
    id::{job_id::JobId, FieldId, ItemId, Money},
    QuestDataId,
};
use shroom_proto95::game::script::{
    AskMsg, AskNumberMsg, AskTextMsg, MsgParamFlags, OptionAnswer, SayMsg, ScriptAnswerReq,
    ScriptMessage,
};

use crate::{
    poll_state::{self, StateRef},
    BoxedNpcPlugin, BoxedSessionCtx,
};

pub trait QuestData: Sized {
    const ID: QuestDataId;

    fn to_data(self) -> Vec<u8>;
    fn from_data(data: &[u8]) -> anyhow::Result<Self>;
}

pub trait EnumQuestData: TryFrom<u32> + Into<u32> {
    const ID: QuestDataId;
}

impl<T: EnumQuestData> QuestData for T
where
    <T as TryFrom<u32>>::Error: std::error::Error + Send + Sync + 'static,
{
    const ID: QuestDataId = T::ID;

    fn to_data(self) -> Vec<u8> {
        let v: u32 = self.into();
        v.to_le_bytes().to_vec()
    }

    fn from_data(data: &[u8]) -> anyhow::Result<Self> {
        Ok(T::try_from(u32::from_le_bytes(data.try_into()?))?)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NpcAction {
    Start,
    Next,
    Prev,
    Selection(usize),
    InputTxt(String),
    InputNum(u32),
    YesNo(bool),
    AvatarSelection(usize),
    PetSelection(usize),
    SliderValue(u32),
    End,
}

impl From<ScriptAnswerReq> for NpcAction {
    fn from(req: ScriptAnswerReq) -> Self {
        match req {
            ScriptAnswerReq::PrevNext(OptionAnswer(Some(true))) => Self::Next,
            ScriptAnswerReq::PrevNext(OptionAnswer(Some(false))) => Self::Prev,
            ScriptAnswerReq::ImgNext(OptionAnswer(Some(true))) => Self::Next,
            ScriptAnswerReq::YesNo(OptionAnswer(Some(v))) => Self::YesNo(v),
            ScriptAnswerReq::InputText(v) if v.is_some() => {
                Self::InputTxt(v.as_ref().unwrap().clone())
            }
            ScriptAnswerReq::InputNumber(v) if v.is_some() => Self::InputNum(*v.as_ref().unwrap()),
            ScriptAnswerReq::InputSelection(v) if v.is_some() => {
                Self::Selection(v.unwrap() as usize)
            }
            ScriptAnswerReq::AvatarSelection(v) if v.is_some() => {
                Self::AvatarSelection(v.unwrap() as usize)
            }
            ScriptAnswerReq::AvatarMembershipSelection(v) if v.is_some() => {
                Self::AvatarSelection(v.unwrap() as usize)
            }
            ScriptAnswerReq::PetSelection(v) if v.is_some() => {
                Self::PetSelection(v.unwrap() as usize)
            }
            ScriptAnswerReq::InputBoxText(v) if v.is_some() => {
                Self::InputTxt(v.as_ref().unwrap().clone())
            }
            ScriptAnswerReq::InputSliderValue(v) if v.is_some() => Self::SliderValue(v.unwrap()),
            _ => Self::End,
        }
    }
}

pub type NpcResult<T> = Result<T, anyhow::Error>;
pub type NpcCtx = StateRef<BoxedSessionCtx, NpcAction>;

impl NpcCtx {
    pub fn meta(&self) -> &'static shroom_meta::MetaService {
        self.with(|c| c.meta())
    }

    pub fn get_quest_data<T: QuestData>(&self) -> anyhow::Result<Option<T>> {
        self.with(|c| {
            let data = c.get_quest_state_data(T::ID);
            data.map(|d| T::from_data(&d)).transpose()
        })
    }

    pub fn get_or_default_quest_data<T: QuestData + Default>(&mut self) -> anyhow::Result<T> {
        if let Some(data) = self.get_quest_data::<T>()? {
            return Ok(data);
        }

        self.set_quest_data(T::default())?;
        Ok(self.get_quest_data::<T>()?.unwrap())
    }

    pub fn set_quest_data<T: QuestData>(&mut self, data: T) -> anyhow::Result<()> {
        self.with_mut(|c| c.set_quest_state_data(T::ID, data.to_data()))
    }

    pub fn update_quest_data<T: QuestData + Clone>(&mut self, data: T) -> anyhow::Result<T> {
        self.with_mut(|c| c.set_quest_state_data(T::ID, data.clone().to_data()))?;
        Ok(data)
    }

    pub fn search_fields(&self, query: &str) -> Result<FieldId, Vec<(FieldId, String)>> {
        self.with(|c| c.search_fields(query))
    }

    pub fn transfer_field(&mut self, field: FieldId) -> NpcResult<()> {
        self.with_mut(|c| c.transfer_field(field));
        Ok(())
    }

    pub async fn wait_for_start(&mut self) -> anyhow::Result<()> {
        match self.next_input().await? {
            NpcAction::Start => Ok(()),
            _ => anyhow::bail!("Expected Start action"),
        }
    }

    pub async fn wait_for_next(&mut self) -> anyhow::Result<()> {
        let action = self.next_input().await?;
        if action != NpcAction::Next {
            anyhow::bail!("Expected next action");
        }

        Ok(())
    }

    pub async fn ask_text(
        &mut self,
        text: impl ShroomDisplay,
        min: u32,
        max: u32,
        default: String,
    ) -> anyhow::Result<String> {
        self.with_mut(|c| {
            c.send_msg(ScriptMessage::AskText(AskTextMsg {
                param: MsgParamFlags::empty(),
                msg: text.to_shroom_string(),
                default_txt: default,
                min: min as u16,
                max: max as u16,
            }))
        });

        match self.next_input().await? {
            NpcAction::InputTxt(v) if v.len() < min as usize || v.len() > max as usize => {
                anyhow::bail!("Invalid text: {v}, min {min}, max {max}")
            }
            NpcAction::InputTxt(v) => Ok(v),
            _ => anyhow::bail!("Expected InputTxt action"),
        }
    }

    pub async fn ask_number(
        &mut self,
        text: impl ShroomDisplay,
        min: u32,
        max: u32,
        default: u32,
    ) -> anyhow::Result<u32> {
        self.with_mut(|c| {
            c.send_msg(ScriptMessage::AskNumber(AskNumberMsg {
                param: MsgParamFlags::empty(),
                msg: text.to_shroom_string(),
                default_number: default,
                min,
                max,
            }))
        });

        match self.next_input().await? {
            NpcAction::InputNum(v) if v < min || v > max => {
                anyhow::bail!("Invalid number: {v}, min {min}, max {max}")
            }
            NpcAction::InputNum(v) => Ok(v),
            _ => anyhow::bail!("Expected InputNum action"),
        }
    }

    pub async fn ask_selection<T: ShroomDisplay>(
        &mut self,
        text: &str,
        items: ShroomMenuList<T>,
    ) -> anyhow::Result<usize> {
        let msg = format!("{}\n{}", text, items).to_shroom_string();
        self.with_mut(|c| {
            c.send_msg(ScriptMessage::AskMenu(AskMsg {
                param: MsgParamFlags::empty(),
                msg,
            }))
        });

        Ok(match self.next_input().await? {
            NpcAction::Selection(v) if v >= items.len() => {
                anyhow::bail!("Invalid selection: {v}, max {}", items.len())
            }
            NpcAction::Selection(v) => v,
            _ => anyhow::bail!("Expected YesNo action"),
        })
    }

    pub async fn ask_raw_selection<T: ShroomDisplay>(
        &mut self,
        text: T,
        max: usize,
    ) -> anyhow::Result<usize> {
        self.with_mut(|c| {
            c.send_msg(ScriptMessage::AskMenu(AskMsg {
                param: MsgParamFlags::empty(),
                msg: text.to_shroom_string(),
            }))
        });

        Ok(match self.next_input().await? {
            NpcAction::Selection(v) if v >= max => {
                anyhow::bail!("Invalid selection: {v}, max {}", max)
            }
            NpcAction::Selection(v) => v,
            _ => anyhow::bail!("Expected YesNo action"),
        })
    }

    pub async fn ask_grid<T: ShroomDisplay>(
        &mut self,
        title: &str,
        grid: &[T],
        rows: usize,
        cols: usize,
    ) -> anyhow::Result<usize> {
        use std::fmt::Write;

        // Format the grid
        let mut buf = String::new();
        writeln!(buf, "{title}\r\n")?;
        for y in 0..rows {
            for x in 0..cols {
                let ix = y * cols + x;
                write!(&mut buf, "{} ", ShroomMenuItem(ix, &grid[ix]))?;
            }
            buf += "\r\n";
        }

        self.with_mut(|c| {
            c.send_msg(ScriptMessage::AskMenu(AskMsg {
                param: MsgParamFlags::empty(),
                msg: buf.to_shroom_string(),
            }))
        });

        let max = rows * cols;
        Ok(match self.next_input().await? {
            NpcAction::Selection(v) if v >= max => {
                anyhow::bail!("Invalid selection: {v}, max {}", max)
            }
            NpcAction::Selection(v) => v,
            _ => anyhow::bail!("Expected YesNo action"),
        })
    }

    pub async fn ask_yes_no(&mut self, text: impl ShroomDisplay) -> anyhow::Result<bool> {
        self.with_mut(|c| {
            c.send_msg(ScriptMessage::AskYesNo(AskMsg {
                param: MsgParamFlags::empty(),
                msg: text.to_shroom_string(),
            }))
        });

        match self.next_input().await? {
            NpcAction::YesNo(b) => Ok(b),
            _ => anyhow::bail!("Expected YesNo action"),
        }
    }

    pub async fn say(&mut self, text: impl ShroomDisplay, last: bool) -> anyhow::Result<()> {
        self.with_mut(|c| {
            c.send_msg(ScriptMessage::Say(SayMsg {
                param: MsgParamFlags::empty(),
                txt: text.to_shroom_string(),
                has_prev: false,
                has_next: !last,
                speaker_tmpl_id: None.into(),
            }))
        });
        Ok(())
    }

    pub async fn say_next(&mut self, text: impl ShroomDisplay) -> anyhow::Result<()> {
        self.say(text, false).await?;
        self.wait_for_next().await?;
        Ok(())
    }

    pub async fn say_end(&mut self, text: impl ShroomDisplay) -> anyhow::Result<()> {
        self.say(text, true).await?;
        self.wait_for_next().await?;
        Ok(())
    }

    pub fn char_level(&self) -> u8 {
        self.with(|c| c.level())
    }

    pub fn set_char_level(&mut self, lvl: u8) {
        self.with_mut(|c| c.set_level(lvl));
    }

    pub fn has_item(&self, id: ItemId) -> bool {
        self.with(|c| c.has_item(id))
    }

    pub fn try_update_money(&mut self, money: i32) -> bool {
        self.with_mut(|c| c.update_money(money))
    }

    pub fn try_take_money(&mut self, money: Money) -> bool {
        self.with_mut(|c| c.update_money(-(money as i32)))
    }

    pub fn try_give_items(&mut self, ids: &[(ItemId, usize)]) -> anyhow::Result<bool> {
        self.with_mut(|c| c.try_give_items(ids))
    }

    pub fn try_give_item(&mut self, id: ItemId, count: usize) -> anyhow::Result<bool> {
        self.with_mut(|c| c.try_give_item(id, count))
    }

    pub fn has_item_quantity(&self, id: ItemId, count: usize) -> bool {
        self.with(|c| c.has_item_quantity(id, count))
    }

    pub fn try_take_all_items(&mut self, id: ItemId) -> anyhow::Result<usize> {
        self.with_mut(|c| c.try_take_all_items(id))
    }

    pub fn try_take_item(&mut self, id: ItemId, count: usize) -> anyhow::Result<bool> {
        self.with_mut(|c| c.try_take_item(id, count))
    }

    pub fn job(&self) -> JobId {
        self.with(|c| c.job())
    }

    pub fn set_job(&mut self, job: JobId) {
        self.with_mut(|c| c.set_job(job))
    }
}

pub trait NpcPlugin {
    fn init(&mut self, ctx: &mut BoxedSessionCtx) -> anyhow::Result<()>;
    fn step(&mut self, ctx: &mut BoxedSessionCtx, action: NpcAction) -> anyhow::Result<()>;
    fn is_finished(&self) -> bool;
}

pub type BoxedNpcFuture = Box<dyn Future<Output = anyhow::Result<()>>>;

pub struct FutureNpcPlugin<Fut>(Pin<Box<poll_state::StateHandle<Fut, BoxedSessionCtx, NpcAction>>>);

impl<Fut> FutureNpcPlugin<Fut> {
    pub fn launch<F>(script: F) -> BoxedNpcPlugin
    where
        F: FnOnce(StateRef<BoxedSessionCtx, NpcAction>) -> Fut,
        Fut: Future<Output = anyhow::Result<()>> + Send + 'static,
    {
        let state = poll_state::StateHandle::from_fn(script);
        Box::new(Self(Box::pin(state)))
    }
}

impl<Fut> NpcPlugin for FutureNpcPlugin<Fut>
where
    Fut: Future<Output = anyhow::Result<()>> + 'static,
{
    fn init(&mut self, ctx: &mut BoxedSessionCtx) -> anyhow::Result<()> {
        self.step(ctx, NpcAction::Start)
    }

    fn step(&mut self, ctx: &mut BoxedSessionCtx, action: NpcAction) -> anyhow::Result<()> {
        match poll_state::StateHandle::transition(self.0.as_mut(), Some(action), ctx) {
            Some(res) => res,
            None => Ok(()),
        }
    }

    fn is_finished(&self) -> bool {
        self.0.is_finished()
    }
}
