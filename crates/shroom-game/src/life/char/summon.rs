use shroom_meta::{
    id::{CharacterId, FootholdId, ObjectId, SkillId},
    twod::Vec2,
};
use shroom_proto95::game::life::summon::{
        SummonAssistType, SummonCreateResp, SummonDeleteResp, SummonEnterType, SummonInitData,
        SummonLeaveType, SummonMoveAbility, SummonMoveAction,
    };
use shroom_srv::{game::pool::PoolItem, GameTime};


#[derive(Debug, Clone)]
pub struct Summon {
    pub pos: Vec2,
    pub fh: FootholdId,
    pub skill_id: SkillId,
    pub skill_level: u8,
    pub char_level: u8,
    pub char_id: CharacterId,
    pub move_ability: shroom_meta::class::SummonMoveAbility,
    pub assist_type: shroom_meta::class::SummonAssistType,
    pub expiration: GameTime
}

fn map_summon_move(s: &shroom_meta::class::SummonMoveAbility) -> SummonMoveAbility {
    match s {
        shroom_meta::class::SummonMoveAbility::Fly => SummonMoveAbility::FlyRandom,
        shroom_meta::class::SummonMoveAbility::Walk => SummonMoveAbility::WalkRandom,
        shroom_meta::class::SummonMoveAbility::Follow => SummonMoveAbility::Follow,
        shroom_meta::class::SummonMoveAbility::CircleFollow => SummonMoveAbility::CircleFollow,
        shroom_meta::class::SummonMoveAbility::Escort => SummonMoveAbility::Escort,
        shroom_meta::class::SummonMoveAbility::Jump => SummonMoveAbility::Jump,
        shroom_meta::class::SummonMoveAbility::None => SummonMoveAbility::NoMove, 
    }
}

fn map_summon_assists(s: &shroom_meta::class::SummonAssistType) -> SummonAssistType {
    match s {
        shroom_meta::class::SummonAssistType::Attack => SummonAssistType::Attack,
        shroom_meta::class::SummonAssistType::Heal => SummonAssistType::Heal,
        shroom_meta::class::SummonAssistType::None => SummonAssistType::None,
        shroom_meta::class::SummonAssistType::AttackExtra1 => SummonAssistType::AttackEx,
        shroom_meta::class::SummonAssistType::AttackExtra2 => SummonAssistType::AttackEx2,
        shroom_meta::class::SummonAssistType::ManualAttack => SummonAssistType::ManualAttack,
    }
}

impl PoolItem for Summon {
    type Id = ObjectId;
    type EnterMsg = SummonCreateResp;
    type LeaveMsg = SummonDeleteResp;

    type LeaveParam = ();

    fn enter_msg(&self, id: Self::Id, _t: GameTime) -> Self::EnterMsg {
        SummonCreateResp {
            char: self.char_id,
            summon_id: id,
            skill_id: self.skill_id,
            char_level: self.char_level,
            skill_level: self.skill_level,
            init: SummonInitData {
                pos: self.pos,
                move_action: SummonMoveAction::Walk, // TODO
                cur_fh: self.fh,
                move_ability: map_summon_move(&self.move_ability),
                assist_type: map_summon_assists(&self.assist_type),
                enter_type: SummonEnterType::CreateSummon,
                avatar: None.into(),
            },
        }
    }

    fn leave_msg(&self, id: Self::Id, _param: Self::LeaveParam) -> Self::LeaveMsg {
        SummonDeleteResp {
            char: self.char_id,
            summon_id: id,
            leave: SummonLeaveType::Default,
        }
    }
}