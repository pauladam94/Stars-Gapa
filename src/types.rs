use std::fmt::Display;

use ratatui::{crossterm::event::Event, style::Stylize};

#[derive(Debug)]
struct Action {}

#[derive(Debug, PartialEq, Eq)]
enum Faction {
    Blob,
    Traides,
    Empire,
    Scrapper,
}
impl Display for Faction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Faction::Blob => write!(f, "B"),
            Faction::Traides => write!(f, "T"),
            Faction::Empire => write!(f, "E"),
            Faction::Scrapper => write!(f, "S"),
        }
    }
}
fn boo(faction: Faction) {
    println!("{}", faction)
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

trait HandleEvent {
    fn handle_event(&mut self, event: &Event);
}

impl Player {
    fn attack(&mut self) {}
    fn draw_hand(&mut self) {}
    fn draw_card(&mut self) {}
    fn play_card(&mut self, card: Card) {}
    fn by_card(&mut self) {}
}

mod game {}
