use ratatui::crossterm::event::Event;

use crate::cards::{Card, Deck};

#[derive(Debug)]
pub struct Player {
    pub(crate) hand: Deck,
    pub(crate) draw_pile: Deck,
    pub(crate) discard: Deck,
    pub(crate) gold: u32,
    pub(crate) pv: u32,
    pub(crate) attack: u32,
}

pub(crate) trait HandleEvent {
    fn handle_event(&mut self, event: &Event);
}

impl Player {
    pub(crate) fn attack(&mut self) {}
    pub(crate) fn draw_hand(&mut self) {}
    pub(crate) fn draw_card(&mut self) {}
    pub(crate) fn play_card(&mut self, card: Card) {}
    pub(crate) fn by_card(&mut self) {}
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hand: Deck::EMPTY,
            draw_pile: Deck::starter(),
            discard: Deck::EMPTY,
            gold: 0,
            attack: 0,
            pv: 50,
        }
    }
}
