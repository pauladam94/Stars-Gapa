use crate::selection::Location;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Action {
    Gold(u32),
    Attack(u32),
    Authority(u32),
    Discard(u32),
    Scrap {
        loc: Location,
        nb: u32,
    },
    Draw(u32),
    OpponentDiscard(u32),
    Complex {
        condition: Condition,
        result: Vec<Action>,
    },
}

pub const GOLD_STR: &'static str = "🪙";
pub const ATTACK_STR: &'static str = "💥";
pub const AUTHORITY_STR: &'static str = "⚕️"; // or maybe 💊🛟
pub const SCRAP_STR: &'static str = "🗑"; // or maybe 🧨💣
pub const DISCARD_STR: &'static str = "♻️"; // or maybe 🧨

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Gold(i) => write!(f, "{}{GOLD_STR}", i),
            Action::Attack(i) => write!(f, "{}{ATTACK_STR}", i),
            Action::Authority(i) => write!(f, "{}{AUTHORITY_STR}", i),
            Action::Discard(i) => write!(f, "{}{DISCARD_STR}", i),
            Action::Scrap { loc, nb } => write!(f, "{} scrap in {}", nb, loc),
            Action::Draw(i) => write!(f, "{}🃏", i),
            Action::OpponentDiscard(i) => write!(f, "opponent discard {}", i),
            Action::Complex { condition, result } => write!(f, "complex todo"),
        }
    }
}

#[derive(Debug)]
pub enum Condition {
    Scrap(Location),
    GreaterThan(Data, u32),
}

#[derive(Debug)]
pub enum Data {}
