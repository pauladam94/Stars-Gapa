use ratatui::{
    crossterm::event::Event,
    style::Stylize,
    widgets::{},
};

#[derive(Debug)]
struct Action {}

#[derive(Debug, PartialEq, Eq)]
enum Faction {
    Blob,
    Traides,
    Empire,
    Scrapper,
}

#[derive(Debug)]
enum OutPost {
    OutPost,
    NotOutpost,
}
#[derive(Debug)]
enum Card {
    Ship {
        name: String,
        action: Vec<Action>,
        scrap: Option<Action>,
        price: u32,
        faction: Faction,
    },
    Base {
        name: String,
        life: u32,
        action: Vec<Action>,
        scrap: Option<Action>,
        price: u32,
        faction: Faction,
        outpost: OutPost,
    },
}

#[derive(Debug)]
struct Deck(Vec<Card>);

#[derive(Debug)]
pub struct Player {
    hand: Deck,
    draw_pile: Deck,
    discard: Deck,
    gold: u32,
    pv: u32,
    attack: u32,
}

impl Player {
    fn attack(&mut self) {}
    fn draw_hand(&mut self) {}
    fn draw_card(&mut self) {}
    fn play_card(&mut self, card: Card) {}
    fn by_card(&mut self) {}
}

mod game {
    
}
