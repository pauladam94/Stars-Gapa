use crate::{action::Action, faction::Faction};

#[derive(Debug)]
enum OutPost {
    OutPost,
    NotOutpost,
}

#[derive(Debug)]
pub struct CardInfo {
    name: String,
    action: Vec<Action>,
    scrap: Option<Action>,
    price: u32,
    faction: Faction,
}
#[derive(Debug)]
pub enum Card {
    Ship(CardInfo),
    OutPost { life: u32, info: CardInfo },
    Base { life: u32, info: CardInfo },
}

#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub const EMPTY: Self = Self(Vec::new());

    pub fn starter() -> Self {
        todo!()
    }
    pub fn starter_shop() -> Self {
        todo!()
    }
}
