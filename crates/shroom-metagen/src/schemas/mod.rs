use shroom_meta::shared::{Circ, Rect, Vec2};

//#[allow(unused)]
//pub mod skill;
#[allow(clippy::all)]
pub mod field_mapper;
#[allow(unused, clippy::all)]
pub mod item_mapper;
#[allow(unused, clippy::all)]
pub mod mob_mapper;
#[allow(clippy::all)]
pub mod shroom_schemas;
#[allow(unused, clippy::all)]
pub mod skill_mapper;

pub mod mob_skill_mapper;
pub mod quest_mapper;

use shroom_schemas as sch;

impl From<shroom_schemas::Bool> for bool {
    fn from(val: shroom_schemas::Bool) -> Self {
        match val {
            shroom_schemas::Bool::Str(v) => v.as_str() == "1",
            shroom_schemas::Bool::Int(v) => v != 0,
        }
    }
}

impl From<&shroom_schemas::Bool> for bool {
    fn from(val: &shroom_schemas::Bool) -> Self {
        match val {
            shroom_schemas::Bool::Str(v) => v.as_str() == "1",
            shroom_schemas::Bool::Int(v) => *v != 0,
        }
    }
}

impl From<shroom_schemas::StrOrNum> for i64 {
    fn from(val: shroom_schemas::StrOrNum) -> Self {
        match val {
            shroom_schemas::StrOrNum::NumStr(s) => s.parse().unwrap(),
            shroom_schemas::StrOrNum::Int(v) => v,
        }
    }
}

impl From<&shroom_schemas::StrOrNum> for i64 {
    fn from(val: &shroom_schemas::StrOrNum) -> Self {
        match val {
            shroom_schemas::StrOrNum::NumStr(s) => s.parse().unwrap(),
            shroom_schemas::StrOrNum::Int(v) => *v,
        }
    }
}

impl From<&shroom_schemas::StrOrNum> for u32 {
    fn from(val: &shroom_schemas::StrOrNum) -> Self {
        match val {
            shroom_schemas::StrOrNum::NumStr(s) => s.parse().unwrap(),
            shroom_schemas::StrOrNum::Int(v) => *v as Self,
        }
    }
}

impl From<shroom_schemas::Vec2> for Vec2 {
    fn from(val: shroom_schemas::Vec2) -> Self {
        Self {
            x: val.x as i32,
            y: val.y as i32,
        }
    }
}

pub struct SchemaRect(pub shroom_schemas::Vec2, pub shroom_schemas::Vec2);

impl From<SchemaRect> for Rect {
    fn from(val: SchemaRect) -> Self {
        Self {
            lt: val.0.into(),
            rb: val.1.into(),
        }
    }
}

pub struct SchemaCirc(pub shroom_schemas::Vec2, pub u32);

impl From<SchemaCirc> for Circ {
    fn from(val: SchemaCirc) -> Self {
        Self {
            sp: val.0.into(),
            radius: val.1,
        }
    }
}

pub trait IntoNum {
    fn into_num(&self) -> i64;
}

impl IntoNum for i64 {
    fn into_num(&self) -> i64 {
        *self
    }
}

impl IntoNum for Option<sch::StrOrNum> {
    fn into_num(&self) -> i64 {
        match self {
            Some(v) => v.into(),
            None => 0,
        }
    }
}

impl IntoNum for Option<sch::StrOrInt> {
    fn into_num(&self) -> i64 {
        match self {
            Some(v) => v.into_num(),
            None => 0,
        }
    }
}

impl IntoNum for sch::StrOrInt {
    fn into_num(&self) -> i64 {
        match self {
            sch::StrOrInt::Variant0(v) => v.parse().unwrap(),
            sch::StrOrInt::Variant1(v) => *v as i64,
        }
    }
}

impl IntoNum for sch::StrOrNum {
    fn into_num(&self) -> i64 {
        self.into()
    }
}

impl IntoNum for Option<i64> {
    fn into_num(&self) -> i64 {
        self.unwrap_or(0)
    }
}

pub trait IntoBool {
    fn into_bool(&self) -> bool;
}

impl IntoBool for Option<sch::Bool> {
    fn into_bool(&self) -> bool {
        match self {
            Some(v) => v.into(),
            None => false,
        }
    }
}
