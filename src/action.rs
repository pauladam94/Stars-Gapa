use crate::{faction::Factions, selection::Location};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ActionLoc {
    Hand,
    DiscardOrHand,
    Shop,
    CurrentCard,
}

impl Display for ActionLoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionLoc::Hand => write!(f, "hand"),
            ActionLoc::DiscardOrHand => write!(f, "discard or hand"),
            ActionLoc::Shop => write!(f, "shop"),
            ActionLoc::CurrentCard => write!(f, "current card"),
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Gold(u32),
    Attack(u32),
    Authority(u32),
    Discard(u32),
    Scrap {
        loc: ActionLoc,
        nb: u32,
    },
    Draw(u32),
    OpponentDiscard(u32),
    Complex {
        condition: Condition,
        result: Vec<Action>,
    },
}

/* Useful possible character

shield : 🛡   🪨
*/
pub const GOLD_STR: &'static str = "🪙";
pub const ATTACK_STR: &'static str = "💥";
pub const AUTHORITY_STR: &'static str = "⚕️"; // or maybe 💊🛟
pub const SCRAP_STR: &'static str = "🗑"; // or maybe 🧨💣
pub const DISCARD_STR: &'static str = "♻️"; // or maybe 🧨

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Action::*;
        match self {
            Gold(i) => write!(f, "{}{GOLD_STR}", i),
            Attack(i) => write!(f, "{}{ATTACK_STR}", i),
            Authority(i) => write!(f, "{}{AUTHORITY_STR}", i),
            Discard(i) => write!(f, "{}{DISCARD_STR}", i),
            Scrap { loc, nb } => write!(f, "{} scrap in {}", nb, loc),
            Draw(i) => write!(f, "{}🃏", i),
            OpponentDiscard(i) => write!(f, "opponent discard {}", i),
            Complex { condition, result } => {
                write!(f, "{} > ", condition)?;
                for (i, action) in result.iter().enumerate() {
                    write!(f, "{}", action)?;
                    if i != result.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                writeln!(f)
            }
        }
    }
}

#[derive(Debug)]
pub enum Condition {
    FactionPlayed(Factions),
    Action(Box<Action>),
    GreaterThan(Data, u32),
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Condition::*;
        match self {
            Action(action) => write!(f, "{}", action),
            GreaterThan(data, _) => write!(f, ""),
            FactionPlayed(factions) => write!(f, "{}", factions),
        }
    }
}

#[derive(Debug)]
pub enum Data {}
