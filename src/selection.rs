use crate::{deck::Deck, game::Game, player::Player, player_id::PlayerId};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Location {
    Explorer,
    Shop,
    Hand,
    Played,
    Discard,
    DrawPile,
}
use Location::*;

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Explorer => write!(f, "explorer"),
            Shop => write!(f, "shop"),
            Hand => write!(f, "hand"),
            Played => write!(f, "played"),
            Discard => write!(f, "discard"),
            DrawPile => write!(f, "draw pile"),
        }
    }
}

#[derive(Debug)]
pub struct Selection {
    pub player: PlayerId,
    pub loc: Location,
    pub index: usize,
}

impl Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at index {} | Turn of {} Player",
            self.player, self.loc, self.index
        )
    }
}

impl std::ops::Index<&Selection> for &Game {
    type Output = Deck;

    fn index(&self, index: &Selection) -> &Self::Output {
        &self.players[index.player][index.loc]
    }
}
impl std::ops::Index<&Selection> for &mut Game {
    type Output = Deck;

    fn index<'a>(&'a self, index: &Selection) -> &'a Self::Output {
        &self.players[index.player][index.loc]
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            player: PlayerId::First,
            loc: Location::Explorer,
            index: 0,
        }
    }
}

impl Selection {
    pub fn player(&self, player: PlayerId) -> Option<usize> {
        if self.player == player {
            Some(self.index)
        } else {
            None
        }
    }
    pub fn explorer(&self) -> Option<usize> {
        if self.loc == Explorer {
            Some(self.index)
        } else {
            None
        }
    }
    pub fn shop(&self) -> Option<usize> {
        if self.loc == Shop {
            Some(self.index)
        } else {
            None
        }
    }
    // todo
    pub fn next_up(&mut self) {}
    // todo
    pub fn next_down(&mut self) {}
}
