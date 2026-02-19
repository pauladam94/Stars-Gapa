use crate::{faction::Faction, selection::DeckKind};

#[derive(Debug)]
pub enum Action {
    Gold(u8),
    Attack(u8),
    Authority(u8),
    Discard,
    Scrap {
        pos: DeckKind,
    },
    Draw,
    OpponentDiscard,
    Complex {
        condition: Condition,
        result: Vec<Action>,
    },
}

#[derive(Debug)]
enum Condition {
    GreaterThan(Data, u32),
}

#[derive(Debug)]
enum Data {}
