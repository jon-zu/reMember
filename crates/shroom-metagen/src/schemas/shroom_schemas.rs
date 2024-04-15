#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

pub mod error {
        pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Bool {
    Str(Str),
    Int(i64),
}
impl From<&Bool> for Bool {
    fn from(value: &Bool) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Bool {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Str(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Int(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for Bool {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Bool {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Bool {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for Bool {
    fn to_string(&self) -> String {
        match self {
            Self::Str(x) => x.to_string(),
            Self::Int(x) => x.to_string(),
        }
    }
}
impl From<Str> for Bool {
    fn from(value: Str) -> Self {
        Self::Str(value)
    }
}
impl From<i64> for Bool {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Canvas {
        #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub: Option<serde_json::Map<String, serde_json::Value>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
impl From<&Canvas> for Canvas {
    fn from(value: &Canvas) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharItem {
    #[serde(
        rename = "ActionAtk0",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub action_atk0: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "ActionAtk00",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub action_atk00: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "ActionAtk1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub action_atk1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub alert: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub angry: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "attackDefault",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub attack_default: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "attackDefaultSE",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub attack_default_se: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "backDefault",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub back_default: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub bewildered: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub blaze: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub blink: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub bowing: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub cheers: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub chu: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub coolingeffect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub cry: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub dam: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub default: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub default0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub default1: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "defaultAC",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub default_ac: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub despair: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub doublefire: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fake: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fireburner: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fly: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub glitter: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub heal: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub homing: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hot: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hum: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<CharItemInfo>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub jump: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ladder: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub love: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "mailArm",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub mail_arm: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub oops: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub pain: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub prone: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "proneStab",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub prone_stab: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "qBlue",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub q_blue: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub rope: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub shine: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub shoot1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub shoot2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "shootF",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub shoot_f: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub sit: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub smile: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "stabO1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub stab_o1: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "stabO2",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub stab_o2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "stabO3",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub stab_o3: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "stabOF",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub stab_of: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "stabT1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub stab_t1: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "stabT2",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub stab_t2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "stabTF",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub stab_tf: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stunned: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingO1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_o1: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingO2",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_o2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingO3",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_o3: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingOF",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_of: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingP1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_p1: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingP2",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_p2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingP3",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_p3: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingPF",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_pf: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingT1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_t1: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingT2",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_t2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingT3",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_t3: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "swingTF",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub swing_tf: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub troubled: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub vomit: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub walk1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub walk2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub weekly: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub wink: serde_json::Map<String, serde_json::Value>,
}
impl From<&CharItem> for CharItem {
    fn from(value: &CharItem) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acc: Option<i64>,
    #[serde(
        rename = "accountSharable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_sharable: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addition: Option<CharItemInfoAddition>,
    #[serde(
        rename = "afterImage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub after_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack: Option<i64>,
    #[serde(
        rename = "attackSpeed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_speed: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub blaze: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "bonusExp",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub bonus_exp: std::collections::HashMap<String, CharItemInfoBonusExpValue>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub bowing: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash: Option<Bool>,
    #[serde(
        rename = "chatBalloon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chat_balloon: Option<i64>,
    #[serde(rename = "consumeHP", default, skip_serializing_if = "Option::is_none")]
    pub consume_hp: Option<StrOrInt>,
    #[serde(rename = "consumeMP", default, skip_serializing_if = "Option::is_none")]
    pub consume_mp: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub despair: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "dropBlock", default, skip_serializing_if = "Option::is_none")]
    pub drop_block: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub durability: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<CharItemInfoEffect>,
    #[serde(
        rename = "elemDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub elem_default: Option<i64>,
    #[serde(
        rename = "enchantCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enchant_category: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epic: Option<CharItemInfoEpic>,
    #[serde(rename = "epicItem", default, skip_serializing_if = "Option::is_none")]
    pub epic_item: Option<Bool>,
    #[serde(
        rename = "equipTradeBlock",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub equip_trade_block: Option<Bool>,
    #[serde(
        rename = "expireOnLogout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_on_logout: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hide: Option<Bool>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hot: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hum: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Canvas>,
    #[serde(rename = "iconRaw", default, skip_serializing_if = "Option::is_none")]
    pub icon_raw: Option<Canvas>,
    #[serde(
        rename = "ignorePickup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_pickup: Option<StrOrInt>,
    #[serde(rename = "incACC", default, skip_serializing_if = "Option::is_none")]
    pub inc_acc: Option<StrOrInt>,
    #[serde(rename = "incCraft", default, skip_serializing_if = "Option::is_none")]
    pub inc_craft: Option<i64>,
    #[serde(rename = "incDEX", default, skip_serializing_if = "Option::is_none")]
    pub inc_dex: Option<StrOrInt>,
    #[serde(rename = "incEVA", default, skip_serializing_if = "Option::is_none")]
    pub inc_eva: Option<StrOrInt>,
    #[serde(rename = "incHP", default, skip_serializing_if = "Option::is_none")]
    pub inc_hp: Option<i64>,
    #[serde(rename = "incINT", default, skip_serializing_if = "Option::is_none")]
    pub inc_int: Option<StrOrInt>,
    #[serde(rename = "incJump", default, skip_serializing_if = "Option::is_none")]
    pub inc_jump: Option<StrOrInt>,
    #[serde(rename = "incLUk", default, skip_serializing_if = "Option::is_none")]
    pub inc_l_uk: Option<StrOrInt>,
    #[serde(rename = "incLUK", default, skip_serializing_if = "Option::is_none")]
    pub inc_luk: Option<StrOrInt>,
    #[serde(rename = "incMAD", default, skip_serializing_if = "Option::is_none")]
    pub inc_mad: Option<StrOrInt>,
    #[serde(rename = "incMDD", default, skip_serializing_if = "Option::is_none")]
    pub inc_mdd: Option<i64>,
    #[serde(rename = "incMHPr", default, skip_serializing_if = "Option::is_none")]
    pub inc_mh_pr: Option<StrOrInt>,
    #[serde(rename = "incMHP", default, skip_serializing_if = "Option::is_none")]
    pub inc_mhp: Option<StrOrInt>,
    #[serde(rename = "incMMPr", default, skip_serializing_if = "Option::is_none")]
    pub inc_mm_pr: Option<StrOrInt>,
    #[serde(rename = "incMMP", default, skip_serializing_if = "Option::is_none")]
    pub inc_mmp: Option<StrOrInt>,
    #[serde(rename = "incPAD", default, skip_serializing_if = "Option::is_none")]
    pub inc_pad: Option<StrOrInt>,
    #[serde(rename = "incPDD", default, skip_serializing_if = "Option::is_none")]
    pub inc_pdd: Option<StrOrInt>,
    #[serde(rename = "incRMAF", default, skip_serializing_if = "Option::is_none")]
    pub inc_rmaf: Option<i64>,
    #[serde(rename = "incRMAI", default, skip_serializing_if = "Option::is_none")]
    pub inc_rmai: Option<i64>,
    #[serde(rename = "incRMAL", default, skip_serializing_if = "Option::is_none")]
    pub inc_rmal: Option<i64>,
    #[serde(rename = "incRMAS", default, skip_serializing_if = "Option::is_none")]
    pub inc_rmas: Option<i64>,
    #[serde(rename = "incSpeed", default, skip_serializing_if = "Option::is_none")]
    pub inc_speed: Option<StrOrInt>,
    #[serde(rename = "incSTR", default, skip_serializing_if = "Option::is_none")]
    pub inc_str: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub islot: Option<String>,
    #[serde(rename = "IUCMax", default, skip_serializing_if = "Option::is_none")]
    pub iuc_max: Option<i64>,
    #[serde(
        rename = "keywordEffect",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub keyword_effect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knockback: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<CharItemInfoLevel>,
    #[serde(rename = "longRange", default, skip_serializing_if = "Option::is_none")]
    pub long_range: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub love: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "MaxHP", default, skip_serializing_if = "Option::is_none")]
    pub max_hp: Option<i64>,
    #[serde(rename = "medalTag", default, skip_serializing_if = "Option::is_none")]
    pub medal_tag: Option<i64>,
    #[serde(rename = "nameTag", default, skip_serializing_if = "Option::is_none")]
    pub name_tag: Option<Bool>,
    #[serde(rename = "noExpend", default, skip_serializing_if = "Option::is_none")]
    pub no_expend: Option<Bool>,
    #[serde(rename = "notExtend", default, skip_serializing_if = "Option::is_none")]
    pub not_extend: Option<Bool>,
    #[serde(rename = "notSale", default, skip_serializing_if = "Option::is_none")]
    pub not_sale: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only: Option<Bool>,
    #[serde(rename = "onlyEquip", default, skip_serializing_if = "Option::is_none")]
    pub only_equip: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pachinko: Option<Bool>,
    #[serde(
        rename = "pickupItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pickup_item: Option<StrOrInt>,
    #[serde(
        rename = "pickupMeso",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pickup_meso: Option<StrOrInt>,
    #[serde(
        rename = "pickupOthers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pickup_others: Option<StrOrInt>,
    #[serde(
        rename = "PotionDiscount",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub potion_discount: std::collections::HashMap<String, CharItemInfoPotionDiscountValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quest: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace: Option<CharItemInfoReplace>,
    #[serde(rename = "reqDEX", default, skip_serializing_if = "Option::is_none")]
    pub req_dex: Option<StrOrInt>,
    #[serde(rename = "reqINT", default, skip_serializing_if = "Option::is_none")]
    pub req_int: Option<StrOrInt>,
    #[serde(rename = "reqJob", default, skip_serializing_if = "Option::is_none")]
    pub req_job: Option<i64>,
    #[serde(rename = "reqLevel", default, skip_serializing_if = "Option::is_none")]
    pub req_level: Option<i64>,
    #[serde(rename = "reqLUK", default, skip_serializing_if = "Option::is_none")]
    pub req_luk: Option<StrOrInt>,
    #[serde(rename = "reqPOP", default, skip_serializing_if = "Option::is_none")]
    pub req_pop: Option<StrOrInt>,
    #[serde(rename = "reqSTR", default, skip_serializing_if = "Option::is_none")]
    pub req_str: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample: Option<Canvas>,
    #[serde(
        rename = "scanTradeBlock",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scan_trade_block: Option<Bool>,
    #[serde(rename = "setItemID", default, skip_serializing_if = "Option::is_none")]
    pub set_item_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sfx: Option<String>,
    #[serde(
        rename = "sharableOnce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sharable_once: Option<Bool>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub shine: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "slotMax", default, skip_serializing_if = "Option::is_none")]
    pub slot_max: Option<i64>,
    #[serde(rename = "specialID", default, skip_serializing_if = "Option::is_none")]
    pub special_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stand: Option<Bool>,
    #[serde(
        rename = "sweepForDrop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sweep_for_drop: Option<StrOrInt>,
    #[serde(
        rename = "timeLimited",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_limited: Option<Bool>,
    #[serde(rename = "tradBlock", default, skip_serializing_if = "Option::is_none")]
    pub trad_block: Option<Bool>,
    #[serde(
        rename = "tradeAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_available: Option<Bool>,
    #[serde(
        rename = "tradeBlock",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_block: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<CharItemInfoTransform>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tuc: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vslot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub walk: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly: Option<Bool>,
}
impl From<&CharItemInfo> for CharItemInfo {
    fn from(value: &CharItemInfo) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAddition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boss: Option<CharItemInfoAdditionBoss>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<CharItemInfoAdditionCritical>,
    #[serde(rename = "elemBoost", default, skip_serializing_if = "Option::is_none")]
    pub elem_boost: Option<CharItemInfoAdditionElemBoost>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elemboost: Option<CharItemInfoAdditionElemboost>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hpmpchange: Option<CharItemInfoAdditionHpmpchange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobcategory: Option<CharItemInfoAdditionMobcategory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobdie: Option<CharItemInfoAdditionMobdie>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill: Option<CharItemInfoAdditionSkill>,
}
impl From<&CharItemInfoAddition> for CharItemInfoAddition {
    fn from(value: &CharItemInfoAddition) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionBoss {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub con: Option<CharItemInfoAdditionBossCon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prob: Option<i64>,
}
impl From<&CharItemInfoAdditionBoss> for CharItemInfoAdditionBoss {
    fn from(value: &CharItemInfoAdditionBoss) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionBossCon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub craft: Option<i64>,
}
impl From<&CharItemInfoAdditionBossCon> for CharItemInfoAdditionBossCon {
    fn from(value: &CharItemInfoAdditionBossCon) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionCritical {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub con: Option<CharItemInfoAdditionCriticalCon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prob: Option<i64>,
}
impl From<&CharItemInfoAdditionCritical> for CharItemInfoAdditionCritical {
    fn from(value: &CharItemInfoAdditionCritical) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionCriticalCon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lv: Option<i64>,
}
impl From<&CharItemInfoAdditionCriticalCon> for CharItemInfoAdditionCriticalCon {
    fn from(value: &CharItemInfoAdditionCriticalCon) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionElemBoost {
    #[serde(rename = "elemVol", default, skip_serializing_if = "Option::is_none")]
    pub elem_vol: Option<String>,
}
impl From<&CharItemInfoAdditionElemBoost> for CharItemInfoAdditionElemBoost {
    fn from(value: &CharItemInfoAdditionElemBoost) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionElemboost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub con: Option<CharItemInfoAdditionElemboostCon>,
    #[serde(rename = "elemVol", default, skip_serializing_if = "Option::is_none")]
    pub elem_vol: Option<String>,
}
impl From<&CharItemInfoAdditionElemboost> for CharItemInfoAdditionElemboost {
    fn from(value: &CharItemInfoAdditionElemboost) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionElemboostCon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub craft: Option<i64>,
}
impl From<&CharItemInfoAdditionElemboostCon> for CharItemInfoAdditionElemboostCon {
    fn from(value: &CharItemInfoAdditionElemboostCon) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionHpmpchange {
    #[serde(
        rename = "hpChangePerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hp_change_per_time: Option<i64>,
    #[serde(
        rename = "hpChangerPerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hp_changer_per_time: Option<i64>,
    #[serde(
        rename = "mpChangerPerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mp_changer_per_time: Option<i64>,
}
impl From<&CharItemInfoAdditionHpmpchange> for CharItemInfoAdditionHpmpchange {
    fn from(value: &CharItemInfoAdditionHpmpchange) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionMobcategory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage: Option<i64>,
}
impl From<&CharItemInfoAdditionMobcategory> for CharItemInfoAdditionMobcategory {
    fn from(value: &CharItemInfoAdditionMobcategory) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionMobdie {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub con: Option<CharItemInfoAdditionMobdieCon>,
    #[serde(
        rename = "hpIncOnMobDie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hp_inc_on_mob_die: Option<i64>,
    #[serde(
        rename = "hpIncRatioOnMobDie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hp_inc_ratio_on_mob_die: Option<i64>,
    #[serde(
        rename = "hpRatioProp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hp_ratio_prop: Option<i64>,
    #[serde(
        rename = "mpIncOnMobDie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mp_inc_on_mob_die: Option<i64>,
    #[serde(
        rename = "mpIncRatioOnMobDie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mp_inc_ratio_on_mob_die: Option<i64>,
    #[serde(
        rename = "mpRatioProp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mp_ratio_prop: Option<i64>,
}
impl From<&CharItemInfoAdditionMobdie> for CharItemInfoAdditionMobdie {
    fn from(value: &CharItemInfoAdditionMobdie) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionMobdieCon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub craft: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lv: Option<i64>,
}
impl From<&CharItemInfoAdditionMobdieCon> for CharItemInfoAdditionMobdieCon {
    fn from(value: &CharItemInfoAdditionMobdieCon) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionSkill {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub con: Option<CharItemInfoAdditionSkillCon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
}
impl From<&CharItemInfoAdditionSkill> for CharItemInfoAdditionSkill {
    fn from(value: &CharItemInfoAdditionSkill) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoAdditionSkillCon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub craft: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lv: Option<i64>,
}
impl From<&CharItemInfoAdditionSkillCon> for CharItemInfoAdditionSkillCon {
    fn from(value: &CharItemInfoAdditionSkillCon) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoBonusExpValue {
    #[serde(rename = "incExpR", default, skip_serializing_if = "Option::is_none")]
    pub inc_exp_r: Option<i64>,
    #[serde(rename = "termStart", default, skip_serializing_if = "Option::is_none")]
    pub term_start: Option<i64>,
}
impl From<&CharItemInfoBonusExpValue> for CharItemInfoBonusExpValue {
    fn from(value: &CharItemInfoBonusExpValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoEffect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animate: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bottom: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dx: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dy: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emission: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow: Option<Bool>,
    #[serde(rename = "genOnMove", default, skip_serializing_if = "Option::is_none")]
    pub gen_on_move: Option<Bool>,
    #[serde(
        rename = "genPoint",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub gen_point: std::collections::HashMap<String, Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pos: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theta: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<StrOrInt>,
}
impl From<&CharItemInfoEffect> for CharItemInfoEffect {
    fn from(value: &CharItemInfoEffect) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoEpic {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hs: Option<String>,
    #[serde(
        rename = "ItemSkill",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub item_skill: std::collections::HashMap<String, CharItemInfoEpicItemSkillValue>,
    #[serde(
        rename = "Skill",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub skill: std::collections::HashMap<String, CharItemInfoEpicSkillValue>,
}
impl From<&CharItemInfoEpic> for CharItemInfoEpic {
    fn from(value: &CharItemInfoEpic) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoEpicItemSkillValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
}
impl From<&CharItemInfoEpicItemSkillValue> for CharItemInfoEpicItemSkillValue {
    fn from(value: &CharItemInfoEpicItemSkillValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoEpicSkillValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
}
impl From<&CharItemInfoEpicSkillValue> for CharItemInfoEpicSkillValue {
    fn from(value: &CharItemInfoEpicSkillValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoLevel {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub case: std::collections::HashMap<String, CharItemInfoLevelCaseValue>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub info: std::collections::HashMap<String, CharItemInfoLevelInfoValue>,
}
impl From<&CharItemInfoLevel> for CharItemInfoLevel {
    fn from(value: &CharItemInfoLevel) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharItemInfoLevelCaseValue {
    #[serde(
        rename = "4",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _4: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prob: Option<i64>,
}
impl From<&CharItemInfoLevelCaseValue> for CharItemInfoLevelCaseValue {
    fn from(value: &CharItemInfoLevelCaseValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoLevelInfoValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[serde(rename = "incACCMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_acc_max: Option<i64>,
    #[serde(rename = "incACCMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_acc_min: Option<i64>,
    #[serde(rename = "incDEXMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_dex_max: Option<i64>,
    #[serde(rename = "incDEXMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_dex_min: Option<i64>,
    #[serde(rename = "incEVAMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_eva_max: Option<i64>,
    #[serde(rename = "incEVAMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_eva_min: Option<i64>,
    #[serde(rename = "incINTMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_int_max: Option<i64>,
    #[serde(rename = "incINTMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_int_min: Option<i64>,
    #[serde(
        rename = "incJumpMax",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inc_jump_max: Option<i64>,
    #[serde(
        rename = "incJumpMin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inc_jump_min: Option<i64>,
    #[serde(rename = "incLUKMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_luk_max: Option<i64>,
    #[serde(rename = "incLUKMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_luk_min: Option<i64>,
    #[serde(rename = "incMADMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_mad_max: Option<i64>,
    #[serde(rename = "incMADMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_mad_min: Option<i64>,
    #[serde(rename = "incMDDMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_mdd_max: Option<i64>,
    #[serde(rename = "incMDDMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_mdd_min: Option<i64>,
    #[serde(rename = "incMHPMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_mhp_max: Option<i64>,
    #[serde(rename = "incMHPMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_mhp_min: Option<i64>,
    #[serde(rename = "incMMPMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_mmp_max: Option<i64>,
    #[serde(rename = "incMMPMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_mmp_min: Option<i64>,
    #[serde(rename = "incPADMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_pad_max: Option<i64>,
    #[serde(rename = "incPADMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_pad_min: Option<i64>,
    #[serde(rename = "incPDDMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_pdd_max: Option<i64>,
    #[serde(rename = "incPDDMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_pdd_min: Option<i64>,
    #[serde(
        rename = "incSpeedMax",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inc_speed_max: Option<i64>,
    #[serde(
        rename = "incSpeedMin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inc_speed_min: Option<i64>,
    #[serde(rename = "incSTRMax", default, skip_serializing_if = "Option::is_none")]
    pub inc_str_max: Option<i64>,
    #[serde(rename = "incSTRMin", default, skip_serializing_if = "Option::is_none")]
    pub inc_str_min: Option<i64>,
}
impl From<&CharItemInfoLevelInfoValue> for CharItemInfoLevelInfoValue {
    fn from(value: &CharItemInfoLevelInfoValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoPotionDiscountValue {
    #[serde(
        rename = "potionDiscR",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub potion_disc_r: Option<i64>,
    #[serde(rename = "termStart", default, skip_serializing_if = "Option::is_none")]
    pub term_start: Option<i64>,
}
impl From<&CharItemInfoPotionDiscountValue> for CharItemInfoPotionDiscountValue {
    fn from(value: &CharItemInfoPotionDiscountValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoReplace {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub itemid: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}
impl From<&CharItemInfoReplace> for CharItemInfoReplace {
    fn from(value: &CharItemInfoReplace) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharItemInfoTransform {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, CharItemInfoTransformExtraValue>,
}
impl From<&CharItemInfoTransform> for CharItemInfoTransform {
    fn from(value: &CharItemInfoTransform) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharItemInfoTransformExtraValue {
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i64>,
    #[serde(rename = "reqDEX", default, skip_serializing_if = "Option::is_none")]
    pub req_dex: Option<i64>,
    #[serde(rename = "reqINT", default, skip_serializing_if = "Option::is_none")]
    pub req_int: Option<i64>,
    #[serde(rename = "reqLUK", default, skip_serializing_if = "Option::is_none")]
    pub req_luk: Option<i64>,
    #[serde(rename = "reqSTR", default, skip_serializing_if = "Option::is_none")]
    pub req_str: Option<i64>,
}
impl From<&CharItemInfoTransformExtraValue> for CharItemInfoTransformExtraValue {
    fn from(value: &CharItemInfoTransformExtraValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fh {
    #[serde(rename = "cantGo", default, skip_serializing_if = "Option::is_none")]
    pub cant_go: Option<Bool>,
    #[serde(
        rename = "cantThrough",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cant_through: Option<Bool>,
    #[serde(
        rename = "forbidFallDown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub forbid_fall_down: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<i64>,
    pub next: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub piece: Option<i64>,
    pub prev: i64,
    pub x1: i64,
    pub x2: i64,
    pub y1: i64,
    pub y2: i64,
}
impl From<&Fh> for Fh {
    fn from(value: &Fh) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Field {
    #[serde(
        rename = "0",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _0: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _1: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "2",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "3",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _3: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "4",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _4: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "5",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _5: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "6",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _6: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "7",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _7: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "8",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _8: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "9",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _9: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub area: std::collections::HashMap<String, FieldAreaValue>,
    #[serde(
        rename = "areaCtrl",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub area_ctrl: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub back: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "battleField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub battle_field: Option<FieldBattleField>,
    #[serde(
        rename = "BuffZone",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub buff_zone: std::collections::HashMap<String, FieldBuffZoneValue>,
    #[serde(
        rename = "climbArea",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub climb_area: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock: Option<FieldClock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coconut: Option<FieldCoconut>,
    #[serde(
        rename = "extinctMO",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub extinct_mo: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "flyingAreaData",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub flying_area_data: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub foothold: std::collections::HashMap<
        String,
        std::collections::HashMap<String, std::collections::HashMap<String, Fh>>,
    >,
    #[serde(
        rename = "footprintData",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub footprint_data: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healer: Option<FieldHealer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<FieldInfo>,
    #[serde(
        rename = "ladderRope",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub ladder_rope: std::collections::HashMap<String, FieldLadderRopeValue>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub life: std::collections::HashMap<String, FieldLifeValue>,
    #[serde(
        rename = "miniMap",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub mini_map: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "MirrorFieldData",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub mirror_field_data: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "mobMassacre",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mob_massacre: Option<FieldMobMassacre>,
    #[serde(
        rename = "mobTeleport",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub mob_teleport: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "monsterCarnival",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub monster_carnival: Option<FieldMonsterCarnival>,
    #[serde(rename = "noSkill", default, skip_serializing_if = "Option::is_none")]
    pub no_skill: Option<FieldNoSkill>,
    #[serde(
        rename = "nodeInfo",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub node_info: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub particle: std::collections::HashMap<
        String,
        std::collections::HashMap<String, FieldParticleValueValue>,
    >,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub portal: std::collections::HashMap<String, FieldPortalValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulley: Option<FieldPulley>,
    #[serde(
        rename = "rapidStream",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub rapid_stream: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub reactor: std::collections::HashMap<String, FieldReactorValue>,
    #[serde(
        rename = "rectInfo",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub rect_info: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "remoteCharacterEffect",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub remote_character_effect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub seat: std::collections::HashMap<String, FieldSeatValue>,
    #[serde(rename = "shipObj", default, skip_serializing_if = "Option::is_none")]
    pub ship_obj: Option<FieldShipObj>,
    #[serde(
        rename = "skyWhale",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub sky_whale: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "snowBall", default, skip_serializing_if = "Option::is_none")]
    pub snow_ball: Option<FieldSnowBall>,
    #[serde(rename = "snowMan", default, skip_serializing_if = "Option::is_none")]
    pub snow_man: Option<FieldSnowMan>,
    #[serde(
        rename = "swimArea",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub swim_area: std::collections::HashMap<String, FieldSwimAreaValue>,
    #[serde(
        rename = "ToolTip",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub tool_tip: std::collections::HashMap<String, FieldToolTipValue>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub user: std::collections::HashMap<String, FieldUserValue>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub weather: std::collections::HashMap<String, FieldWeatherValue>,
}
impl From<&Field> for Field {
    fn from(value: &Field) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldAreaValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x2: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y2: Option<i64>,
}
impl From<&FieldAreaValue> for FieldAreaValue {
    fn from(value: &FieldAreaValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldBattleField {
    #[serde(
        rename = "effectLose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effect_lose: Option<String>,
    #[serde(rename = "effectWin", default, skip_serializing_if = "Option::is_none")]
    pub effect_win: Option<String>,
    #[serde(
        rename = "rewardMapLoseSheep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_lose_sheep: Option<i64>,
    #[serde(
        rename = "rewardMapLoseWolf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_lose_wolf: Option<i64>,
    #[serde(
        rename = "rewardMapWinSheep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_win_sheep: Option<i64>,
    #[serde(
        rename = "rewardMapWinWolf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_win_wolf: Option<i64>,
    #[serde(
        rename = "timeDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_default: Option<i64>,
    #[serde(
        rename = "timeFinish",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_finish: Option<i64>,
}
impl From<&FieldBattleField> for FieldBattleField {
    fn from(value: &FieldBattleField) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldBuffZoneValue {
    #[serde(rename = "Duration", default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "Interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(rename = "ItemID", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x2: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y2: Option<i64>,
}
impl From<&FieldBuffZoneValue> for FieldBuffZoneValue {
    fn from(value: &FieldBuffZoneValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldClock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldClock> for FieldClock {
    fn from(value: &FieldClock) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldCoconut {
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub avatar: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "countBombing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub count_bombing: Option<i64>,
    #[serde(
        rename = "countFalling",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub count_falling: Option<i64>,
    #[serde(rename = "countHit", default, skip_serializing_if = "Option::is_none")]
    pub count_hit: Option<i64>,
    #[serde(
        rename = "countStopped",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub count_stopped: Option<i64>,
    #[serde(
        rename = "effectLose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effect_lose: Option<String>,
    #[serde(rename = "effectWin", default, skip_serializing_if = "Option::is_none")]
    pub effect_win: Option<String>,
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(
        rename = "eventObjectName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_object_name: Option<String>,
    #[serde(rename = "soundLose", default, skip_serializing_if = "Option::is_none")]
    pub sound_lose: Option<String>,
    #[serde(rename = "soundWin", default, skip_serializing_if = "Option::is_none")]
    pub sound_win: Option<String>,
    #[serde(
        rename = "timeDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_default: Option<i64>,
    #[serde(
        rename = "timeExpand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_expand: Option<i64>,
    #[serde(
        rename = "timeFinish",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_finish: Option<i64>,
    #[serde(
        rename = "timeMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_message: Option<i64>,
}
impl From<&FieldCoconut> for FieldCoconut {
    fn from(value: &FieldCoconut) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldHealer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fall: Option<i64>,
    #[serde(rename = "healMax", default, skip_serializing_if = "Option::is_none")]
    pub heal_max: Option<i64>,
    #[serde(rename = "healMin", default, skip_serializing_if = "Option::is_none")]
    pub heal_min: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rise: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(rename = "yMax", default, skip_serializing_if = "Option::is_none")]
    pub y_max: Option<i64>,
    #[serde(rename = "yMin", default, skip_serializing_if = "Option::is_none")]
    pub y_min: Option<i64>,
}
impl From<&FieldHealer> for FieldHealer {
    fn from(value: &FieldHealer) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldInfo {
    #[serde(
        rename = "allMoveCheck",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_move_check: Option<Bool>,
    #[serde(
        rename = "allowedItem",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub allowed_item: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "AmbientBGMv",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ambient_bg_mv: Option<i64>,
    #[serde(
        rename = "AmbientBGM",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ambient_bgm: Option<String>,
    #[serde(
        rename = "autoLieDetector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_lie_detector: Option<FieldInfoAutoLieDetector>,
    #[serde(
        rename = "barrierArc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub barrier_arc: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bgm: Option<String>,
    #[serde(
        rename = "bgmSub",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub bgm_sub: std::collections::HashMap<String, String>,
    #[serde(
        rename = "blockPBossChange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_p_boss_change: Option<Bool>,
    #[serde(
        rename = "bonusStageNoChangeBack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bonus_stage_no_change_back: Option<Bool>,
    #[serde(
        rename = "canPartyStatChangeIgnoreParty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_party_stat_change_ignore_party: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<Bool>,
    #[serde(
        rename = "consumeItemCoolTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consume_item_cool_time: Option<i64>,
    #[serde(
        rename = "createMobInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_mob_interval: Option<i64>,
    #[serde(
        rename = "damageCheckFree",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub damage_check_free: Option<Bool>,
    #[serde(rename = "decHP", default, skip_serializing_if = "Option::is_none")]
    pub dec_hp: Option<i64>,
    #[serde(
        rename = "decInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dec_interval: Option<i64>,
    #[serde(rename = "decMP", default, skip_serializing_if = "Option::is_none")]
    pub dec_mp: Option<i64>,
    #[serde(rename = "decRate", default, skip_serializing_if = "Option::is_none")]
    pub dec_rate: Option<i64>,
    #[serde(
        rename = "dropExpire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drop_expire: Option<i64>,
    #[serde(rename = "dropRate", default, skip_serializing_if = "Option::is_none")]
    pub drop_rate: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(
        rename = "entrustedShop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub entrusted_shop: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escort: Option<FieldInfoEscort>,
    #[serde(
        rename = "EscortMinTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub escort_min_time: Option<i64>,
    #[serde(
        rename = "eventChairIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_chair_index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub everlast: Option<Bool>,
    #[serde(
        rename = "expeditionOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expedition_only: Option<Bool>,
    #[serde(
        rename = "fieldLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_limit: Option<i64>,
    #[serde(
        rename = "fieldLimit_tw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_limit_tw: Option<i64>,
    #[serde(
        rename = "fieldScript",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_script: Option<String>,
    #[serde(
        rename = "fieldSubType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_sub_type: Option<i64>,
    #[serde(rename = "fieldType", default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<StrOrNum>,
    #[serde(
        rename = "fixedMobCapacity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_mob_capacity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fly: Option<Bool>,
    #[serde(
        rename = "footStepSound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub foot_step_sound: Option<String>,
    #[serde(
        rename = "forceReturnOnDead",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_return_on_dead: Option<Bool>,
    #[serde(
        rename = "forceSpeed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_speed: Option<i64>,
    #[serde(
        rename = "forcedReturn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub forced_return: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    #[serde(
        rename = "hideMinimap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_minimap: Option<Bool>,
    #[serde(
        rename = "HobbangKing",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub hobbang_king: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "individualMobPool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub individual_mob_pool: Option<Bool>,
    #[serde(
        rename = "largeSplit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub large_split: Option<i64>,
    #[serde(rename = "LBBottom", default, skip_serializing_if = "Option::is_none")]
    pub lb_bottom: Option<i64>,
    #[serde(rename = "LBSide", default, skip_serializing_if = "Option::is_none")]
    pub lb_side: Option<i64>,
    #[serde(rename = "LBTop", default, skip_serializing_if = "Option::is_none")]
    pub lb_top: Option<i64>,
    #[serde(
        rename = "limitSpeedAndJump",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_speed_and_jump: Option<Bool>,
    #[serde(
        rename = "limitUseShop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_use_shop: Option<Bool>,
    #[serde(
        rename = "limitUseTrunk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_use_trunk: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(
        rename = "lvForceMove",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lv_force_move: Option<i64>,
    #[serde(rename = "lvLimit", default, skip_serializing_if = "Option::is_none")]
    pub lv_limit: Option<i64>,
    #[serde(rename = "mapDesc", default, skip_serializing_if = "Option::is_none")]
    pub map_desc: Option<String>,
    #[serde(rename = "mapMark", default, skip_serializing_if = "Option::is_none")]
    pub map_mark: Option<String>,
    #[serde(rename = "mapName", default, skip_serializing_if = "Option::is_none")]
    pub map_name: Option<String>,
    #[serde(
        rename = "maxDeathCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_death_count: Option<i64>,
    #[serde(
        rename = "miniMapOnOff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mini_map_on_off: Option<Bool>,
    #[serde(
        rename = "mirror_Bottom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mirror_bottom: Option<Bool>,
    #[serde(rename = "mobRate", default, skip_serializing_if = "Option::is_none")]
    pub mob_rate: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<FieldInfoMode>,
    #[serde(rename = "moveLimit", default, skip_serializing_if = "Option::is_none")]
    pub move_limit: Option<Bool>,
    #[serde(rename = "MR", default, skip_serializing_if = "Option::is_none")]
    pub mr: Option<f64>,
    #[serde(rename = "MRBottom", default, skip_serializing_if = "Option::is_none")]
    pub mr_bottom: Option<i64>,
    #[serde(rename = "MRLeft", default, skip_serializing_if = "Option::is_none")]
    pub mr_left: Option<i64>,
    #[serde(rename = "MRRight", default, skip_serializing_if = "Option::is_none")]
    pub mr_right: Option<i64>,
    #[serde(rename = "MRTop", default, skip_serializing_if = "Option::is_none")]
    pub mr_top: Option<i64>,
    #[serde(
        rename = "needSkillForFly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub need_skill_for_fly: Option<Bool>,
    #[serde(
        rename = "noBackOverlapped",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_back_overlapped: Option<Bool>,
    #[serde(rename = "noChair", default, skip_serializing_if = "Option::is_none")]
    pub no_chair: Option<Bool>,
    #[serde(
        rename = "noHekatonEffect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_hekaton_effect: Option<Bool>,
    #[serde(rename = "noMapCmd", default, skip_serializing_if = "Option::is_none")]
    pub no_map_cmd: Option<Bool>,
    #[serde(
        rename = "noRegenMap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_regen_map: Option<Bool>,
    #[serde(
        rename = "onFirstUserEnter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub on_first_user_enter: Option<String>,
    #[serde(
        rename = "onUserEnter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub on_user_enter: Option<String>,
    #[serde(
        rename = "partyBonusR",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub party_bonus_r: Option<i64>,
    #[serde(rename = "partyOnly", default, skip_serializing_if = "Option::is_none")]
    pub party_only: Option<Bool>,
    #[serde(
        rename = "partyStandAlone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub party_stand_alone: Option<i64>,
    #[serde(
        rename = "personalShop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub personal_shop: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<i64>,
    #[serde(
        rename = "phaseAlpha",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phase_alpha: Option<i64>,
    #[serde(
        rename = "phaseBG",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub phase_bg: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "protectItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protect_item: Option<i64>,
    #[serde(
        rename = "protectSetKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protect_set_key: Option<i64>,
    #[serde(rename = "qrLimit", default, skip_serializing_if = "Option::is_none")]
    pub qr_limit: Option<f64>,
    #[serde(
        rename = "qrLimitState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qr_limit_state: Option<i64>,
    #[serde(
        rename = "qrLimitState2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qr_limit_state2: Option<i64>,
    #[serde(
        rename = "quarterView",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quarter_view: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rain: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratemob: Option<f64>,
    #[serde(
        rename = "reactorShuffle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_shuffle: Option<Bool>,
    #[serde(
        rename = "reactorShuffleName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_shuffle_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<f64>,
    #[serde(
        rename = "remoteEffect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_effect: Option<i64>,
    #[serde(rename = "returnMap", default, skip_serializing_if = "Option::is_none")]
    pub return_map: Option<i64>,
    #[serde(
        rename = "reviveCurField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub revive_cur_field: Option<Bool>,
    #[serde(
        rename = "ReviveCurFieldOfNoTransfer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub revive_cur_field_of_no_transfer: Option<Bool>,
    #[serde(
        rename = "ReviveCurFieldOfNoTransferPoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub revive_cur_field_of_no_transfer_point: Option<Vec2>,
    #[serde(
        rename = "ridingField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub riding_field: Option<i64>,
    #[serde(
        rename = "scrollDisable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_disable: Option<Bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub skill: std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snow: Option<Bool>,
    #[serde(
        rename = "specialSound",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub special_sound:
        std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
    #[serde(
        rename = "standAlone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stand_alone: Option<f64>,
    #[serde(
        rename = "streetName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub street_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swim: Option<Bool>,
    #[serde(rename = "timeLimit", default, skip_serializing_if = "Option::is_none")]
    pub time_limit: Option<i64>,
    #[serde(rename = "timeMob", default, skip_serializing_if = "Option::is_none")]
    pub time_mob: Option<FieldInfoTimeMob>,
    #[serde(rename = "timeOut", default, skip_serializing_if = "Option::is_none")]
    pub time_out: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub town: Option<Bool>,
    #[serde(
        rename = "vanishAndroid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vanish_android: Option<Bool>,
    #[serde(rename = "vanishPet", default, skip_serializing_if = "Option::is_none")]
    pub vanish_pet: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "VRBottom", default, skip_serializing_if = "Option::is_none")]
    pub vr_bottom: Option<i64>,
    #[serde(rename = "VRLeft", default, skip_serializing_if = "Option::is_none")]
    pub vr_left: Option<i64>,
    #[serde(rename = "VRLimit", default, skip_serializing_if = "Option::is_none")]
    pub vr_limit: Option<Bool>,
    #[serde(rename = "VRRight", default, skip_serializing_if = "Option::is_none")]
    pub vr_right: Option<i64>,
    #[serde(rename = "VRTop", default, skip_serializing_if = "Option::is_none")]
    pub vr_top: Option<i64>,
    #[serde(
        rename = "waitReviveTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wait_revive_time: Option<i64>,
    #[serde(
        rename = "zakum2Hack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zakum2_hack: Option<Bool>,
}
impl From<&FieldInfo> for FieldInfo {
    fn from(value: &FieldInfo) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldInfoAutoLieDetector {
    #[serde(rename = "endHour", default, skip_serializing_if = "Option::is_none")]
    pub end_hour: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<i64>,
    #[serde(rename = "startHour", default, skip_serializing_if = "Option::is_none")]
    pub start_hour: Option<i64>,
}
impl From<&FieldInfoAutoLieDetector> for FieldInfoAutoLieDetector {
    fn from(value: &FieldInfoAutoLieDetector) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldInfoEscort {
    #[serde(
        rename = "checkDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_distance: Option<Bool>,
    #[serde(
        rename = "failMessageOnDie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fail_message_on_die: Option<String>,
    #[serde(
        rename = "failMessageOnDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fail_message_on_distance: Option<String>,
    #[serde(rename = "mobID", default, skip_serializing_if = "Option::is_none")]
    pub mob_id: Option<i64>,
    #[serde(
        rename = "timeOutLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_out_limit: Option<i64>,
    #[serde(
        rename = "timeOutWarningTerm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_out_warning_term: Option<i64>,
    #[serde(
        rename = "warningDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warning_distance: Option<i64>,
    #[serde(
        rename = "warningMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warning_message: Option<String>,
    #[serde(
        rename = "weatherItemID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub weather_item_id: Option<i64>,
}
impl From<&FieldInfoEscort> for FieldInfoEscort {
    fn from(value: &FieldInfoEscort) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FieldInfoMode {
    Variant0(String),
    Variant1(i64),
}
impl From<&FieldInfoMode> for FieldInfoMode {
    fn from(value: &FieldInfoMode) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FieldInfoMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for FieldInfoMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FieldInfoMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FieldInfoMode {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for FieldInfoMode {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for FieldInfoMode {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldInfoTimeMob {
    #[serde(rename = "endHour", default, skip_serializing_if = "Option::is_none")]
    pub end_hour: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "startHour", default, skip_serializing_if = "Option::is_none")]
    pub start_hour: Option<i64>,
}
impl From<&FieldInfoTimeMob> for FieldInfoTimeMob {
    fn from(value: &FieldInfoTimeMob) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldLadderRopeValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub piece: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uf: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y2: Option<i64>,
}
impl From<&FieldLadderRopeValue> for FieldLadderRopeValue {
    fn from(value: &FieldLadderRopeValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldLifeValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cy: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fh: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hide: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hold: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limitedname: Option<String>,
    #[serde(rename = "mobTime", default, skip_serializing_if = "Option::is_none")]
    pub mob_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nofoothold: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rx0: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rx1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<i64>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<FieldLifeValueType>,
    #[serde(rename = "useDay", default, skip_serializing_if = "Option::is_none")]
    pub use_day: Option<Bool>,
    #[serde(rename = "useNight", default, skip_serializing_if = "Option::is_none")]
    pub use_night: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldLifeValue> for FieldLifeValue {
    fn from(value: &FieldLifeValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FieldLifeValueType(String);
impl std::ops::Deref for FieldLifeValueType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<FieldLifeValueType> for String {
    fn from(value: FieldLifeValueType) -> Self {
        value.0
    }
}
impl From<&FieldLifeValueType> for FieldLifeValueType {
    fn from(value: &FieldLifeValueType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FieldLifeValueType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("n|m").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"n|m\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for FieldLifeValueType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FieldLifeValueType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FieldLifeValueType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for FieldLifeValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMobMassacre {
    #[serde(
        rename = "countEffect",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub count_effect: std::collections::HashMap<String, FieldMobMassacreCountEffectValue>,
    #[serde(
        rename = "disableSkill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_skill: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gauge: Option<FieldMobMassacreGauge>,
    #[serde(
        rename = "mapDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_distance: Option<i64>,
}
impl From<&FieldMobMassacre> for FieldMobMassacre {
    fn from(value: &FieldMobMassacre) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMobMassacreCountEffectValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buff: Option<i64>,
    #[serde(rename = "skillUse", default, skip_serializing_if = "Option::is_none")]
    pub skill_use: Option<Bool>,
}
impl From<&FieldMobMassacreCountEffectValue> for FieldMobMassacreCountEffectValue {
    fn from(value: &FieldMobMassacreCountEffectValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMobMassacreGauge {
    #[serde(rename = "coolAdd", default, skip_serializing_if = "Option::is_none")]
    pub cool_add: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decrease: Option<i64>,
    #[serde(rename = "hitAdd", default, skip_serializing_if = "Option::is_none")]
    pub hit_add: Option<i64>,
    #[serde(rename = "missSub", default, skip_serializing_if = "Option::is_none")]
    pub miss_sub: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
impl From<&FieldMobMassacreGauge> for FieldMobMassacreGauge {
    fn from(value: &FieldMobMassacreGauge) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnival {
    #[serde(rename = "deathCP", default, skip_serializing_if = "Option::is_none")]
    pub death_cp: Option<i64>,
    #[serde(
        rename = "effectLose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effect_lose: Option<String>,
    #[serde(rename = "effectWin", default, skip_serializing_if = "Option::is_none")]
    pub effect_win: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub guardian: std::collections::HashMap<String, f64>,
    #[serde(
        rename = "guardianGenMax",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guardian_gen_max: Option<i64>,
    #[serde(
        rename = "guardianGenPos",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub guardian_gen_pos:
        std::collections::HashMap<String, FieldMonsterCarnivalGuardianGenPosValue>,
    #[serde(
        rename = "mapDivided",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_divided: Option<Bool>,
    #[serde(rename = "mapType", default, skip_serializing_if = "Option::is_none")]
    pub map_type: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub mob: std::collections::HashMap<String, FieldMonsterCarnivalMobValue>,
    #[serde(rename = "mobGenMax", default, skip_serializing_if = "Option::is_none")]
    pub mob_gen_max: Option<i64>,
    #[serde(
        rename = "mobGenPos",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub mob_gen_pos: std::collections::HashMap<String, FieldMonsterCarnivalMobGenPosValue>,
    #[serde(
        rename = "reactorBlue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_blue: Option<i64>,
    #[serde(
        rename = "reactorRed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_red: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reward: Option<FieldMonsterCarnivalReward>,
    #[serde(
        rename = "rewardMapLose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_lose: Option<i64>,
    #[serde(
        rename = "rewardMapWin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_win: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub skill: std::collections::HashMap<String, f64>,
    #[serde(rename = "soundLose", default, skip_serializing_if = "Option::is_none")]
    pub sound_lose: Option<String>,
    #[serde(rename = "soundWin", default, skip_serializing_if = "Option::is_none")]
    pub sound_win: Option<String>,
    #[serde(
        rename = "timeDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_default: Option<i64>,
    #[serde(
        rename = "timeExpand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_expand: Option<i64>,
    #[serde(
        rename = "timeFinish",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_finish: Option<i64>,
    #[serde(
        rename = "timeMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_message: Option<i64>,
}
impl From<&FieldMonsterCarnival> for FieldMonsterCarnival {
    fn from(value: &FieldMonsterCarnival) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnivalGuardianGenPosValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldMonsterCarnivalGuardianGenPosValue> for FieldMonsterCarnivalGuardianGenPosValue {
    fn from(value: &FieldMonsterCarnivalGuardianGenPosValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnivalMobGenPosValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cy: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fh: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldMonsterCarnivalMobGenPosValue> for FieldMonsterCarnivalMobGenPosValue {
    fn from(value: &FieldMonsterCarnivalMobGenPosValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnivalMobValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StrOrNum>,
    #[serde(rename = "mobTime", default, skip_serializing_if = "Option::is_none")]
    pub mob_time: Option<i64>,
    #[serde(rename = "spendCP", default, skip_serializing_if = "Option::is_none")]
    pub spend_cp: Option<i64>,
}
impl From<&FieldMonsterCarnivalMobValue> for FieldMonsterCarnivalMobValue {
    fn from(value: &FieldMonsterCarnivalMobValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnivalReward {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub climax: Option<f64>,
    #[serde(
        rename = "cpDiff",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub cp_diff: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "probChange",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub prob_change: std::collections::HashMap<String, FieldMonsterCarnivalRewardProbChangeValue>,
}
impl From<&FieldMonsterCarnivalReward> for FieldMonsterCarnivalReward {
    fn from(value: &FieldMonsterCarnivalReward) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnivalRewardProbChangeValue {
    #[serde(rename = "loseCoin", default, skip_serializing_if = "Option::is_none")]
    pub lose_coin: Option<f64>,
    #[serde(rename = "loseCP", default, skip_serializing_if = "Option::is_none")]
    pub lose_cp: Option<f64>,
    #[serde(rename = "loseNuff", default, skip_serializing_if = "Option::is_none")]
    pub lose_nuff: Option<f64>,
    #[serde(
        rename = "loseRecovery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lose_recovery: Option<f64>,
    #[serde(rename = "wInCoin", default, skip_serializing_if = "Option::is_none")]
    pub w_in_coin: Option<f64>,
    #[serde(rename = "winCP", default, skip_serializing_if = "Option::is_none")]
    pub win_cp: Option<f64>,
    #[serde(rename = "winNuff", default, skip_serializing_if = "Option::is_none")]
    pub win_nuff: Option<f64>,
    #[serde(
        rename = "winRecovery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub win_recovery: Option<f64>,
}
impl From<&FieldMonsterCarnivalRewardProbChangeValue>
    for FieldMonsterCarnivalRewardProbChangeValue
{
    fn from(value: &FieldMonsterCarnivalRewardProbChangeValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldNoSkill {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub class: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub skill: std::collections::HashMap<String, i64>,
}
impl From<&FieldNoSkill> for FieldNoSkill {
    fn from(value: &FieldNoSkill) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FieldParticleValueValue {
    Variant0(Particle),
    Variant1(i64),
}
impl From<&FieldParticleValueValue> for FieldParticleValueValue {
    fn from(value: &FieldParticleValueValue) -> Self {
        value.clone()
    }
}
impl From<Particle> for FieldParticleValueValue {
    fn from(value: Particle) -> Self {
        Self::Variant0(value)
    }
}
impl From<i64> for FieldParticleValueValue {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldPortalValue {
    #[serde(
        rename = "2",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<f64>,
    #[serde(rename = "hRange", default, skip_serializing_if = "Option::is_none")]
    pub h_range: Option<i64>,
    #[serde(
        rename = "hideTooltip",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_tooltip: Option<Bool>,
    #[serde(
        rename = "horizontalImpact",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_impact: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "onlyOnce", default, skip_serializing_if = "Option::is_none")]
    pub only_once: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pt: Option<i64>,
    #[serde(
        rename = "reactorName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(
        rename = "sessionValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_value: Option<String>,
    #[serde(
        rename = "sessionValueKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_value_key: Option<String>,
    #[serde(
        rename = "shownAtMinimap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shown_at_minimap: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teleport: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tn: Option<String>,
    #[serde(rename = "vRange", default, skip_serializing_if = "Option::is_none")]
    pub v_range: Option<i64>,
    #[serde(
        rename = "verticalImpact",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_impact: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldPortalValue> for FieldPortalValue {
    fn from(value: &FieldPortalValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldPulley {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulley: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldPulley> for FieldPulley {
    fn from(value: &FieldPulley) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldReactorValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "reactorTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldReactorValue> for FieldReactorValue {
    fn from(value: &FieldReactorValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSeatValue {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldSeatValue> for FieldSeatValue {
    fn from(value: &FieldSeatValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldShipObj {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    #[serde(rename = "shipKind", default, skip_serializing_if = "Option::is_none")]
    pub ship_kind: Option<i64>,
    #[serde(rename = "shipObj", default, skip_serializing_if = "Option::is_none")]
    pub ship_obj: Option<String>,
    #[serde(rename = "tMove", default, skip_serializing_if = "Option::is_none")]
    pub t_move: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x0: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<i64>,
}
impl From<&FieldShipObj> for FieldShipObj {
    fn from(value: &FieldShipObj) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSnowBall {
    #[serde(
        rename = "0",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _0: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _1: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "damageSnowBall",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub damage_snow_ball: Option<StrOrNum>,
    #[serde(
        rename = "damageSnowMan0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub damage_snow_man0: Option<i64>,
    #[serde(
        rename = "damageSnowMan1",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub damage_snow_man1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dx: Option<i64>,
    #[serde(
        rename = "recoveryAmount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_amount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section2: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section3: Option<i64>,
    #[serde(rename = "snowManHP", default, skip_serializing_if = "Option::is_none")]
    pub snow_man_hp: Option<i64>,
    #[serde(
        rename = "snowManWait",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snow_man_wait: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x0: Option<i64>,
    #[serde(rename = "xMax", default, skip_serializing_if = "Option::is_none")]
    pub x_max: Option<i64>,
    #[serde(rename = "xMin", default, skip_serializing_if = "Option::is_none")]
    pub x_min: Option<i64>,
}
impl From<&FieldSnowBall> for FieldSnowBall {
    fn from(value: &FieldSnowBall) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSnowMan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl From<&FieldSnowMan> for FieldSnowMan {
    fn from(value: &FieldSnowMan) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSwimAreaValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x2: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y2: Option<i64>,
}
impl From<&FieldSwimAreaValue> for FieldSwimAreaValue {
    fn from(value: &FieldSwimAreaValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldToolTipValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x2: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y2: Option<i64>,
}
impl From<&FieldToolTipValue> for FieldToolTipValue {
    fn from(value: &FieldToolTipValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldUserValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cond: Option<FieldUserValueCond>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub look: Option<FieldUserValueLook>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub noitem: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stat: Option<FieldUserValueStat>,
}
impl From<&FieldUserValue> for FieldUserValue {
    fn from(value: &FieldUserValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldUserValueCond {
    #[serde(
        rename = "battleFieldTeam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub battle_field_team: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<StrOrNum>,
    #[serde(rename = "itemCount", default, skip_serializing_if = "Option::is_none")]
    pub item_count: Option<StrOrNum>,
    #[serde(rename = "itemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<StrOrNum>,
    #[serde(
        rename = "jobCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_category: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<StrOrNum>,
}
impl From<&FieldUserValueCond> for FieldUserValueCond {
    fn from(value: &FieldUserValueCond) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldUserValueLook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cape: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clothes: Option<StrOrNum>,
    #[serde(rename = "earAcc", default, skip_serializing_if = "Option::is_none")]
    pub ear_acc: Option<StrOrNum>,
    #[serde(rename = "faceAcc", default, skip_serializing_if = "Option::is_none")]
    pub face_acc: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gloves: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pants: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shield: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shoes: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weapon: Option<StrOrNum>,
}
impl From<&FieldUserValueLook> for FieldUserValueLook {
    fn from(value: &FieldUserValueLook) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldUserValueStat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acc: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dex: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eva: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub int: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub luk: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pad: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speedmax: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub str: Option<StrOrNum>,
}
impl From<&FieldUserValueStat> for FieldUserValueStat {
    fn from(value: &FieldUserValueStat) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldWeatherValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}
impl From<&FieldWeatherValue> for FieldWeatherValue {
    fn from(value: &FieldWeatherValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdCount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}
impl From<&IdCount> for IdCount {
    fn from(value: &IdCount) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdCountProp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(
        rename = "dateExpire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_expire: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<i64>,
    #[serde(rename = "jobEx", default, skip_serializing_if = "Option::is_none")]
    pub job_ex: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<i64>,
    #[serde(
        rename = "resignRemove",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resign_remove: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub var: Option<i64>,
}
impl From<&IdCountProp> for IdCountProp {
    fn from(value: &IdCountProp) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IntStr(String);
impl std::ops::Deref for IntStr {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IntStr> for String {
    fn from(value: IntStr) -> Self {
        value.0
    }
}
impl From<&IntStr> for IntStr {
    fn from(value: &IntStr) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IntStr {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(-)?\\d+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(-)?\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IntStr {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IntStr {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IntStr {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IntStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item(pub std::collections::HashMap<String, ItemValue>);
impl std::ops::Deref for Item {
    type Target = std::collections::HashMap<String, ItemValue>;
    fn deref(&self) -> &std::collections::HashMap<String, ItemValue> {
        &self.0
    }
}
impl From<Item> for std::collections::HashMap<String, ItemValue> {
    fn from(value: Item) -> Self {
        value.0
    }
}
impl From<&Item> for Item {
    fn from(value: &Item) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, ItemValue>> for Item {
    fn from(value: std::collections::HashMap<String, ItemValue>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemOption {
    #[serde(
        rename = "attackType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boss: Option<Bool>,
    #[serde(
        rename = "DAMreflect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub da_mreflect: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face: Option<String>,
    #[serde(rename = "HP", default, skip_serializing_if = "Option::is_none")]
    pub hp: Option<i64>,
    #[serde(
        rename = "ignoreDAMr",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_da_mr: Option<i64>,
    #[serde(rename = "ignoreDAM", default, skip_serializing_if = "Option::is_none")]
    pub ignore_dam: Option<i64>,
    #[serde(
        rename = "ignoreTargetDEF",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_target_def: Option<i64>,
    #[serde(rename = "incACCr", default, skip_serializing_if = "Option::is_none")]
    pub inc_ac_cr: Option<i64>,
    #[serde(rename = "incACC", default, skip_serializing_if = "Option::is_none")]
    pub inc_acc: Option<i64>,
    #[serde(
        rename = "incAllskill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inc_allskill: Option<i64>,
    #[serde(rename = "incCr", default, skip_serializing_if = "Option::is_none")]
    pub inc_cr: Option<i64>,
    #[serde(rename = "incDAMr", default, skip_serializing_if = "Option::is_none")]
    pub inc_da_mr: Option<i64>,
    #[serde(rename = "incDEXr", default, skip_serializing_if = "Option::is_none")]
    pub inc_de_xr: Option<i64>,
    #[serde(rename = "incDEX", default, skip_serializing_if = "Option::is_none")]
    pub inc_dex: Option<i64>,
    #[serde(rename = "incEVAr", default, skip_serializing_if = "Option::is_none")]
    pub inc_ev_ar: Option<i64>,
    #[serde(rename = "incEVA", default, skip_serializing_if = "Option::is_none")]
    pub inc_eva: Option<i64>,
    #[serde(rename = "incINTr", default, skip_serializing_if = "Option::is_none")]
    pub inc_in_tr: Option<i64>,
    #[serde(rename = "incINT", default, skip_serializing_if = "Option::is_none")]
    pub inc_int: Option<i64>,
    #[serde(rename = "incJump", default, skip_serializing_if = "Option::is_none")]
    pub inc_jump: Option<i64>,
    #[serde(rename = "incLUKr", default, skip_serializing_if = "Option::is_none")]
    pub inc_lu_kr: Option<i64>,
    #[serde(rename = "incLUK", default, skip_serializing_if = "Option::is_none")]
    pub inc_luk: Option<i64>,
    #[serde(rename = "incMADr", default, skip_serializing_if = "Option::is_none")]
    pub inc_ma_dr: Option<i64>,
    #[serde(rename = "incMAD", default, skip_serializing_if = "Option::is_none")]
    pub inc_mad: Option<i64>,
    #[serde(rename = "incMDDr", default, skip_serializing_if = "Option::is_none")]
    pub inc_md_dr: Option<i64>,
    #[serde(rename = "incMDD", default, skip_serializing_if = "Option::is_none")]
    pub inc_mdd: Option<i64>,
    #[serde(
        rename = "incMesoProp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inc_meso_prop: Option<i64>,
    #[serde(rename = "incMHPr", default, skip_serializing_if = "Option::is_none")]
    pub inc_mh_pr: Option<i64>,
    #[serde(rename = "incMHP", default, skip_serializing_if = "Option::is_none")]
    pub inc_mhp: Option<i64>,
    #[serde(rename = "incMMPr", default, skip_serializing_if = "Option::is_none")]
    pub inc_mm_pr: Option<i64>,
    #[serde(rename = "incMMP", default, skip_serializing_if = "Option::is_none")]
    pub inc_mmp: Option<i64>,
    #[serde(rename = "incPADr", default, skip_serializing_if = "Option::is_none")]
    pub inc_pa_dr: Option<i64>,
    #[serde(rename = "incPAD", default, skip_serializing_if = "Option::is_none")]
    pub inc_pad: Option<i64>,
    #[serde(rename = "incPDDr", default, skip_serializing_if = "Option::is_none")]
    pub inc_pd_dr: Option<i64>,
    #[serde(rename = "incPDD", default, skip_serializing_if = "Option::is_none")]
    pub inc_pdd: Option<i64>,
    #[serde(
        rename = "incRewardProp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inc_reward_prop: Option<i64>,
    #[serde(rename = "incSpeed", default, skip_serializing_if = "Option::is_none")]
    pub inc_speed: Option<i64>,
    #[serde(rename = "incSTRr", default, skip_serializing_if = "Option::is_none")]
    pub inc_st_rr: Option<i64>,
    #[serde(rename = "incSTR", default, skip_serializing_if = "Option::is_none")]
    pub inc_str: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(rename = "MP", default, skip_serializing_if = "Option::is_none")]
    pub mp: Option<i64>,
    #[serde(rename = "mpRestore", default, skip_serializing_if = "Option::is_none")]
    pub mp_restore: Option<i64>,
    #[serde(
        rename = "mpconReduce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mpcon_reduce: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<StrOrInt>,
    #[serde(
        rename = "RecoveryHP",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_hp: Option<i64>,
    #[serde(
        rename = "RecoveryMP",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_mp: Option<i64>,
    #[serde(
        rename = "RecoveryUP",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_up: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}
impl From<&ItemOption> for ItemOption {
    fn from(value: &ItemOption) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemOptions(pub std::collections::HashMap<String, ItemOptionsValue>);
impl std::ops::Deref for ItemOptions {
    type Target = std::collections::HashMap<String, ItemOptionsValue>;
    fn deref(&self) -> &std::collections::HashMap<String, ItemOptionsValue> {
        &self.0
    }
}
impl From<ItemOptions> for std::collections::HashMap<String, ItemOptionsValue> {
    fn from(value: ItemOptions) -> Self {
        value.0
    }
}
impl From<&ItemOptions> for ItemOptions {
    fn from(value: &ItemOptions) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, ItemOptionsValue>> for ItemOptions {
    fn from(value: std::collections::HashMap<String, ItemOptionsValue>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemOptionsValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<ItemOptionsValueInfo>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub level: std::collections::HashMap<String, ItemOption>,
}
impl From<&ItemOptionsValue> for ItemOptionsValue {
    fn from(value: &ItemOptionsValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemOptionsValueInfo {
    #[serde(
        rename = "optionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_type: Option<i64>,
    #[serde(rename = "reqLevel", default, skip_serializing_if = "Option::is_none")]
    pub req_level: Option<i64>,
}
impl From<&ItemOptionsValueInfo> for ItemOptionsValueInfo {
    fn from(value: &ItemOptionsValueInfo) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemValue {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub book: std::collections::HashMap<
        String,
        std::collections::HashMap<String, ItemValueBookValueValue>,
    >,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub bullet: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash: Option<Bool>,
    #[serde(
        rename = "chatBalloon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chat_balloon: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emotion: Option<ItemValueEmotion>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub employee: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hungry: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ItemValueIcon>,
    #[serde(
        rename = "iconD",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub icon_d: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "iconRaw",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub icon_raw: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "iconRawD",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub icon_raw_d: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "iconReward",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_reward: Option<ItemValueIconReward>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<ItemValueInfo>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub message: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub mob: std::collections::HashMap<String, ItemValueMobValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quest: Option<Bool>,
    #[serde(
        rename = "questAccept",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub quest_accept: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "questComplete",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub quest_complete: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub req: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub reward: std::collections::HashMap<String, ItemValueRewardValue>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub sample: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skin: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ItemValueSpec>,
    #[serde(rename = "specEx", default, skip_serializing_if = "Option::is_none")]
    pub spec_ex: Option<ItemValueSpecEx>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub tile: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ui: serde_json::Map<String, serde_json::Value>,
}
impl From<&ItemValue> for ItemValue {
    fn from(value: &ItemValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemValueBookValueValue {
    Variant0(String),
    Variant1 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        align: Option<StrOrNum>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
}
impl From<&ItemValueBookValueValue> for ItemValueBookValueValue {
    fn from(value: &ItemValueBookValueValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemValueEmotion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angry: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cheers: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chu: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cry: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glitter: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub love: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oops: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vomit: Option<f64>,
}
impl From<&ItemValueEmotion> for ItemValueEmotion {
    fn from(value: &ItemValueEmotion) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemValueIcon {
    Variant0(String),
    Variant1(Canvas),
}
impl From<&ItemValueIcon> for ItemValueIcon {
    fn from(value: &ItemValueIcon) -> Self {
        value.clone()
    }
}
impl From<Canvas> for ItemValueIcon {
    fn from(value: Canvas) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemValueIconReward {
    Variant0(String),
    Variant1(Canvas),
}
impl From<&ItemValueIconReward> for ItemValueIconReward {
    fn from(value: &ItemValueIconReward) -> Self {
        value.clone()
    }
}
impl From<Canvas> for ItemValueIconReward {
    fn from(value: Canvas) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemValueInfo {
    #[serde(
        rename = "accountSharable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_sharable: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Bool>,
    #[serde(rename = "addDay", default, skip_serializing_if = "Option::is_none")]
    pub add_day: Option<i64>,
    #[serde(rename = "addTime", default, skip_serializing_if = "Option::is_none")]
    pub add_time: Option<i64>,
    #[serde(
        rename = "autoSpeaking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_speaking: Option<Bool>,
    #[serde(rename = "bgmPath", default, skip_serializing_if = "Option::is_none")]
    pub bgm_path: Option<String>,
    #[serde(rename = "bigSize", default, skip_serializing_if = "Option::is_none")]
    pub big_size: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bottom: Option<i64>,
    #[serde(
        rename = "bridleMsgType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bridle_msg_type: Option<i64>,
    #[serde(
        rename = "bridleProp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bridle_prop: Option<i64>,
    #[serde(
        rename = "bridlePropChg",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bridle_prop_chg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash: Option<Bool>,
    #[serde(rename = "consumeHP", default, skip_serializing_if = "Option::is_none")]
    pub consume_hp: Option<Bool>,
    #[serde(
        rename = "consumeItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consume_item: Option<ItemValueInfoConsumeItem>,
    #[serde(rename = "consumeMP", default, skip_serializing_if = "Option::is_none")]
    pub consume_mp: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursed: Option<i64>,
    #[serde(
        rename = "cursedRates",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub cursed_rates: std::collections::HashMap<String, i64>,
    #[serde(rename = "delayMsg", default, skip_serializing_if = "Option::is_none")]
    pub delay_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<i64>,
    #[serde(rename = "distanceX", default, skip_serializing_if = "Option::is_none")]
    pub distance_x: Option<i64>,
    #[serde(rename = "distanceY", default, skip_serializing_if = "Option::is_none")]
    pub distance_y: Option<i64>,
    #[serde(rename = "dropSweep", default, skip_serializing_if = "Option::is_none")]
    pub drop_sweep: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emotion: Option<i64>,
    #[serde(
        rename = "enchantCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enchant_category: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[serde(
        rename = "expireOnLogout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_on_logout: Option<Bool>,
    #[serde(rename = "floatType", default, skip_serializing_if = "Option::is_none")]
    pub float_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ItemValueInfoIcon>,
    #[serde(rename = "iconRaw", default, skip_serializing_if = "Option::is_none")]
    pub icon_raw: Option<ItemValueInfoIconRaw>,
    #[serde(
        rename = "iconReward",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_reward: Option<ItemValueInfoIconReward>,
    #[serde(rename = "iconShop", default, skip_serializing_if = "Option::is_none")]
    pub icon_shop: Option<ItemValueInfoIconShop>,
    #[serde(
        rename = "ignorePickup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_pickup: Option<Bool>,
    #[serde(rename = "incACC", default, skip_serializing_if = "Option::is_none")]
    pub inc_acc: Option<i64>,
    #[serde(rename = "incCraft", default, skip_serializing_if = "Option::is_none")]
    pub inc_craft: Option<i64>,
    #[serde(rename = "incDEX", default, skip_serializing_if = "Option::is_none")]
    pub inc_dex: Option<i64>,
    #[serde(rename = "incEVA", default, skip_serializing_if = "Option::is_none")]
    pub inc_eva: Option<i64>,
    #[serde(rename = "incINT", default, skip_serializing_if = "Option::is_none")]
    pub inc_int: Option<i64>,
    #[serde(rename = "incIUC", default, skip_serializing_if = "Option::is_none")]
    pub inc_iuc: Option<i64>,
    #[serde(rename = "incJump", default, skip_serializing_if = "Option::is_none")]
    pub inc_jump: Option<i64>,
    #[serde(rename = "incLEV", default, skip_serializing_if = "Option::is_none")]
    pub inc_lev: Option<i64>,
    #[serde(rename = "incLUK", default, skip_serializing_if = "Option::is_none")]
    pub inc_luk: Option<i64>,
    #[serde(rename = "incMAD", default, skip_serializing_if = "Option::is_none")]
    pub inc_mad: Option<i64>,
    #[serde(rename = "incMaxHP", default, skip_serializing_if = "Option::is_none")]
    pub inc_max_hp: Option<i64>,
    #[serde(rename = "incMaxMP", default, skip_serializing_if = "Option::is_none")]
    pub inc_max_mp: Option<i64>,
    #[serde(rename = "incMDD", default, skip_serializing_if = "Option::is_none")]
    pub inc_mdd: Option<i64>,
    #[serde(rename = "incMHP", default, skip_serializing_if = "Option::is_none")]
    pub inc_mhp: Option<i64>,
    #[serde(rename = "incMMP", default, skip_serializing_if = "Option::is_none")]
    pub inc_mmp: Option<i64>,
    #[serde(rename = "incPAD", default, skip_serializing_if = "Option::is_none")]
    pub inc_pad: Option<i64>,
    #[serde(rename = "incPDD", default, skip_serializing_if = "Option::is_none")]
    pub inc_pdd: Option<StrOrInt>,
    #[serde(rename = "incPERIOD", default, skip_serializing_if = "Option::is_none")]
    pub inc_period: Option<i64>,
    #[serde(
        rename = "incRandVol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inc_rand_vol: Option<Bool>,
    #[serde(
        rename = "incReqLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inc_req_level: Option<i64>,
    #[serde(rename = "incSpeed", default, skip_serializing_if = "Option::is_none")]
    pub inc_speed: Option<i64>,
    #[serde(rename = "incSTR", default, skip_serializing_if = "Option::is_none")]
    pub inc_str: Option<i64>,
    #[serde(
        rename = "isBgmOrEffect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_bgm_or_effect: Option<Bool>,
    #[serde(rename = "itemMsg", default, skip_serializing_if = "Option::is_none")]
    pub item_msg: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub karma: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub life: Option<i64>,
    #[serde(rename = "longRange", default, skip_serializing_if = "Option::is_none")]
    pub long_range: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lv: Option<StrOrInt>,
    #[serde(rename = "lvMax", default, skip_serializing_if = "Option::is_none")]
    pub lv_max: Option<i64>,
    #[serde(rename = "lvMin", default, skip_serializing_if = "Option::is_none")]
    pub lv_min: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maplepoint: Option<i64>,
    #[serde(
        rename = "masterLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub master_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<StrOrInt>,
    #[serde(rename = "maxDays", default, skip_serializing_if = "Option::is_none")]
    pub max_days: Option<i64>,
    #[serde(rename = "maxDiff", default, skip_serializing_if = "Option::is_none")]
    pub max_diff: Option<i64>,
    #[serde(rename = "maxLevel", default, skip_serializing_if = "Option::is_none")]
    pub max_level: Option<i64>,
    #[serde(rename = "mcType", default, skip_serializing_if = "Option::is_none")]
    pub mc_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meso: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesomax: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesomin: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesostdev: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub message: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mob: Option<i64>,
    #[serde(rename = "mobHP", default, skip_serializing_if = "Option::is_none")]
    pub mob_hp: Option<i64>,
    #[serde(rename = "mobPotion", default, skip_serializing_if = "Option::is_none")]
    pub mob_potion: Option<Bool>,
    #[serde(
        rename = "monsterBook",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub monster_book: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "noCancelMouse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_cancel_mouse: Option<Bool>,
    #[serde(rename = "noFlip", default, skip_serializing_if = "Option::is_none")]
    pub no_flip: Option<Bool>,
    #[serde(rename = "nomobMsg", default, skip_serializing_if = "Option::is_none")]
    pub nomob_msg: Option<String>,
    #[serde(rename = "notExtend", default, skip_serializing_if = "Option::is_none")]
    pub not_extend: Option<Bool>,
    #[serde(rename = "notSale", default, skip_serializing_if = "Option::is_none")]
    pub not_sale: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npc: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pachinko: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "pickUpBlock",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pick_up_block: Option<Bool>,
    #[serde(rename = "pickupAll", default, skip_serializing_if = "Option::is_none")]
    pub pickup_all: Option<Bool>,
    #[serde(
        rename = "pickupItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pickup_item: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pquest: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preventslip: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<StrOrInt>,
    #[serde(
        rename = "protectTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protect_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quest: Option<Bool>,
    #[serde(rename = "questId", default, skip_serializing_if = "Option::is_none")]
    pub quest_id: Option<i64>,
    #[serde(
        rename = "randOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rand_option: Option<Bool>,
    #[serde(rename = "randStat", default, skip_serializing_if = "Option::is_none")]
    pub rand_stat: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub randstat: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rb: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recall: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recover: Option<StrOrInt>,
    #[serde(
        rename = "recoveryHP",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_hp: Option<i64>,
    #[serde(
        rename = "recoveryMP",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_mp: Option<i64>,
    #[serde(
        rename = "recoveryRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_rate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeat: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace: Option<ItemValueInfoReplace>,
    #[serde(rename = "reqCUC", default, skip_serializing_if = "Option::is_none")]
    pub req_cuc: Option<i64>,
    #[serde(rename = "reqLevel", default, skip_serializing_if = "Option::is_none")]
    pub req_level: Option<i64>,
    #[serde(
        rename = "reqMap",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub req_map: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "reqQuestOnProgress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub req_quest_on_progress: Option<i64>,
    #[serde(rename = "reqRUC", default, skip_serializing_if = "Option::is_none")]
    pub req_ruc: Option<i64>,
    #[serde(
        rename = "reqSkillLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub req_skill_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub sample: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "scanTradeBlock",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scan_trade_block: Option<Bool>,
    #[serde(
        rename = "showMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_message: Option<Bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub skill: std::collections::HashMap<String, i64>,
    #[serde(rename = "slotIndex", default, skip_serializing_if = "Option::is_none")]
    pub slot_index: Option<i64>,
    #[serde(rename = "slotMat", default, skip_serializing_if = "Option::is_none")]
    pub slot_mat: Option<i64>,
    #[serde(rename = "slotMax", default, skip_serializing_if = "Option::is_none")]
    pub slot_max: Option<StrOrInt>,
    #[serde(
        rename = "soldInform",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sold_inform: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    #[serde(
        rename = "stateChangeItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_change_item: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<StrOrInt>,
    #[serde(
        rename = "successRates",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub success_rates: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub successes: std::collections::HashMap<String, i64>,
    #[serde(rename = "tamingMob", default, skip_serializing_if = "Option::is_none")]
    pub taming_mob: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<ItemValueInfoTime>,
    #[serde(
        rename = "timeLimited",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_limited: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<StrOrInt>,
    #[serde(rename = "tradBlock", default, skip_serializing_if = "Option::is_none")]
    pub trad_block: Option<Bool>,
    #[serde(
        rename = "tradeAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_available: Option<Bool>,
    #[serde(
        rename = "tradeBlock",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_block: Option<Bool>,
    #[serde(
        rename = "tragetBlock",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traget_block: Option<Bool>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
    #[serde(rename = "uiData", default, skip_serializing_if = "Option::is_none")]
    pub ui_data: Option<String>,
    #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
    #[serde(rename = "useDelay", default, skip_serializing_if = "Option::is_none")]
    pub use_delay: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmsupport: Option<Bool>,
}
impl From<&ItemValueInfo> for ItemValueInfo {
    fn from(value: &ItemValueInfo) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemValueInfoConsumeItem {
    #[serde(
        rename = "consumeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consume_count: Option<i64>,
    #[serde(
        rename = "consumeCountMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consume_count_message: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, ItemValueInfoConsumeItemExtraValue>,
}
impl From<&ItemValueInfoConsumeItem> for ItemValueInfoConsumeItem {
    fn from(value: &ItemValueInfoConsumeItem) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum ItemValueInfoConsumeItemExtraValue {
    Variant0 {
        #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
        _0: Option<i64>,
        #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
        _1: Option<i64>,
    },
    Variant1(i64),
}
impl From<&ItemValueInfoConsumeItemExtraValue> for ItemValueInfoConsumeItemExtraValue {
    fn from(value: &ItemValueInfoConsumeItemExtraValue) -> Self {
        value.clone()
    }
}
impl From<i64> for ItemValueInfoConsumeItemExtraValue {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemValueInfoIcon {
    Variant0(String),
    Variant1(Canvas),
}
impl From<&ItemValueInfoIcon> for ItemValueInfoIcon {
    fn from(value: &ItemValueInfoIcon) -> Self {
        value.clone()
    }
}
impl From<Canvas> for ItemValueInfoIcon {
    fn from(value: Canvas) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemValueInfoIconRaw {
    Variant0(String),
    Variant1(Canvas),
}
impl From<&ItemValueInfoIconRaw> for ItemValueInfoIconRaw {
    fn from(value: &ItemValueInfoIconRaw) -> Self {
        value.clone()
    }
}
impl From<Canvas> for ItemValueInfoIconRaw {
    fn from(value: Canvas) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemValueInfoIconReward {
    Variant0(String),
    Variant1(Canvas),
}
impl From<&ItemValueInfoIconReward> for ItemValueInfoIconReward {
    fn from(value: &ItemValueInfoIconReward) -> Self {
        value.clone()
    }
}
impl From<Canvas> for ItemValueInfoIconReward {
    fn from(value: Canvas) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemValueInfoIconShop {
    Variant0(String),
    Variant1(Canvas),
}
impl From<&ItemValueInfoIconShop> for ItemValueInfoIconShop {
    fn from(value: &ItemValueInfoIconShop) -> Self {
        value.clone()
    }
}
impl From<Canvas> for ItemValueInfoIconShop {
    fn from(value: Canvas) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemValueInfoReplace {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub itemid: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
}
impl From<&ItemValueInfoReplace> for ItemValueInfoReplace {
    fn from(value: &ItemValueInfoReplace) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemValueInfoTime {
    Variant0(std::collections::HashMap<String, String>),
    Variant1(i64),
}
impl From<&ItemValueInfoTime> for ItemValueInfoTime {
    fn from(value: &ItemValueInfoTime) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, String>> for ItemValueInfoTime {
    fn from(value: std::collections::HashMap<String, String>) -> Self {
        Self::Variant0(value)
    }
}
impl From<i64> for ItemValueInfoTime {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemValueMobValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prob: Option<i64>,
}
impl From<&ItemValueMobValue> for ItemValueMobValue {
    fn from(value: &ItemValueMobValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemValueRewardValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<StrOrInt>,
    #[serde(rename = "Effect", default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prob: Option<StrOrNum>,
    #[serde(rename = "worldMsg", default, skip_serializing_if = "Option::is_none")]
    pub world_msg: Option<String>,
}
impl From<&ItemValueRewardValue> for ItemValueRewardValue {
    fn from(value: &ItemValueRewardValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemValueSpec {
    #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
    pub _0: Option<i64>,
    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub _1: Option<i64>,
    #[serde(rename = "2", default, skip_serializing_if = "Option::is_none")]
    pub _2: Option<i64>,
    #[serde(rename = "3", default, skip_serializing_if = "Option::is_none")]
    pub _3: Option<i64>,
    #[serde(rename = "4", default, skip_serializing_if = "Option::is_none")]
    pub _4: Option<i64>,
    #[serde(rename = "5", default, skip_serializing_if = "Option::is_none")]
    pub _5: Option<i64>,
    #[serde(rename = "6", default, skip_serializing_if = "Option::is_none")]
    pub _6: Option<i64>,
    #[serde(rename = "7", default, skip_serializing_if = "Option::is_none")]
    pub _7: Option<i64>,
    #[serde(rename = "8", default, skip_serializing_if = "Option::is_none")]
    pub _8: Option<i64>,
    #[serde(rename = "9", default, skip_serializing_if = "Option::is_none")]
    pub _9: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acc: Option<StrOrInt>,
    #[serde(rename = "accRate", default, skip_serializing_if = "Option::is_none")]
    pub acc_rate: Option<i64>,
    #[serde(
        rename = "attackIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_index: Option<i64>,
    #[serde(
        rename = "attackMobID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_mob_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barrier: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub berserk: Option<i64>,
    #[serde(rename = "BFSkill", default, skip_serializing_if = "Option::is_none")]
    pub bf_skill: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booster: Option<i64>,
    #[serde(rename = "buffSkill", default, skip_serializing_if = "Option::is_none")]
    pub buff_skill: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub con: std::collections::HashMap<String, ItemValueSpecConValue>,
    #[serde(
        rename = "consumeOnPickup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consume_on_pickup: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curse: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub darkness: Option<Bool>,
    #[serde(
        rename = "defenseAtt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub defense_att: Option<String>,
    #[serde(
        rename = "defenseState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub defense_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dex: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dojangshield: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eva: Option<i64>,
    #[serde(rename = "evaRate", default, skip_serializing_if = "Option::is_none")]
    pub eva_rate: Option<i64>,
    #[serde(
        rename = "eventPoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_point: Option<i64>,
    #[serde(rename = "eventRate", default, skip_serializing_if = "Option::is_none")]
    pub event_rate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[serde(rename = "expBuff", default, skip_serializing_if = "Option::is_none")]
    pub exp_buff: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expinc: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ghost: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hp: Option<i64>,
    #[serde(rename = "hpR", default, skip_serializing_if = "Option::is_none")]
    pub hp_r: Option<StrOrInt>,
    #[serde(
        rename = "ignoreContinent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_continent: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inc: Option<i64>,
    #[serde(
        rename = "incFatigue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inc_fatigue: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    #[serde(rename = "itemCode", default, skip_serializing_if = "Option::is_none")]
    pub item_code: Option<i64>,
    #[serde(rename = "itemRange", default, skip_serializing_if = "Option::is_none")]
    pub item_range: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub itemupbyitem: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub luk: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad: Option<i64>,
    #[serde(rename = "madRate", default, skip_serializing_if = "Option::is_none")]
    pub mad_rate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdd: Option<i64>,
    #[serde(rename = "mddRate", default, skip_serializing_if = "Option::is_none")]
    pub mdd_rate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesoupbyitem: Option<Bool>,
    #[serde(rename = "mhpR", default, skip_serializing_if = "Option::is_none")]
    pub mhp_r: Option<i64>,
    #[serde(rename = "mhpRRate", default, skip_serializing_if = "Option::is_none")]
    pub mhp_r_rate: Option<i64>,
    #[serde(rename = "mmpR", default, skip_serializing_if = "Option::is_none")]
    pub mmp_r: Option<i64>,
    #[serde(rename = "mmpRRate", default, skip_serializing_if = "Option::is_none")]
    pub mmp_r_rate: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub mob: std::collections::HashMap<String, i64>,
    #[serde(rename = "mobHp", default, skip_serializing_if = "Option::is_none")]
    pub mob_hp: Option<i64>,
    #[serde(rename = "mobID", default, skip_serializing_if = "Option::is_none")]
    pub mob_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub morph: Option<i64>,
    #[serde(
        rename = "morphRandom",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub morph_random: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "moveTo", default, skip_serializing_if = "Option::is_none")]
    pub move_to: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mp: Option<i64>,
    #[serde(rename = "mpR", default, skip_serializing_if = "Option::is_none")]
    pub mp_r: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npc: Option<i64>,
    #[serde(
        rename = "onlyPickup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub only_pickup: Option<Bool>,
    #[serde(
        rename = "otherParty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub other_party: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pad: Option<i64>,
    #[serde(rename = "padRate", default, skip_serializing_if = "Option::is_none")]
    pub pad_rate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdd: Option<i64>,
    #[serde(rename = "pddRate", default, skip_serializing_if = "Option::is_none")]
    pub pdd_rate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poison: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prob: Option<StrOrInt>,
    #[serde(
        rename = "randomMoveInFieldSet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub random_move_in_field_set: Option<Bool>,
    #[serde(
        rename = "repeatEffect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repeat_effect: Option<Bool>,
    #[serde(rename = "respectFS", default, skip_serializing_if = "Option::is_none")]
    pub respect_fs: Option<Bool>,
    #[serde(
        rename = "respectMimmune",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub respect_mimmune: Option<Bool>,
    #[serde(
        rename = "respectPimmune",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub respect_pimmune: Option<Bool>,
    #[serde(
        rename = "returnMapQR",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub return_map_qr: Option<i64>,
    #[serde(
        rename = "runOnPickup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_on_pickup: Option<Bool>,
    #[serde(rename = "screenMsg", default, skip_serializing_if = "Option::is_none")]
    pub screen_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seal: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    #[serde(rename = "speedRate", default, skip_serializing_if = "Option::is_none")]
    pub speed_rate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub str: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thaw: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(rename = "uiNumber", default, skip_serializing_if = "Option::is_none")]
    pub ui_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weakness: Option<Bool>,
}
impl From<&ItemValueSpec> for ItemValueSpec {
    fn from(value: &ItemValueSpec) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemValueSpecConValue {
    #[serde(rename = "eMap", default, skip_serializing_if = "Option::is_none")]
    pub e_map: Option<i64>,
    #[serde(rename = "inParty", default, skip_serializing_if = "Option::is_none")]
    pub in_party: Option<Bool>,
    #[serde(rename = "sMap", default, skip_serializing_if = "Option::is_none")]
    pub s_map: Option<i64>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
}
impl From<&ItemValueSpecConValue> for ItemValueSpecConValue {
    fn from(value: &ItemValueSpecConValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemValueSpecEx {
    #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
    pub _0: Option<ItemValueSpecEx0>,
    #[serde(
        rename = "consumeOnPickup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consume_on_pickup: Option<Bool>,
}
impl From<&ItemValueSpecEx> for ItemValueSpecEx {
    fn from(value: &ItemValueSpecEx) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemValueSpecEx0 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(rename = "mobSkill", default, skip_serializing_if = "Option::is_none")]
    pub mob_skill: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<i64>,
}
impl From<&ItemValueSpecEx0> for ItemValueSpecEx0 {
    fn from(value: &ItemValueSpecEx0) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mob {
    #[serde(
        rename = "AngerGaugeAnimation",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub anger_gauge_animation: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "AngerGaugeEffect",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub anger_gauge_effect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack1: Option<MobAttack>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack2: Option<MobAttack>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack3: Option<MobAttack>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack4: Option<MobAttack>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack5: Option<MobAttack>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack6: Option<MobAttack>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack7: Option<MobAttack>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub chase: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub die: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub die1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub die2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "dieF",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub die_f: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub eye: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "filpL",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub filp_l: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "filpR",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub filp_r: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fly: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<MobInfo>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub jump: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ladder: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub miss: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "move",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub move_: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub regen: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub rope: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub say: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill16: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill3: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill4: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill5: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill6: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill7: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill8: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "skillAfter1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub skill_after1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub sleep: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stop: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub summon: serde_json::Map<String, serde_json::Value>,
}
impl From<&Mob> for Mob {
    fn from(value: &Mob) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MobAttack {
    pub info: MobAttackInfo,
}
impl From<&MobAttack> for MobAttack {
    fn from(value: &MobAttack) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobAttackInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<i64>,
    #[serde(
        rename = "additionalAttacksWhenAttackingIdx",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_attacks_when_attacking_idx: Option<i64>,
    #[serde(
        rename = "angerAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub anger_attack: Option<Bool>,
    #[serde(
        rename = "areaWarning",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub area_warning: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "areaWarning1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub area_warning1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attach: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachfacing: Option<Bool>,
    #[serde(
        rename = "attackAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_after: Option<i64>,
    #[serde(
        rename = "attackCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_count: Option<i64>,
    #[serde(
        rename = "attackRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_ratio: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ball: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "bulletCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bullet_count: Option<i64>,
    #[serde(
        rename = "bulletNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bullet_number: Option<i64>,
    #[serde(
        rename = "bulletPattern",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bullet_pattern: Option<i64>,
    #[serde(
        rename = "bulletSpeed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bullet_speed: Option<i64>,
    #[serde(
        rename = "checkHitPeriod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_hit_period: Option<Bool>,
    #[serde(rename = "conMP", default, skip_serializing_if = "Option::is_none")]
    pub con_mp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooltime: Option<i64>,
    #[serde(
        rename = "deadlyAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deadly_attack: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disease: Option<i64>,
    #[serde(rename = "doFirst", default, skip_serializing_if = "Option::is_none")]
    pub do_first: Option<Bool>,
    #[serde(
        rename = "dummyAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dummy_attack: Option<Bool>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect3: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect4: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect5: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect6: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect7: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect8: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "effectAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effect_after: Option<i64>,
    #[serde(rename = "elemAttr", default, skip_serializing_if = "Option::is_none")]
    pub elem_attr: Option<String>,
    #[serde(rename = "fixAttack", default, skip_serializing_if = "Option::is_none")]
    pub fix_attack: Option<Bool>,
    #[serde(rename = "fixDamR", default, skip_serializing_if = "Option::is_none")]
    pub fix_dam_r: Option<i64>,
    #[serde(
        rename = "fixDamRType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fix_dam_r_type: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub flash: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "ignoreEvasion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_evasion: Option<i64>,
    #[serde(
        rename = "ignoreStance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_stance: Option<i64>,
    #[serde(
        rename = "jumpAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub jump_attack: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knockback: Option<Bool>,
    #[serde(
        rename = "knockbackEx",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub knockback_ex: Option<Bool>,
    #[serde(
        rename = "knockbackExImpactX",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub knockback_ex_impact_x: Option<i64>,
    #[serde(
        rename = "knockbackExImpactY",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub knockback_ex_impact_y: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(rename = "MADamage", default, skip_serializing_if = "Option::is_none")]
    pub ma_damage: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub magic: Option<Bool>,
    #[serde(rename = "mpBurn", default, skip_serializing_if = "Option::is_none")]
    pub mp_burn: Option<i64>,
    #[serde(
        rename = "notMissAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_miss_attack: Option<Bool>,
    #[serde(
        rename = "onlyAfterAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub only_after_attack: Option<Bool>,
    #[serde(rename = "PADamage", default, skip_serializing_if = "Option::is_none")]
    pub pa_damage: Option<i64>,
    #[serde(
        rename = "randDelayAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rand_delay_attack: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<MobAttackInfoRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rush: Option<Bool>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub screen: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub special: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "specialAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_attack: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tremble: Option<Bool>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<StrOrNum>,
}
impl From<&MobAttackInfo> for MobAttackInfo {
    fn from(value: &MobAttackInfo) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobAttackInfoRange {
    #[serde(rename = "areaCount", default, skip_serializing_if = "Option::is_none")]
    pub area_count: Option<i64>,
    #[serde(
        rename = "attackCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<Vec2>,
    #[serde(
        rename = "onlyTargetY",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub only_target_y: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rb: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reverse: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sp: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(
        rename = "variableRect",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub variable_rect: serde_json::Map<String, serde_json::Value>,
}
impl From<&MobAttackInfoRange> for MobAttackInfoRange {
    fn from(value: &MobAttackInfoRange) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobInfo {
    #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
    pub _0: Option<i64>,
    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub _1: Option<i64>,
    #[serde(rename = "2", default, skip_serializing_if = "Option::is_none")]
    pub _2: Option<i64>,
    #[serde(rename = "3", default, skip_serializing_if = "Option::is_none")]
    pub _3: Option<i64>,
    #[serde(rename = "4", default, skip_serializing_if = "Option::is_none")]
    pub _4: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acc: Option<StrOrNum>,
    #[serde(
        rename = "actionParticle",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub action_particle: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "allyMob", default, skip_serializing_if = "Option::is_none")]
    pub ally_mob: Option<Bool>,
    #[serde(
        rename = "alwaysShow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub always_show: Option<Bool>,
    #[serde(
        rename = "AngerGauge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub anger_gauge: Option<i64>,
    #[serde(
        rename = "applyBuffCooltime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_buff_cooltime: Option<Bool>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub attack: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ban: Option<MobInfoBan>,
    #[serde(
        rename = "blockUserMove",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_user_move: Option<Bool>,
    #[serde(
        rename = "bodyAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub body_attack: Option<Bool>,
    #[serde(
        rename = "bodyDisease",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub body_disease: Option<i64>,
    #[serde(
        rename = "bodyDiseaseLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub body_disease_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bodyattack: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boss: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buff: Option<StrOrNum>,
    #[serde(
        rename = "cannotEvade",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cannot_evade: Option<Bool>,
    #[serde(
        rename = "cantPassByTeleport",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cant_pass_by_teleport: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<i64>,
    #[serde(
        rename = "changeableMob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub changeable_mob: Option<Bool>,
    #[serde(
        rename = "changeableMob_Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub changeable_mob_type: Option<String>,
    #[serde(
        rename = "ChargeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub charge_count: Option<i64>,
    #[serde(
        rename = "charismaEXP",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub charisma_exp: Option<i64>,
    #[serde(
        rename = "chaseSpeed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chase_speed: Option<i64>,
    #[serde(
        rename = "coolDamage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cool_damage: Option<i64>,
    #[serde(
        rename = "coolDamageProb",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cool_damage_prob: Option<i64>,
    #[serde(
        rename = "copyCharacter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_character: Option<MobInfoCopyCharacter>,
    #[serde(
        rename = "createInvincible",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_invincible: Option<MobInfoCreateInvincible>,
    #[serde(
        rename = "damageRecordQuest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub damage_record_quest: Option<i64>,
    #[serde(
        rename = "damagedByMob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub damaged_by_mob: Option<Bool>,
    #[serde(
        rename = "damagedBySelectedMob",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub damaged_by_selected_mob: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "damagedBySelectedSkill",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub damaged_by_selected_skill: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub default: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "defaultHP", default, skip_serializing_if = "Option::is_none")]
    pub default_hp: Option<String>,
    #[serde(rename = "defaultMP", default, skip_serializing_if = "Option::is_none")]
    pub default_mp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable: Option<Bool>,
    #[serde(
        rename = "doNotRemove",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_remove: Option<Bool>,
    #[serde(
        rename = "dropItemPeriod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drop_item_period: Option<i64>,
    #[serde(rename = "dualGauge", default, skip_serializing_if = "Option::is_none")]
    pub dual_gauge: Option<Bool>,
    #[serde(rename = "elemAttr", default, skip_serializing_if = "Option::is_none")]
    pub elem_attr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escort: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eva: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<StrOrNum>,
    #[serde(
        rename = "explosiveReward",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub explosive_reward: Option<Bool>,
    #[serde(
        rename = "finalmaxHP",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub finalmax_hp: Option<StrOrNum>,
    #[serde(
        rename = "firstAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub first_attack: Option<Bool>,
    #[serde(
        rename = "firstAttackRange",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub first_attack_range: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firstattack: Option<Bool>,
    #[serde(rename = "fixDamage", default, skip_serializing_if = "Option::is_none")]
    pub fix_damage: Option<StrOrNum>,
    #[serde(
        rename = "fixedBodyAttackDamageR",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_body_attack_damage_r: Option<i64>,
    #[serde(
        rename = "fixedDamage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_damage: Option<StrOrNum>,
    #[serde(
        rename = "fixedMobStat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_mob_stat: Option<Bool>,
    #[serde(rename = "flySpeed", default, skip_serializing_if = "Option::is_none")]
    pub fly_speed: Option<i64>,
    #[serde(
        rename = "flyingMove",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub flying_move: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "forceChaseEscort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_chase_escort: Option<Bool>,
    #[serde(
        rename = "forcedSeperateSoul",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub forced_seperate_soul: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs: Option<f64>,
    #[serde(rename = "getCP", default, skip_serializing_if = "Option::is_none")]
    pub get_cp: Option<i64>,
    #[serde(
        rename = "giveOwnerExp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub give_owner_exp: Option<Bool>,
    #[serde(
        rename = "HPgaugeHide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub h_pgauge_hide: Option<Bool>,
    #[serde(
        rename = "HPgaugeShow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub h_pgauge_show: Option<Bool>,
    #[serde(
        rename = "healOnDestroy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub heal_on_destroy: Option<MobInfoHealOnDestroy>,
    #[serde(rename = "hideHP", default, skip_serializing_if = "Option::is_none")]
    pub hide_hp: Option<Bool>,
    #[serde(rename = "hideLevel", default, skip_serializing_if = "Option::is_none")]
    pub hide_level: Option<Bool>,
    #[serde(rename = "hideName", default, skip_serializing_if = "Option::is_none")]
    pub hide_name: Option<Bool>,
    #[serde(
        rename = "hideUserDamage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_user_damage: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidename: Option<Bool>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "HpLinkMob", default, skip_serializing_if = "Option::is_none")]
    pub hp_link_mob: Option<StrOrNum>,
    #[serde(
        rename = "hpNoticePerNum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hp_notice_per_num: Option<i64>,
    #[serde(
        rename = "hpRecovery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hp_recovery: Option<i64>,
    #[serde(
        rename = "hpTagBgcolor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hp_tag_bgcolor: Option<i64>,
    #[serde(
        rename = "hpTagColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hp_tag_color: Option<i64>,
    #[serde(
        rename = "ignoreDamage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_damage: Option<Bool>,
    #[serde(
        rename = "ignoreFieldOut",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_field_out: Option<Bool>,
    #[serde(
        rename = "ignoreMovable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_movable: Option<Bool>,
    #[serde(
        rename = "ignoreMoveImpact",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_move_impact: Option<Bool>,
    #[serde(
        rename = "ignoreMoveableMsg",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_moveable_msg: Option<String>,
    #[serde(
        rename = "individualReward",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub individual_reward: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invincible: Option<Bool>,
    #[serde(
        rename = "isNotChaseSummoned",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_not_chase_summoned: Option<Bool>,
    #[serde(
        rename = "isRemoteRange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_remote_range: Option<Bool>,
    #[serde(
        rename = "largeDamageRecord",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub large_damage_record: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<StrOrNum>,
    #[serde(
        rename = "loseItem",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub lose_item: std::collections::HashMap<String, MobInfoLoseItemValue>,
    #[serde(rename = "MADamage", default, skip_serializing_if = "Option::is_none")]
    pub ma_damage: Option<i64>,
    #[serde(rename = "maxHP", default, skip_serializing_if = "Option::is_none")]
    pub max_hp: Option<StrOrNum>,
    #[serde(rename = "maxMP", default, skip_serializing_if = "Option::is_none")]
    pub max_mp: Option<StrOrNum>,
    #[serde(rename = "mbookID", default, skip_serializing_if = "Option::is_none")]
    pub mbook_id: Option<i64>,
    #[serde(rename = "MDDamage", default, skip_serializing_if = "Option::is_none")]
    pub md_damage: Option<i64>,
    #[serde(rename = "MDRate", default, skip_serializing_if = "Option::is_none")]
    pub md_rate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimap: Option<Bool>,
    #[serde(
        rename = "mobJobCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mob_job_category: Option<i64>,
    #[serde(rename = "mobType", default, skip_serializing_if = "Option::is_none")]
    pub mob_type: Option<MobInfoMobType>,
    #[serde(
        rename = "mobZone",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub mob_zone: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "mobZoneType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mob_zone_type: Option<i64>,
    #[serde(
        rename = "movePerHit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub move_per_hit: Option<MobInfoMovePerHit>,
    #[serde(
        rename = "mpRecovery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mp_recovery: Option<i64>,
    #[serde(rename = "noDebuff", default, skip_serializing_if = "Option::is_none")]
    pub no_debuff: Option<Bool>,
    #[serde(rename = "noDoom", default, skip_serializing_if = "Option::is_none")]
    pub no_doom: Option<Bool>,
    #[serde(rename = "noFlip", default, skip_serializing_if = "Option::is_none")]
    pub no_flip: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noregen: Option<Bool>,
    #[serde(rename = "notAttack", default, skip_serializing_if = "Option::is_none")]
    pub not_attack: Option<Bool>,
    #[serde(
        rename = "notDamaged",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_damaged: Option<Bool>,
    #[serde(
        rename = "onlyNormalAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub only_normal_attack: Option<Bool>,
    #[serde(
        rename = "onlySelectedSkill",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub only_selected_skill: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "opacityLayer",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub opacity_layer: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "overSpeed", default, skip_serializing_if = "Option::is_none")]
    pub over_speed: Option<Bool>,
    #[serde(rename = "PADamage", default, skip_serializing_if = "Option::is_none")]
    pub pa_damage: Option<i64>,
    #[serde(
        rename = "partyBonusMob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub party_bonus_mob: Option<Bool>,
    #[serde(
        rename = "PartyReward",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub party_reward: Option<String>,
    #[serde(rename = "PDDamage", default, skip_serializing_if = "Option::is_none")]
    pub pd_damage: Option<i64>,
    #[serde(rename = "PDRate", default, skip_serializing_if = "Option::is_none")]
    pub pd_rate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point: Option<i64>,
    #[serde(
        rename = "posFixedMoveMob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pos_fixed_move_mob: Option<Bool>,
    #[serde(
        rename = "publicReward",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_reward: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pushed: Option<i64>,
    #[serde(
        rename = "randomFlyingMob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub random_flying_mob: Option<i64>,
    #[serde(
        rename = "rareItemDropLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rare_item_drop_level: Option<i64>,
    #[serde(
        rename = "removeAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remove_after: Option<StrOrNum>,
    #[serde(
        rename = "removeOnMiss",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remove_on_miss: Option<Bool>,
    #[serde(
        rename = "removeQuest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remove_quest: Option<Bool>,
    #[serde(
        rename = "resistSkill",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub resist_skill: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub revive: std::collections::HashMap<String, StrOrNum>,
    #[serde(
        rename = "selfDestruction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_destruction: Option<MobInfoSelfDestruction>,
    #[serde(
        rename = "showNotRemoteDam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_not_remote_dam: Option<Bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub skill: std::collections::HashMap<String, MobInfoSkillValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speak: Option<MobInfoSpeak>,
    #[serde(
        rename = "specialDieAction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_die_action: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub summon: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "summonEffect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub summon_effect: Option<i64>,
    #[serde(
        rename = "summonType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub summon_type: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub thumbnail: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undead: Option<Bool>,
    #[serde(
        rename = "underObject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub under_object: Option<Bool>,
    #[serde(
        rename = "upperMostLayer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_most_layer: Option<Bool>,
    #[serde(
        rename = "useReaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_reaction: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weapon: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wp: Option<i64>,
}
impl From<&MobInfo> for MobInfo {
    fn from(value: &MobInfo) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobInfoBan {
    #[serde(
        rename = "banMap",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub ban_map: std::collections::HashMap<String, MobInfoBanBanMapValue>,
    #[serde(rename = "banMsg", default, skip_serializing_if = "Option::is_none")]
    pub ban_msg: Option<String>,
    #[serde(
        rename = "banMsgType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ban_msg_type: Option<i64>,
    #[serde(rename = "banType", default, skip_serializing_if = "Option::is_none")]
    pub ban_type: Option<i64>,
}
impl From<&MobInfoBan> for MobInfoBan {
    fn from(value: &MobInfoBan) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MobInfoBanBanMapValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portal: Option<String>,
}
impl From<&MobInfoBanBanMapValue> for MobInfoBanBanMapValue {
    fn from(value: &MobInfoBanBanMapValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobInfoCopyCharacter {
    #[serde(rename = "hpRatio", default, skip_serializing_if = "Option::is_none")]
    pub hp_ratio: Option<i64>,
    #[serde(rename = "mpRatio", default, skip_serializing_if = "Option::is_none")]
    pub mp_ratio: Option<i64>,
}
impl From<&MobInfoCopyCharacter> for MobInfoCopyCharacter {
    fn from(value: &MobInfoCopyCharacter) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MobInfoCreateInvincible {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(rename = "skillID", default, skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<i64>,
}
impl From<&MobInfoCreateInvincible> for MobInfoCreateInvincible {
    fn from(value: &MobInfoCreateInvincible) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobInfoHealOnDestroy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
}
impl From<&MobInfoHealOnDestroy> for MobInfoHealOnDestroy {
    fn from(value: &MobInfoHealOnDestroy) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobInfoLoseItemValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "loseMsg", default, skip_serializing_if = "Option::is_none")]
    pub lose_msg: Option<String>,
    #[serde(
        rename = "loseMsgType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lose_msg_type: Option<i64>,
    #[serde(rename = "notDrop", default, skip_serializing_if = "Option::is_none")]
    pub not_drop: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
}
impl From<&MobInfoLoseItemValue> for MobInfoLoseItemValue {
    fn from(value: &MobInfoLoseItemValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MobInfoMobType {
    Variant0(String),
    Variant1(i64),
}
impl From<&MobInfoMobType> for MobInfoMobType {
    fn from(value: &MobInfoMobType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for MobInfoMobType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for MobInfoMobType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MobInfoMobType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MobInfoMobType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for MobInfoMobType {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for MobInfoMobType {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MobInfoMovePerHit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<i64>,
    #[serde(rename = "speedR", default, skip_serializing_if = "Option::is_none")]
    pub speed_r: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub way: Option<i64>,
}
impl From<&MobInfoMovePerHit> for MobInfoMovePerHit {
    fn from(value: &MobInfoMovePerHit) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobInfoSelfDestruction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<i64>,
    #[serde(
        rename = "attackIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_index: Option<i64>,
    #[serde(
        rename = "attackableNoMob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attackable_no_mob: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hp: Option<i64>,
    #[serde(
        rename = "removeAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remove_after: Option<i64>,
    #[serde(
        rename = "serverType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_type: Option<i64>,
}
impl From<&MobInfoSelfDestruction> for MobInfoSelfDestruction {
    fn from(value: &MobInfoSelfDestruction) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobInfoSkillValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<i64>,
    #[serde(
        rename = "afterAttack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub after_attack: Option<i64>,
    #[serde(
        rename = "afterAttackCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub after_attack_count: Option<i64>,
    #[serde(
        rename = "afterDelay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub after_delay: Option<i64>,
    #[serde(
        rename = "effectAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effect_after: Option<i64>,
    #[serde(
        rename = "ignoreStance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_stance: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knockback: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<StrOrNum>,
    #[serde(rename = "onlyFsm", default, skip_serializing_if = "Option::is_none")]
    pub only_fsm: Option<Bool>,
    #[serde(
        rename = "onlyOtherSkill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub only_other_skill: Option<Bool>,
    #[serde(
        rename = "preSkillCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_skill_count: Option<i64>,
    #[serde(
        rename = "preSkillIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_skill_index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill: Option<StrOrNum>,
    #[serde(
        rename = "skillAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skill_after: Option<i64>,
    #[serde(
        rename = "skillForbid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skill_forbid: Option<i64>,
    #[serde(
        rename = "teleportTarget",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub teleport_target: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "weatherMsg",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub weather_msg: serde_json::Map<String, serde_json::Value>,
}
impl From<&MobInfoSkillValue> for MobInfoSkillValue {
    fn from(value: &MobInfoSkillValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MobInfoSpeak {
    #[serde(
        rename = "chatBalloon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chat_balloon: Option<Bool>,
    #[serde(
        rename = "chataBalloon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chata_balloon: Option<Bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub con: std::collections::HashMap<String, MobInfoSpeakConValue>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, MobInfoSpeakExtraValue>,
}
impl From<&MobInfoSpeak> for MobInfoSpeak {
    fn from(value: &MobInfoSpeak) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobInfoSpeakConValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pet: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quest: Option<MobInfoSpeakConValueQuest>,
}
impl From<&MobInfoSpeakConValue> for MobInfoSpeakConValue {
    fn from(value: &MobInfoSpeakConValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MobInfoSpeakConValueQuest {
    #[serde(rename = "questID", default, skip_serializing_if = "Option::is_none")]
    pub quest_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<i64>,
}
impl From<&MobInfoSpeakConValueQuest> for MobInfoSpeakConValueQuest {
    fn from(value: &MobInfoSpeakConValueQuest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum MobInfoSpeakExtraValue {
    ObjMsg {
        #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
        _0: Option<String>,
        #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
        _1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hp: Option<StrOrNum>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prob: Option<i64>,
    },
    Message(String),
}
impl From<&MobInfoSpeakExtraValue> for MobInfoSpeakExtraValue {
    fn from(value: &MobInfoSpeakExtraValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MobSkill(pub std::collections::HashMap<String, MobSkillValue>);
impl std::ops::Deref for MobSkill {
    type Target = std::collections::HashMap<String, MobSkillValue>;
    fn deref(&self) -> &std::collections::HashMap<String, MobSkillValue> {
        &self.0
    }
}
impl From<MobSkill> for std::collections::HashMap<String, MobSkillValue> {
    fn from(value: MobSkill) -> Self {
        value.0
    }
}
impl From<&MobSkill> for MobSkill {
    fn from(value: &MobSkill) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, MobSkillValue>> for MobSkill {
    fn from(value: std::collections::HashMap<String, MobSkillValue>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobSkillValue {
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub info: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub level: std::collections::HashMap<String, MobSkillValueLevelValue>,
}
impl From<&MobSkillValue> for MobSkillValue {
    fn from(value: &MobSkillValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MobSkillValueLevelValue {
    #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
    pub _0: Option<StrOrNum>,
    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub _1: Option<StrOrNum>,
    #[serde(rename = "10", default, skip_serializing_if = "Option::is_none")]
    pub _10: Option<StrOrNum>,
    #[serde(rename = "11", default, skip_serializing_if = "Option::is_none")]
    pub _11: Option<StrOrNum>,
    #[serde(rename = "2", default, skip_serializing_if = "Option::is_none")]
    pub _2: Option<StrOrNum>,
    #[serde(rename = "3", default, skip_serializing_if = "Option::is_none")]
    pub _3: Option<StrOrNum>,
    #[serde(rename = "4", default, skip_serializing_if = "Option::is_none")]
    pub _4: Option<StrOrNum>,
    #[serde(rename = "5", default, skip_serializing_if = "Option::is_none")]
    pub _5: Option<StrOrNum>,
    #[serde(rename = "6", default, skip_serializing_if = "Option::is_none")]
    pub _6: Option<StrOrNum>,
    #[serde(rename = "7", default, skip_serializing_if = "Option::is_none")]
    pub _7: Option<StrOrNum>,
    #[serde(rename = "8", default, skip_serializing_if = "Option::is_none")]
    pub _8: Option<StrOrNum>,
    #[serde(rename = "9", default, skip_serializing_if = "Option::is_none")]
    pub _9: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub affected: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<MobSkillValueLevelValueEffect>,
    #[serde(rename = "elemAttr", default, skip_serializing_if = "Option::is_none")]
    pub elem_attr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hp: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inteval: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mob: Option<MobSkillValueLevelValueMob>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mob0: Option<MobSkillValueLevelValueMob0>,
    #[serde(rename = "mobCount", default, skip_serializing_if = "Option::is_none")]
    pub mob_count: Option<StrOrNum>,
    #[serde(rename = "mpCon", default, skip_serializing_if = "Option::is_none")]
    pub mp_con: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<StrOrNum>,
    #[serde(
        rename = "randomTarget",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub random_target: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rb: Option<Vec2>,
    #[serde(
        rename = "summonEffect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub summon_effect: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub tile: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<StrOrNum>,
}
impl From<&MobSkillValueLevelValue> for MobSkillValueLevelValue {
    fn from(value: &MobSkillValueLevelValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MobSkillValueLevelValueEffect {
    Animation(serde_json::Map<String, serde_json::Value>),
    Link(String),
}
impl From<&MobSkillValueLevelValueEffect> for MobSkillValueLevelValueEffect {
    fn from(value: &MobSkillValueLevelValueEffect) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for MobSkillValueLevelValueEffect {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self::Animation(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MobSkillValueLevelValueMob {
    Animation(serde_json::Map<String, serde_json::Value>),
    Link(String),
}
impl From<&MobSkillValueLevelValueMob> for MobSkillValueLevelValueMob {
    fn from(value: &MobSkillValueLevelValueMob) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for MobSkillValueLevelValueMob {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self::Animation(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MobSkillValueLevelValueMob0 {
    Animation(serde_json::Map<String, serde_json::Value>),
    Link(String),
}
impl From<&MobSkillValueLevelValueMob0> for MobSkillValueLevelValueMob0 {
    fn from(value: &MobSkillValueLevelValueMob0) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for MobSkillValueLevelValueMob0 {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self::Animation(value)
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NumStr(String);
impl std::ops::Deref for NumStr {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<NumStr> for String {
    fn from(value: NumStr) -> Self {
        value.0
    }
}
impl From<&NumStr> for NumStr {
    fn from(value: &NumStr) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for NumStr {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^\\d+(\\.\\d+)?$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^\\d+(\\.\\d+)?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for NumStr {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NumStr {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NumStr {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for NumStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Particle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rx: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ry: Option<i64>,
    pub x: i64,
    pub y: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<i64>,
}
impl From<&Particle> for Particle {
    fn from(value: &Particle) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PetItem {
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub alert: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub angry: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub angry2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub angry_short: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub arrogance: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub belch: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub bewildered: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub birdeye: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub blush: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub burp: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub change: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub change1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub charge: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub charming: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub chat: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub christmas: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub complain: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub cry: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub dance: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub deride: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub donno: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub dung: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub eat: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub eye: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fart: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "favoriteItem",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub favorite_item: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fight: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fire: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub flash: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fly: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub flyr: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub food: std::collections::HashMap<String, PetItemFoodValue>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub front: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub glitter: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub go: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub good: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub goodboy: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub guitar: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hand: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hands: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hang: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub happy: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hide: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hug: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hungry: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ignore: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub imhungry: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<PetItemInfo>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub interact: std::collections::HashMap<String, PetItemInteractValue>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub jump: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub jumpfly: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub kiss: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub lonely: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub love: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub love2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub melong: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub merong: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub mischief: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub move1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub move2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "move",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub move_: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub nap: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub no: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub nothing: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub piddle: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub play: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub plot: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub poop: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub poor: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub prone: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub proud: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub puling: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub question: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "randAction",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub rand_action: std::collections::HashMap<String, PetItemRandActionValue>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub rest0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub rise: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub sad: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub scratch: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub shock: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub sigh: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub sit: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub slang: std::collections::HashMap<String, PetItemSlangValue>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub sleep: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub smile: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub sneer: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand3: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand4: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand5: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand6: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub start: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stretch: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stunned: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub sulk: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub surprise: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub sweat: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub tedious: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub thanksgiving: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub transform: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub transformation: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub turn: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub twinkling: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "unfavoriteItem",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub unfavorite_item: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub upset: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub vomit: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub wait: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub what: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub yes: serde_json::Map<String, serde_json::Value>,
}
impl From<&PetItem> for PetItem {
    fn from(value: &PetItem) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PetItemFoodValue {
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fail: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l0: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l1: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub success: serde_json::Map<String, serde_json::Value>,
}
impl From<&PetItemFoodValue> for PetItemFoodValue {
    fn from(value: &PetItemFoodValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PetItemInfo {
    #[serde(rename = "autoReact", default, skip_serializing_if = "Option::is_none")]
    pub auto_react: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash: Option<Bool>,
    #[serde(
        rename = "chatBalloon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chat_balloon: Option<i64>,
    #[serde(rename = "consumeHP", default, skip_serializing_if = "Option::is_none")]
    pub consume_hp: Option<Bool>,
    #[serde(rename = "consumeMP", default, skip_serializing_if = "Option::is_none")]
    pub consume_mp: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evol: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evol1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evol2: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evol3: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evol4: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evol5: Option<i64>,
    #[serde(rename = "evolNo", default, skip_serializing_if = "Option::is_none")]
    pub evol_no: Option<i64>,
    #[serde(rename = "evolProb1", default, skip_serializing_if = "Option::is_none")]
    pub evol_prob1: Option<i64>,
    #[serde(rename = "evolProb2", default, skip_serializing_if = "Option::is_none")]
    pub evol_prob2: Option<i64>,
    #[serde(rename = "evolProb3", default, skip_serializing_if = "Option::is_none")]
    pub evol_prob3: Option<i64>,
    #[serde(rename = "evolProb4", default, skip_serializing_if = "Option::is_none")]
    pub evol_prob4: Option<i64>,
    #[serde(rename = "evolProb5", default, skip_serializing_if = "Option::is_none")]
    pub evol_prob5: Option<i64>,
    #[serde(
        rename = "evolReqItemID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evol_req_item_id: Option<i64>,
    #[serde(
        rename = "evolReqPetLvl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evol_req_pet_lvl: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hungry: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Canvas>,
    #[serde(rename = "iconD", default, skip_serializing_if = "Option::is_none")]
    pub icon_d: Option<Canvas>,
    #[serde(rename = "iconRaw", default, skip_serializing_if = "Option::is_none")]
    pub icon_raw: Option<Canvas>,
    #[serde(rename = "iconRawD", default, skip_serializing_if = "Option::is_none")]
    pub icon_raw_d: Option<Canvas>,
    #[serde(
        rename = "interactByUserAction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interact_by_user_action: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub life: Option<i64>,
    #[serde(
        rename = "limitedLife",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limited_life: Option<i64>,
    #[serde(rename = "longRange", default, skip_serializing_if = "Option::is_none")]
    pub long_range: Option<Bool>,
    #[serde(rename = "nameTag", default, skip_serializing_if = "Option::is_none")]
    pub name_tag: Option<i64>,
    #[serde(
        rename = "noMoveToLocker",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_move_to_locker: Option<Bool>,
    #[serde(rename = "noRevive", default, skip_serializing_if = "Option::is_none")]
    pub no_revive: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permanent: Option<Bool>,
    #[serde(rename = "pickupAll", default, skip_serializing_if = "Option::is_none")]
    pub pickup_all: Option<Bool>,
    #[serde(
        rename = "pickupItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pickup_item: Option<Bool>,
    #[serde(
        rename = "sweepForDrop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sweep_for_drop: Option<Bool>,
}
impl From<&PetItemInfo> for PetItemInfo {
    fn from(value: &PetItemInfo) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PetItemInteractValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fail: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inc: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l0: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prob: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub success: serde_json::Map<String, serde_json::Value>,
}
impl From<&PetItemInteractValue> for PetItemInteractValue {
    fn from(value: &PetItemInteractValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PetItemRandActionValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub act: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l0: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l1: Option<i64>,
}
impl From<&PetItemRandActionValue> for PetItemRandActionValue {
    fn from(value: &PetItemRandActionValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PetItemSlangValue {
    #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
    pub _0: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub act: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l0: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l1: Option<i64>,
}
impl From<&PetItemSlangValue> for PetItemSlangValue {
    fn from(value: &PetItemSlangValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuestAct(
    pub std::collections::HashMap<String, std::collections::HashMap<String, QuestActValueValue>>,
);
impl std::ops::Deref for QuestAct {
    type Target =
        std::collections::HashMap<String, std::collections::HashMap<String, QuestActValueValue>>;
    fn deref(
        &self,
    ) -> &std::collections::HashMap<String, std::collections::HashMap<String, QuestActValueValue>>
    {
        &self.0
    }
}
impl From<QuestAct>
    for std::collections::HashMap<String, std::collections::HashMap<String, QuestActValueValue>>
{
    fn from(value: QuestAct) -> Self {
        value.0
    }
}
impl From<&QuestAct> for QuestAct {
    fn from(value: &QuestAct) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, std::collections::HashMap<String, QuestActValueValue>>>
    for QuestAct
{
    fn from(
        value: std::collections::HashMap<
            String,
            std::collections::HashMap<String, QuestActValueValue>,
        >,
    ) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestActValueValue {
    #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
    pub _0: Option<String>,
    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub _1: Option<String>,
    #[serde(rename = "2", default, skip_serializing_if = "Option::is_none")]
    pub _2: Option<String>,
    #[serde(rename = "3", default, skip_serializing_if = "Option::is_none")]
    pub _3: Option<String>,
    #[serde(rename = "4", default, skip_serializing_if = "Option::is_none")]
    pub _4: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ask: Option<Bool>,
    #[serde(
        rename = "buffItemID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub buff_item_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[serde(
        rename = "fieldEnter",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub field_enter: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub item: std::collections::HashMap<String, IdCountProp>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub job: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lvmax: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lvmin: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub map: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub money: Option<i64>,
    #[serde(rename = "nextQuest", default, skip_serializing_if = "Option::is_none")]
    pub next_quest: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub no: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npc: Option<i64>,
    #[serde(rename = "npcAct", default, skip_serializing_if = "Option::is_none")]
    pub npc_act: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub petskill: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub petspeed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pettameness: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pop: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub quest: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub skill: std::collections::HashMap<String, QuestSkill>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub sp: std::collections::HashMap<String, QuestActValueValueSpValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stop: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub yes: serde_json::Map<String, serde_json::Value>,
}
impl From<&QuestActValueValue> for QuestActValueValue {
    fn from(value: &QuestActValueValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestActValueValueSpValue {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub job: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sp_value: Option<i64>,
}
impl From<&QuestActValueValueSpValue> for QuestActValueValueSpValue {
    fn from(value: &QuestActValueValueSpValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuestCheck(
    pub std::collections::HashMap<String, std::collections::HashMap<String, QuestCheckValueValue>>,
);
impl std::ops::Deref for QuestCheck {
    type Target =
        std::collections::HashMap<String, std::collections::HashMap<String, QuestCheckValueValue>>;
    fn deref(
        &self,
    ) -> &std::collections::HashMap<String, std::collections::HashMap<String, QuestCheckValueValue>>
    {
        &self.0
    }
}
impl From<QuestCheck>
    for std::collections::HashMap<String, std::collections::HashMap<String, QuestCheckValueValue>>
{
    fn from(value: QuestCheck) -> Self {
        value.0
    }
}
impl From<&QuestCheck> for QuestCheck {
    fn from(value: &QuestCheck) -> Self {
        value.clone()
    }
}
impl
    From<std::collections::HashMap<String, std::collections::HashMap<String, QuestCheckValueValue>>>
    for QuestCheck
{
    fn from(
        value: std::collections::HashMap<
            String,
            std::collections::HashMap<String, QuestCheckValueValue>,
        >,
    ) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestCheckValueValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buff: Option<String>,
    #[serde(rename = "dayByDay", default, skip_serializing_if = "Option::is_none")]
    pub day_by_day: Option<Bool>,
    #[serde(
        rename = "dayOfWeek",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub day_of_week: std::collections::HashMap<String, Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endmeso: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endscript: Option<String>,
    #[serde(
        rename = "equipAllNeed",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub equip_all_need: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "equipSelectNeed",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub equip_select_need: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceptbuff: Option<String>,
    #[serde(
        rename = "fieldEnter",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub field_enter: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fieldset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fieldsetkeeptime: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub info: std::collections::HashMap<String, String>,
    #[serde(
        rename = "infoNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub info_number: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub infoex: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub item: std::collections::HashMap<String, QuestCheckValueValueItemValue>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub job: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lvmax: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lvmin: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub mbcard: std::collections::HashMap<String, QuestCheckValueValueMbcardValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mbmin: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub mob: std::collections::HashMap<String, IdCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub morph: Option<i64>,
    #[serde(
        rename = "normalAutoStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub normal_auto_start: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npc: Option<i64>,
    #[serde(
        rename = "partyQuest_S",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub party_quest_s: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub pet: std::collections::HashMap<String, QuestCheckValueValuePetValue>,
    #[serde(
        rename = "petAutoSpeakingLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pet_auto_speaking_limit: Option<Bool>,
    #[serde(
        rename = "petRecallLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pet_recall_limit: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pettamenessmin: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pop: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium: Option<Bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub quest: std::collections::HashMap<String, QuestCheckValueValueQuestValue>,
    #[serde(
        rename = "questComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quest_complete: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub skill: std::collections::HashMap<String, QuestSkill>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startscript: Option<String>,
    #[serde(
        rename = "subJobFlags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_job_flags: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tamingmoblevelmin: Option<i64>,
    #[serde(
        rename = "userInteract",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_interact: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worldmax: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worldmin: Option<StrOrInt>,
}
impl From<&QuestCheckValueValue> for QuestCheckValueValue {
    fn from(value: &QuestCheckValueValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestCheckValueValueItemValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}
impl From<&QuestCheckValueValueItemValue> for QuestCheckValueValueItemValue {
    fn from(value: &QuestCheckValueValueItemValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestCheckValueValueMbcardValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
}
impl From<&QuestCheckValueValueMbcardValue> for QuestCheckValueValueMbcardValue {
    fn from(value: &QuestCheckValueValueMbcardValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestCheckValueValuePetValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}
impl From<&QuestCheckValueValuePetValue> for QuestCheckValueValuePetValue {
    fn from(value: &QuestCheckValueValuePetValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestCheckValueValueQuestValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<i64>,
}
impl From<&QuestCheckValueValueQuestValue> for QuestCheckValueValueQuestValue {
    fn from(value: &QuestCheckValueValueQuestValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuestInfo(pub std::collections::HashMap<String, QuestInfoValue>);
impl std::ops::Deref for QuestInfo {
    type Target = std::collections::HashMap<String, QuestInfoValue>;
    fn deref(&self) -> &std::collections::HashMap<String, QuestInfoValue> {
        &self.0
    }
}
impl From<QuestInfo> for std::collections::HashMap<String, QuestInfoValue> {
    fn from(value: QuestInfo) -> Self {
        value.0
    }
}
impl From<&QuestInfo> for QuestInfo {
    fn from(value: &QuestInfo) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, QuestInfoValue>> for QuestInfo {
    fn from(value: std::collections::HashMap<String, QuestInfoValue>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestInfoValue {
    #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
    pub _0: Option<String>,
    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub _1: Option<String>,
    #[serde(rename = "2", default, skip_serializing_if = "Option::is_none")]
    pub _2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<i64>,
    #[serde(
        rename = "autoAccept",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_accept: Option<Bool>,
    #[serde(
        rename = "autoCancel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_cancel: Option<Bool>,
    #[serde(
        rename = "autoComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_complete: Option<Bool>,
    #[serde(
        rename = "autoPreComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_pre_complete: Option<Bool>,
    #[serde(rename = "autoStart", default, skip_serializing_if = "Option::is_none")]
    pub auto_start: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked: Option<Bool>,
    #[serde(
        rename = "dailyPlayTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub daily_play_time: Option<i64>,
    #[serde(
        rename = "demandSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub demand_summary: Option<String>,
    #[serde(
        rename = "medalCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub medal_category: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "oneShot", default, skip_serializing_if = "Option::is_none")]
    pub one_shot: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(
        rename = "resignedTogetherQuest",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub resigned_together_quest: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "rewardSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_summary: Option<String>,
    #[serde(
        rename = "selectedMob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub selected_mob: Option<Bool>,
    #[serde(
        rename = "selectedSkillID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub selected_skill_id: Option<i64>,
    #[serde(
        rename = "showEffect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_effect: Option<String>,
    #[serde(
        rename = "showLayerTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_layer_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sortkey: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "timeLimit", default, skip_serializing_if = "Option::is_none")]
    pub time_limit: Option<i64>,
    #[serde(
        rename = "timeLimit2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_limit2: Option<i64>,
    #[serde(rename = "timerUI", default, skip_serializing_if = "Option::is_none")]
    pub timer_ui: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(
        rename = "viewMedalItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub view_medal_item: Option<i64>,
}
impl From<&QuestInfoValue> for QuestInfoValue {
    fn from(value: &QuestInfoValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuestSay(
    pub std::collections::HashMap<String, std::collections::HashMap<String, QuestSayValueValue>>,
);
impl std::ops::Deref for QuestSay {
    type Target =
        std::collections::HashMap<String, std::collections::HashMap<String, QuestSayValueValue>>;
    fn deref(
        &self,
    ) -> &std::collections::HashMap<String, std::collections::HashMap<String, QuestSayValueValue>>
    {
        &self.0
    }
}
impl From<QuestSay>
    for std::collections::HashMap<String, std::collections::HashMap<String, QuestSayValueValue>>
{
    fn from(value: QuestSay) -> Self {
        value.0
    }
}
impl From<&QuestSay> for QuestSay {
    fn from(value: &QuestSay) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, std::collections::HashMap<String, QuestSayValueValue>>>
    for QuestSay
{
    fn from(
        value: std::collections::HashMap<
            String,
            std::collections::HashMap<String, QuestSayValueValue>,
        >,
    ) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestSayValueValue {
    #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
    pub _0: Option<String>,
    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub _1: Option<String>,
    #[serde(rename = "10", default, skip_serializing_if = "Option::is_none")]
    pub _10: Option<String>,
    #[serde(rename = "11", default, skip_serializing_if = "Option::is_none")]
    pub _11: Option<String>,
    #[serde(rename = "12", default, skip_serializing_if = "Option::is_none")]
    pub _12: Option<String>,
    #[serde(rename = "2", default, skip_serializing_if = "Option::is_none")]
    pub _2: Option<String>,
    #[serde(rename = "3", default, skip_serializing_if = "Option::is_none")]
    pub _3: Option<String>,
    #[serde(rename = "4", default, skip_serializing_if = "Option::is_none")]
    pub _4: Option<String>,
    #[serde(rename = "5", default, skip_serializing_if = "Option::is_none")]
    pub _5: Option<String>,
    #[serde(rename = "6", default, skip_serializing_if = "Option::is_none")]
    pub _6: Option<String>,
    #[serde(rename = "7", default, skip_serializing_if = "Option::is_none")]
    pub _7: Option<String>,
    #[serde(rename = "8", default, skip_serializing_if = "Option::is_none")]
    pub _8: Option<String>,
    #[serde(rename = "9", default, skip_serializing_if = "Option::is_none")]
    pub _9: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ask: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub job: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub lost: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub no: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npc: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub quest: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stop: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub yes: serde_json::Map<String, serde_json::Value>,
}
impl From<&QuestSayValueValue> for QuestSayValueValue {
    fn from(value: &QuestSayValueValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestSkill {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acquire: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub job: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "masterLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub master_level: Option<i64>,
    #[serde(
        rename = "onlyMasterLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub only_master_level: Option<Bool>,
    #[serde(
        rename = "skillLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skill_level: Option<i64>,
}
impl From<&QuestSkill> for QuestSkill {
    fn from(value: &QuestSkill) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Shroom(pub serde_json::Value);
impl std::ops::Deref for Shroom {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<Shroom> for serde_json::Value {
    fn from(value: Shroom) -> Self {
        value.0
    }
}
impl From<&Shroom> for Shroom {
    fn from(value: &Shroom) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for Shroom {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Skill {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<SkillInfo>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub skill: std::collections::HashMap<String, SkillSkillValue>,
}
impl From<&Skill> for Skill {
    fn from(value: &Skill) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillCommonInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acc: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
        #[serde(rename = "asrR", default, skip_serializing_if = "Option::is_none")]
    pub asr_r: Option<SkillExpr>,
    #[serde(
        rename = "attackCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_count: Option<SkillExpr>,
    #[serde(
        rename = "bulletConsume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bullet_consume: Option<StrOrNum>,
    #[serde(
        rename = "bulletCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bullet_count: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooltime: Option<SkillExpr>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cr: Option<SkillExpr>,
    #[serde(
        rename = "criticaldamageMax",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub criticaldamage_max: Option<SkillExpr>,
    #[serde(
        rename = "criticaldamageMin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub criticaldamage_min: Option<SkillExpr>,
    #[serde(rename = "damR", default, skip_serializing_if = "Option::is_none")]
    pub dam_r: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dot: Option<SkillExpr>,
    #[serde(
        rename = "dotInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dot_interval: Option<SkillExpr>,
    #[serde(rename = "dotTime", default, skip_serializing_if = "Option::is_none")]
    pub dot_time: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emdd: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emhp: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emmp: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epad: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epdd: Option<SkillExpr>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
    pub er: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eva: Option<SkillExpr>,
    #[serde(rename = "expR", default, skip_serializing_if = "Option::is_none")]
    pub exp_r: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hp: Option<SkillExpr>,
    #[serde(rename = "hpCon", default, skip_serializing_if = "Option::is_none")]
    pub hp_con: Option<SkillExpr>,
    #[serde(
        rename = "ignoreMobpdpR",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_mobpdp_r: Option<SkillExpr>,
    #[serde(rename = "itemCon", default, skip_serializing_if = "Option::is_none")]
    pub item_con: Option<StrOrNum>,
    #[serde(rename = "itemConNo", default, skip_serializing_if = "Option::is_none")]
    pub item_con_no: Option<StrOrNum>,
    #[serde(
        rename = "itemConsume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub item_consume: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad: Option<SkillExpr>,
    #[serde(rename = "madX", default, skip_serializing_if = "Option::is_none")]
    pub mad_x: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mastery: Option<SkillExpr>,
        #[serde(rename = "maxLevel", default, skip_serializing_if = "Option::is_none")]
    pub max_level: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdd: Option<SkillExpr>,
    #[serde(rename = "mddR", default, skip_serializing_if = "Option::is_none")]
    pub mdd_r: Option<SkillExpr>,
    #[serde(rename = "mesoR", default, skip_serializing_if = "Option::is_none")]
    pub meso_r: Option<SkillExpr>,
    #[serde(rename = "mhpR", default, skip_serializing_if = "Option::is_none")]
    pub mhp_r: Option<SkillExpr>,
    #[serde(rename = "mmpR", default, skip_serializing_if = "Option::is_none")]
    pub mmp_r: Option<SkillExpr>,
    #[serde(rename = "mobCount", default, skip_serializing_if = "Option::is_none")]
    pub mob_count: Option<SkillExpr>,
    #[serde(rename = "moneyCon", default, skip_serializing_if = "Option::is_none")]
    pub money_con: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub morph: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mp: Option<SkillExpr>,
    #[serde(rename = "mpCon", default, skip_serializing_if = "Option::is_none")]
    pub mp_con: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pad: Option<SkillExpr>,
    #[serde(rename = "padX", default, skip_serializing_if = "Option::is_none")]
    pub pad_x: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdd: Option<SkillExpr>,
    #[serde(rename = "pddR", default, skip_serializing_if = "Option::is_none")]
    pub pdd_r: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rb: Option<Vec2>,
    #[serde(
        rename = "selfDestruction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_destruction: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<SkillExpr>,
    #[serde(rename = "subProp", default, skip_serializing_if = "Option::is_none")]
    pub sub_prop: Option<SkillExpr>,
    #[serde(rename = "subTime", default, skip_serializing_if = "Option::is_none")]
    pub sub_time: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<SkillExpr>,
        #[serde(rename = "terR", default, skip_serializing_if = "Option::is_none")]
    pub ter_r: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<SkillExpr>,
}
impl From<&SkillCommonInfo> for SkillCommonInfo {
    fn from(value: &SkillCommonInfo) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SkillExpr {
    Expr(String),
    Int(i64),
}
impl From<&SkillExpr> for SkillExpr {
    fn from(value: &SkillExpr) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SkillExpr {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Expr(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Int(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for SkillExpr {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SkillExpr {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SkillExpr {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for SkillExpr {
    fn to_string(&self) -> String {
        match self {
            Self::Expr(x) => x.to_string(),
            Self::Int(x) => x.to_string(),
        }
    }
}
impl From<i64> for SkillExpr {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Canvas>,
}
impl From<&SkillInfo> for SkillInfo {
    fn from(value: &SkillInfo) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<SkillSkillValueAction>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub affected: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub afterimage: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub back: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub back_effect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub back_effect0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub back_finish: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ball: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ball0: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "cDoor",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub c_door: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "CharLevel",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub char_level: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "combatOrders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub combat_orders: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common: Option<SkillCommonInfo>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub damage: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable: Option<Bool>,
    #[serde(
        rename = "eDoor",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub e_door: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect3: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect_ship: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "elemAttr", default, skip_serializing_if = "Option::is_none")]
    pub elem_attr: Option<String>,
        #[serde(
        rename = "finalAttack",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub final_attack: std::collections::HashMap<String, std::collections::HashMap<String, i64>>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub finish: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub finish0: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "flipBall", default, skip_serializing_if = "Option::is_none")]
    pub flip_ball: Option<serde_json::Value>,
    #[serde(
        rename = "Frame",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub frame: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Canvas>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon1: Option<Canvas>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon2: Option<Canvas>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon3: Option<Canvas>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon4: Option<Canvas>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon5: Option<Canvas>,
    #[serde(
        rename = "iconDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_disabled: Option<Canvas>,
    #[serde(
        rename = "iconMouseOver",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_mouse_over: Option<Canvas>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invisible: Option<Bool>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub keydown: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub keydown0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub keydownend: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub level: std::collections::HashMap<String, SkillSkillValueLevelValue>,
    #[serde(
        rename = "mDoor",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub m_door: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "masterLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub master_level: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub mob: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub mob0: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "mobCode", default, skip_serializing_if = "Option::is_none")]
    pub mob_code: Option<StrOrNum>,
    #[serde(
        rename = "oDoor",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub o_door: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub prepare: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub psd: Option<Bool>,
        #[serde(
        rename = "psdSkill",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub psd_skill: std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub repeat: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub req: std::collections::HashMap<String, StrOrNum>,
    #[serde(
        rename = "sDoor",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub s_door: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub screen: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "skillType", default, skip_serializing_if = "Option::is_none")]
    pub skill_type: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub special: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub special0: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "specialAction",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub special_action: std::collections::HashMap<String, String>,
    #[serde(
        rename = "specialActionFrame",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_action_frame: Option<SkillSkillValueSpecialActionFrame>,
    #[serde(
        rename = "specialAffected",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub special_affected: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub state: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "stopEffect",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub stop_effect: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "subWeapon", default, skip_serializing_if = "Option::is_none")]
    pub sub_weapon: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summon: Option<SkillSkillValueSummon>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub tile: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "timeLimited",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_limited: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weapon: Option<StrOrNum>,
}
impl From<&SkillSkillValue> for SkillSkillValue {
    fn from(value: &SkillSkillValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SkillSkillValueAction {
    Variant0(String),
    Variant1(std::collections::HashMap<String, String>),
}
impl From<&SkillSkillValueAction> for SkillSkillValueAction {
    fn from(value: &SkillSkillValueAction) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, String>> for SkillSkillValueAction {
    fn from(value: std::collections::HashMap<String, String>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValueLevelValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acc: Option<i64>,
    #[serde(
        rename = "attackCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_count: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ball: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooltime: Option<i64>,
    #[serde(
        rename = "criticaldamageMax",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub criticaldamage_max: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damagepc: Option<i64>,
    #[serde(
        rename = "dateExpire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_expire: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dot: Option<StrOrNum>,
    #[serde(
        rename = "dotInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dot_interval: Option<StrOrNum>,
    #[serde(rename = "dotTime", default, skip_serializing_if = "Option::is_none")]
    pub dot_time: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eva: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixdamage: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "hpCon", default, skip_serializing_if = "Option::is_none")]
    pub hp_con: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hs: Option<String>,
    #[serde(rename = "itemCon", default, skip_serializing_if = "Option::is_none")]
    pub item_con: Option<i64>,
    #[serde(rename = "itemConNo", default, skip_serializing_if = "Option::is_none")]
    pub item_con_no: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mastery: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdd: Option<i64>,
    #[serde(rename = "mobCount", default, skip_serializing_if = "Option::is_none")]
    pub mob_count: Option<i64>,
    #[serde(rename = "mpCon", default, skip_serializing_if = "Option::is_none")]
    pub mp_con: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pad: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdd: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rb: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<i64>,
}
impl From<&SkillSkillValueLevelValue> for SkillSkillValueLevelValue {
    fn from(value: &SkillSkillValueLevelValue) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValueSpecialActionFrame {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
}
impl From<&SkillSkillValueSpecialActionFrame> for SkillSkillValueSpecialActionFrame {
    fn from(value: &SkillSkillValueSpecialActionFrame) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValueSummon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack1: Option<SkillSkillValueSummonAttack1>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub attack2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "attackTriangle",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub attack_triangle: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub die: Option<SkillSkillValueSummonDie>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub die1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fly: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub heal: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "move",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub move_: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub prepare: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub repeat: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub repeat0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub say: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill3: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill4: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill5: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill6: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub subsummon: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub summoned: serde_json::Map<String, serde_json::Value>,
}
impl From<&SkillSkillValueSummon> for SkillSkillValueSummon {
    fn from(value: &SkillSkillValueSummon) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillSkillValueSummonAttack1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<SkillSkillValueSummonAttack1Info>,
}
impl From<&SkillSkillValueSummonAttack1> for SkillSkillValueSummonAttack1 {
    fn from(value: &SkillSkillValueSummonAttack1) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValueSummonAttack1Info {
    #[serde(
        rename = "attackAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_after: Option<i64>,
    #[serde(
        rename = "attackCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_count: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ball: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "bulletSpeed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bullet_speed: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "effectAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effect_after: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "mobCount", default, skip_serializing_if = "Option::is_none")]
    pub mob_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<SummonRange>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
}
impl From<&SkillSkillValueSummonAttack1Info> for SkillSkillValueSummonAttack1Info {
    fn from(value: &SkillSkillValueSummonAttack1Info) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillSkillValueSummonDie {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<SkillSkillValueSummonDieInfo>,
}
impl From<&SkillSkillValueSummonDie> for SkillSkillValueSummonDie {
    fn from(value: &SkillSkillValueSummonDie) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValueSummonDieInfo {
    #[serde(
        rename = "attackAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_after: Option<i64>,
    #[serde(rename = "mobCount", default, skip_serializing_if = "Option::is_none")]
    pub mob_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<SummonRange>,
}
impl From<&SkillSkillValueSummonDieInfo> for SkillSkillValueSummonDieInfo {
    fn from(value: &SkillSkillValueSummonDieInfo) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Str(String);
impl std::ops::Deref for Str {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Str> for String {
    fn from(value: Str) -> Self {
        value.0
    }
}
impl From<&Str> for Str {
    fn from(value: &Str) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Str {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^1|0$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^1|0$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Str {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Str {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Str {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Str {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StrOrInt {
    Variant0(IntStr),
    Variant1(i64),
}
impl From<&StrOrInt> for StrOrInt {
    fn from(value: &StrOrInt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for StrOrInt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for StrOrInt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StrOrInt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StrOrInt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for StrOrInt {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<IntStr> for StrOrInt {
    fn from(value: IntStr) -> Self {
        Self::Variant0(value)
    }
}
impl From<i64> for StrOrInt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StrOrNum {
    NumStr(NumStr),
    Int(i64),
}
impl From<&StrOrNum> for StrOrNum {
    fn from(value: &StrOrNum) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for StrOrNum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::NumStr(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Int(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for StrOrNum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StrOrNum {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StrOrNum {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for StrOrNum {
    fn to_string(&self) -> String {
        match self {
            Self::NumStr(x) => x.to_string(),
            Self::Int(x) => x.to_string(),
        }
    }
}
impl From<NumStr> for StrOrNum {
    fn from(value: NumStr) -> Self {
        Self::NumStr(value)
    }
}
impl From<i64> for StrOrNum {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SummonRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rb: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sp: Option<Vec2>,
}
impl From<&SummonRange> for SummonRange {
    fn from(value: &SummonRange) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Vec2 {
        pub x: i64,
        pub y: i64,
}
impl From<&Vec2> for Vec2 {
    fn from(value: &Vec2) -> Self {
        value.clone()
    }
}
