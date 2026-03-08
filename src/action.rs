use crate::{faction::Factions, selection::Location};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum ActionLoc {
    Hand,
    DiscardOrHand,
    Shop,
    CurrentCard,
}
impl ActionLoc {
    pub fn contains(&self, loc: &Location) -> bool {
        use Location::*;
        match (self, loc) {
            (ActionLoc::Hand, Hand) => true,
            (ActionLoc::DiscardOrHand, Hand) => true,
            (ActionLoc::DiscardOrHand, Discard) => true,
            (ActionLoc::Shop, Shop) => true,
            _ => false,
        }
    }
}

impl Display for ActionLoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionLoc::Hand => write!(f, "hand"),
            ActionLoc::DiscardOrHand => write!(f, "discard | hand"),
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
    // be more precise on the copy
    Copy,
    Complex {
        cond: Condition,
        action: Vec<Action>,
    },
    Or(Box<Action>, Box<Action>),
}

impl Action {
    pub fn or(self, action: Action) -> Self {
        Action::Or(Box::new(self), Box::new(action))
    }
}

/* Useful possible character

shield : 🛡   🪨
*/
pub const BASE_STR: &'static str = "🛡 ";
pub const OUTPOST_STR: &'static str = "🪨";
pub const GOLD_STR: &'static str = "🪙";
pub const ATTACK_STR: &'static str = "💥";
pub const AUTHORITY_STR: &'static str = "⚕️"; // or maybe 💊🛟
pub const SCRAP_STR: &'static str = "🗑"; // or maybe 🧨💣
pub const DISCARD_STR: &'static str = "♻️"; // or maybe 🧨

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Action::*;
        use ActionLoc::*;
        match self {
            Gold(i) => write!(f, "{}{GOLD_STR}", i),
            Attack(i) => write!(f, "{}{ATTACK_STR}", i),
            Authority(i) => write!(f, "{}{AUTHORITY_STR}", i),
            Discard(i) => write!(f, "{}{DISCARD_STR}", i),
            Scrap {
                loc: CurrentCard,
                nb: 1,
            } => write!(f, "{SCRAP_STR}"),
            Scrap { loc, nb: 1 } => write!(f, "{SCRAP_STR} {}", loc),
            Scrap { loc, nb } => write!(f, "{} {SCRAP_STR} {}", nb, loc),
            Draw(i) => write!(f, "{}🃏", i),
            OpponentDiscard(0) => write!(f, "opponent {DISCARD_STR}"),
            OpponentDiscard(i) => write!(f, "opponent {DISCARD_STR}{}", i),
            Complex {
                cond: condition,
                action: result,
            } => {
                write!(f, "{} > ", condition)?;
                for (i, action) in result.iter().enumerate() {
                    write!(f, "{}", action)?;
                    if i != result.len() - 1 {
                        write!(f, " & ")?;
                    }
                }
                writeln!(f)
            }
            Copy => write!(f, "copy ship"),
            Or(action1, action2) => write!(f, "{action1} | {action2}"),
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
pub enum Data {
    NbBases,
}
