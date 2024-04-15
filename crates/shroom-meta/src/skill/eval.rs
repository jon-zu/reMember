use std::str::FromStr;

use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};
use serde::{Deserialize, Serialize};

#[derive(pest_derive::Parser)]
#[grammar = "skill/eval.pest"]
pub struct EvalParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
    };
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    Var(char),
    NVar(char),
    Ceil(Box<Expr>),
    Floor(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

impl<'de> Deserialize<'de> for Expr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let expr = String::deserialize(deserializer)?;
        expr.as_str().parse().map_err(serde::de::Error::custom)
    }
}

impl Serialize for Expr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl FromStr for Expr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse = EvalParser::parse(Rule::expr, s)?;
        Ok(parse_expr(parse))
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Var(c) => write!(f, "{}", c),
            Self::NVar(c) => write!(f, "-{}", c),
            Self::Ceil(expr) => write!(f, "u({})", expr),
            Self::Floor(expr) => write!(f, "d({})", expr),
            Self::BinOp { lhs, op, rhs } => write!(f, "({}{}{})", lhs, op, rhs),
        }
    }
}

impl Expr {}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::p_integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
            Rule::n_integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            Rule::var => Expr::Var('x'),
            Rule::n_var => Expr::NVar('x'),
            Rule::floor => Expr::Floor(Box::new(parse_expr(primary.into_inner()))),
            Rule::ceil => Expr::Ceil(Box::new(parse_expr(primary.into_inner()))),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .parse(pairs)
}

pub struct EvalContext {
    pub x: i32,
}

impl EvalContext {
    pub fn new(x: i32) -> Self {
        Self { x }
    }

    pub fn eval(&self, expr: &Expr) -> f32 {
        match expr {
            Expr::Integer(i) => *i as f32,
            Expr::Var(_c) => self.x as f32,
            Expr::NVar(_c) => -self.x as f32,
            Expr::Ceil(expr) => self.eval(expr.as_ref()).ceil(),
            Expr::Floor(expr) => self.eval(expr.as_ref()).floor(),
            Expr::BinOp { lhs, op, rhs } => {
                let lhs = self.eval(lhs);
                let rhs = self.eval(rhs);

                match op {
                    Op::Add => lhs + rhs,
                    Op::Subtract => lhs - rhs,
                    Op::Multiply => lhs * rhs,
                    Op::Divide => lhs / rhs,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;

    #[test]
    fn parse() {
        let parse = EvalParser::parse(Rule::expr, "6+2*u(x/5)+d(1)-5").unwrap();
        let expr = parse_expr(parse);
        assert_eq!(EvalContext::new(0).eval(&expr), 2.);
        assert_eq!(expr.to_string(), "(((6+(2*u((x/5))))+d(1))-5)");
    }

    #[test]
    fn negative() {
        let parse = EvalParser::parse(Rule::expr, "-2").unwrap();
        let expr = parse_expr(parse);
        assert_eq!(EvalContext::new(0).eval(&expr), -2.);
        assert_eq!(expr.to_string(), "-2");
    }
    #[test]
    fn negative_var() {
        let parse = EvalParser::parse(Rule::expr, "-x").unwrap();
        let expr = parse_expr(parse);
        assert_eq!(EvalContext::new(1).eval(&expr), -1.);
        assert_eq!(expr.to_string(), "-x");
    }
    #[test]
    fn simple_expr() {
        let parse = EvalParser::parse(Rule::expr, "375+5*x").unwrap();
        let expr = parse_expr(parse);
        assert_eq!(EvalContext::new(1).eval(&expr), 380.);
        assert_eq!(expr.to_string(), "(375+(5*x))");
    }
}
