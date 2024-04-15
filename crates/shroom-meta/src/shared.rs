use std::{collections::HashMap, ops::Deref, str::FromStr};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::skill::eval::{self, EvalContext};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rect {
    pub lt: Vec2,
    pub rb: Vec2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Circ {
    pub sp: Vec2,
    pub radius: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum ElementAttribute {
    Fire,     // 2 TODO add elem mapping
    Ice,      // 1
    Poison,   //4
    Holy,     // 5
    Light,    // 3
    Physical, // 0
    Dark,     // 6
}

impl TryFrom<&str> for ElementAttribute {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "f" => Self::Fire,
            "i" => Self::Ice,
            "s" => Self::Poison,
            "h" => Self::Holy,
            "l" => Self::Light,
            "p" => Self::Physical,
            "d" => Self::Dark,
            _ => anyhow::bail!("Invalid elem attribute: {}", s),
        })
    }
}

impl TryFrom<char> for ElementAttribute {
    type Error = anyhow::Error;

    fn try_from(c: char) -> anyhow::Result<Self> {
        Ok(match c.to_ascii_lowercase() {
            'f' => Self::Fire,
            'i' => Self::Ice,
            's' => Self::Poison,
            'h' => Self::Holy,
            'l' => Self::Light,
            'p' => Self::Physical,
            'd' => Self::Dark,
            _ => anyhow::bail!("Invalid elem attribute: {}", c),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElemAttrList(pub HashMap<ElementAttribute, u8>);

impl FromStr for ElemAttrList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut map = HashMap::new();
        for (elem, factor) in s.chars().tuples() {
            let elem = ElementAttribute::try_from(elem)?;
            let factor = factor
                .to_digit(10)
                .ok_or_else(|| anyhow::format_err!("Invalid elem factor: {}", factor))?
                as u8;
            map.insert(elem, factor);
        }
        Ok(Self(map))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EvalExpr {
    Num(i32),
    Expr(eval::Expr),
}

impl EvalExpr {
    pub fn eval(&self, x: i32) -> i32 {
        match self {
            Self::Num(n) => *n,
            Self::Expr(expr) => EvalContext::new(x).eval(expr).ceil() as i32,
        }
    }
}

pub fn opt_map2<'a, D, T, U>(opt: &'a Option<D>) -> Result<Option<U>, U::Error>
where
    D: Deref<Target = T>,
    T: 'a + ?Sized,
    U: TryFrom<&'a T>,
{
    opt.as_deref().map(U::try_from).transpose()
}

pub fn opt_map1<'a, T, U>(opt: &'a Option<T>) -> Result<Option<U>, U::Error>
where
    T: 'a,
    U: TryFrom<&'a T>,
{
    opt.as_ref().map(U::try_from).transpose()
}
