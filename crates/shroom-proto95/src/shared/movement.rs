use bit_struct::u1;
use shroom_meta::{id::FootholdId, twod::{Rect, Vec2}};
use shroom_pkt::{
    DecodePacket, EncodePacket, ShroomDurationMs16, ShroomList8, ShroomPacket, ShroomPacketEnum,
};


bit_struct::bit_struct! {
    pub struct ArrowKeyState(u8) {
        down: u1,
        up: u1,
        right: u1,
        left: u1,
    }
}

#[derive(Debug, Default)]
pub struct ArrowKeyStates(pub Vec<ArrowKeyState>);

//TODO verify the encoding order
impl ArrowKeyStates {
    pub fn as_bytes(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.chunks(2).map(|chunk| match chunk.len() {
            1 => chunk[0].raw(),
            2 => chunk[0].raw() | (chunk[1].raw() << 4),
            _ => unreachable!(),
        })
    }

    pub fn from_bytes(bytes: &[u8], len: usize) -> Self {
        Self(
            bytes
                .iter()
                .flat_map(|&b| [b & 0xF, b >> 4].into_iter())
                .map(|b| ArrowKeyState::try_from(b).unwrap())
                .take(len)
                .collect(),
        )
    }

    fn byte_len(len: usize) -> usize {
        (len + 1) / 2
    }
}

impl EncodePacket for ArrowKeyStates {
    const SIZE_HINT: shroom_pkt::SizeHint = shroom_pkt::SizeHint::NONE;

    fn encode_len(&self) -> usize {
        Self::byte_len(self.0.len())
    }

    fn encode<B: bytes::BufMut>(
        &self,
        pw: &mut shroom_pkt::PacketWriter<B>,
    ) -> shroom_pkt::PacketResult<()> {
        (self.0.len() as u8).encode(pw)?;
        self.as_bytes().try_for_each(|b| b.encode(pw))
    }
}

impl<'de> DecodePacket<'de> for ArrowKeyStates {
    fn decode(pr: &mut shroom_pkt::PacketReader<'de>) -> shroom_pkt::PacketResult<Self> {
        let len = pr.read_u8()? as usize;
        let bytes = pr.read_bytes(Self::byte_len(len))?;
        Ok(Self::from_bytes(bytes, len))
    }
}

#[derive(ShroomPacket, Debug)]
pub struct MovePassiveInfo {
    pub key_pad_state: ArrowKeyStates,
    pub bounds: Rect,
}

/*
shroom_enum_code!(
    MovementState,
    u8,
    LeftWalk = 3,
    RightWalk = 2,
    LeftStanding = 5,
    RightStanding = 4,
    LeftFalling = 7,
    RightFalling = 6,
    LeftAttack = 9,
    RightAttack = 8,
    LeftProne = 11,
    RightProne = 10,
    LeftRope = 13,
    RightRope = 12,
    LeftLadder = 15,
    RightLadder = 14
);*/
pub type MovementAction = u8;

#[derive(Debug, ShroomPacket)]
pub struct MovementFooter {
    pub action: MovementAction,
    pub dur: ShroomDurationMs16,
}

#[derive(Debug, ShroomPacket)]
pub struct AbsoluteMovement {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub fh: FootholdId,
    pub offset: Vec2,
    pub footer: MovementFooter,
}

impl AbsoluteMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        Some((self.pos, Some(self.fh)))
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}
#[derive(Debug, ShroomPacket)]
pub struct AbsoluteFallMovement {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub fh: FootholdId,
    pub fh_fall_start: FootholdId,
    pub offset: Vec2,
    pub footer: MovementFooter,
}

impl AbsoluteFallMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        Some((self.pos, Some(self.fh)))
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}

#[derive(Debug, ShroomPacket)]
pub struct RelativeMovement {
    pub velocity: Vec2,
    pub footer: MovementFooter,
}

impl RelativeMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        None
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}

#[derive(Debug, ShroomPacket)]
pub struct InstantMovement {
    pub pos: Vec2,
    pub fh: FootholdId,
    pub footer: MovementFooter,
}

impl InstantMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        Some((self.pos, Some(self.fh)))
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}

#[derive(Debug, ShroomPacket)]
pub struct FallDownMovement {
    pub velocity: Vec2,
    pub fh_fall_start: FootholdId,
    pub footer: MovementFooter,
}

impl FallDownMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        None
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}

#[derive(Debug, ShroomPacket)]
pub struct FlyingMovement {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub footer: MovementFooter,
}

impl FlyingMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        Some((self.pos, None))
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}

#[derive(Debug, ShroomPacket)]
pub struct UnknownMovement {
    pub footer: MovementFooter,
}

impl UnknownMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        None
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum Movement {
    Normal(AbsoluteMovement) = 0,
    Jump(RelativeMovement) = 1,
    Impact(RelativeMovement) = 2,
    Immediate(InstantMovement) = 0x3,
    Teleport(InstantMovement) = 0x4,
    HangOnBack(AbsoluteMovement) = 5,
    Assaulter(InstantMovement) = 0x6,
    Assassinate(InstantMovement) = 0x7,
    Rush(InstantMovement) = 0x8,
    StatChange(bool) = 0x9, // bool is if stat changed
    SitDown(InstantMovement) = 0xA,
    StartFallDown(FallDownMovement) = 0xB,
    FallDown(AbsoluteFallMovement) = 0xC,
    StartWings(RelativeMovement) = 0xD,
    Wings(AbsoluteMovement) = 0xE,
    //0xF ?? -> ara adjust?
    MobToss(RelativeMovement) = 0x10,
    FlyingBlock(FlyingMovement) = 0x11,
    DashSlide(RelativeMovement) = 0x12,
    // 0x13 -> bmag adjust?
    FlashJump(UnknownMovement) = 0x14,
    RocketBooster(UnknownMovement) = 0x15,
    BackstepShot(UnknownMovement) = 0x16,
    MobPowerKnockback(UnknownMovement) = 0x17,
    VerticalJump(UnknownMovement) = 0x18,
    CustomImpact(UnknownMovement) = 0x19,
    CombatStep(UnknownMovement) = 0x1A,
    Hit(UnknownMovement) = 0x1B,
    TimeBombAttack(UnknownMovement) = 0x1C,
    SnowballTouch(UnknownMovement) = 0x1D,
    BuffZoneEffect(UnknownMovement) = 0x1E,
    MobLadder(RelativeMovement) = 0x1F,
    MobRightAngle(RelativeMovement) = 0x20,
    MobStopNodeStart(RelativeMovement) = 0x21,
    MobBeforeNode(RelativeMovement) = 0x22,
    MobAttackRush(AbsoluteMovement) = 0x23,
    MobAttackRushStop(AbsoluteMovement) = 0x24,
}

impl Movement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        match self {
            Self::Normal(mv) => mv.get_pos_fh(),
            Self::MobAttackRush(mv) => mv.get_pos_fh(),
            Self::MobAttackRushStop(mv) => mv.get_pos_fh(),
            Self::Jump(mv) => mv.get_pos_fh(),
            Self::Impact(mv) => mv.get_pos_fh(),
            Self::Immediate(mv) => mv.get_pos_fh(),
            Self::Teleport(mv) => mv.get_pos_fh(),
            Self::HangOnBack(mv) => mv.get_pos_fh(),
            Self::Assaulter(mv) => mv.get_pos_fh(),
            Self::Assassinate(mv) => mv.get_pos_fh(),
            Self::Rush(mv) => mv.get_pos_fh(),
            Self::StatChange(_mv) => None,
            Self::SitDown(mv) => mv.get_pos_fh(),
            Self::StartFallDown(mv) => mv.get_pos_fh(),
            Self::FallDown(mv) => mv.get_pos_fh(),
            Self::StartWings(mv) => mv.get_pos_fh(),
            Self::Wings(mv) => mv.get_pos_fh(),
            Self::MobToss(mv) => mv.get_pos_fh(),
            Self::FlyingBlock(mv) => mv.get_pos_fh(),
            Self::DashSlide(mv) => mv.get_pos_fh(),
            Self::FlashJump(mv) => mv.get_pos_fh(),
            Self::RocketBooster(mv) => mv.get_pos_fh(),
            Self::BackstepShot(mv) => mv.get_pos_fh(),
            Self::MobPowerKnockback(mv) => mv.get_pos_fh(),
            Self::VerticalJump(mv) => mv.get_pos_fh(),
            Self::CustomImpact(mv) => mv.get_pos_fh(),
            Self::CombatStep(mv) => mv.get_pos_fh(),
            Self::Hit(mv) => mv.get_pos_fh(),
            Self::TimeBombAttack(mv) => mv.get_pos_fh(),
            Self::SnowballTouch(mv) => mv.get_pos_fh(),
            Self::BuffZoneEffect(mv) => mv.get_pos_fh(),
            Self::MobLadder(mv) => mv.get_pos_fh(),
            Self::MobRightAngle(mv) => mv.get_pos_fh(),
            Self::MobStopNodeStart(mv) => mv.get_pos_fh(),
            Self::MobBeforeNode(mv) => mv.get_pos_fh(),
        }
    }

    pub fn get_footer(&self) -> Option<&MovementFooter> {
        match self {
            Self::Normal(mv)
            | Self::MobAttackRush(mv)
            | Self::MobAttackRushStop(mv)
            | Self::HangOnBack(mv) => Some(mv.get_footer()),
            Self::Jump(mv) | Self::Impact(mv) => Some(mv.get_footer()),
            Self::Immediate(mv) | Self::Teleport(mv) => Some(mv.get_footer()),
            Self::SitDown(mv) => Some(mv.get_footer()),
            Self::Assaulter(mv) | Self::Assassinate(mv) | Self::Rush(mv) => {
                Some(mv.get_footer())
            }
            Self::StartFallDown(mv) => Some(mv.get_footer()),
            Self::FallDown(mv) => Some(mv.get_footer()),
            Self::StartWings(mv) => Some(mv.get_footer()),
            Self::Wings(mv) => Some(mv.get_footer()),
            Self::MobToss(mv) => Some(mv.get_footer()),
            Self::FlyingBlock(mv) => Some(mv.get_footer()),
            Self::DashSlide(mv) => Some(mv.get_footer()),
            Self::FlashJump(mv) => Some(mv.get_footer()),
            Self::RocketBooster(mv) => Some(mv.get_footer()),
            Self::BackstepShot(mv) => Some(mv.get_footer()),
            Self::MobPowerKnockback(mv) => Some(mv.get_footer()),
            Self::VerticalJump(mv) => Some(mv.get_footer()),
            Self::CustomImpact(mv) => Some(mv.get_footer()),
            Self::CombatStep(mv) => Some(mv.get_footer()),
            Self::Hit(mv) => Some(mv.get_footer()),
            Self::TimeBombAttack(mv) => Some(mv.get_footer()),
            Self::SnowballTouch(mv) => Some(mv.get_footer()),
            Self::BuffZoneEffect(mv) => Some(mv.get_footer()),
            Self::MobLadder(mv) => Some(mv.get_footer()),
            Self::MobRightAngle(mv) => Some(mv.get_footer()),
            Self::MobStopNodeStart(mv) => Some(mv.get_footer()),
            Self::MobBeforeNode(mv) => Some(mv.get_footer()),
            Self::StatChange(_) => None,
        }
    }
}

#[derive(ShroomPacket, Debug)]
pub struct MovePath {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub moves: ShroomList8<Movement>,
}

impl MovePath {
    pub fn get_last_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        self.moves.iter().rev().find_map(Movement::get_pos_fh)
    }
}

#[derive(ShroomPacket, Debug)]
pub struct MovePassivePath {
    pub path: MovePath,
    pub passive_info: MovePassiveInfo,
}
