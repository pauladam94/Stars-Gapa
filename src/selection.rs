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
impl Location {
    pub const fn next_right(&self) -> Location {
        match self {
            Explorer => Shop,
            Shop => Shop,
            Hand => DrawPile,
            Played => Discard,
            Discard => Discard,
            DrawPile => DrawPile,
        }
    }
    pub const fn next_left(&self) -> Location {
        match self {
            Explorer => Explorer,
            Shop => Explorer,
            Hand => Hand,
            Played => Played,
            Discard => Played,
            DrawPile => Hand,
        }
    }
    const fn next_down_first_player(&self, player: &PlayerId) -> (Location, PlayerId) {
        use PlayerId::*;
        match (self, player) {
            (Explorer, _) | (Shop, _) => (Played, First),
            (Hand, First) => (Hand, First),
            (Hand, Second) => (Played, Second),
            (Played, First) => (Hand, First),
            (Played, Second) => (Shop, First),
            (Discard, First) => (DrawPile, First),
            (Discard, Second) => (Shop, First),
            (DrawPile, First) => (DrawPile, First),
            (DrawPile, Second) => (Discard, Second),
        }
    }
    const fn next_up_first_player(&self, player: &PlayerId) -> (Location, PlayerId) {
        use PlayerId::*;
        match (self, player) {
            (Explorer, _) | (Shop, _) => (Played, Second),
            (Hand, First) => (Played, First),
            (Hand, Second) => (Hand, Second),
            (Played, First) => (Shop, First),
            (Played, Second) => (Hand, Second),
            (Discard, First) => (Shop, First),
            (Discard, Second) => (DrawPile, Second),
            (DrawPile, First) => (Discard, First),
            (DrawPile, Second) => (DrawPile, Second),
        }
    }
    pub const fn next_down(
        &self,
        player: &PlayerId,
        current_player: &PlayerId,
    ) -> (Location, PlayerId) {
        use PlayerId::*;
        match current_player {
            First => self.next_down_first_player(player),
            Second => self.next_up_first_player(player),
        }
    }
    pub const fn next_up(
        &self,
        player: &PlayerId,
        current_player: &PlayerId,
    ) -> (Location, PlayerId) {
        use PlayerId::*;
        match current_player {
            First => self.next_up_first_player(player),
            Second => self.next_down_first_player(player),
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
            self.loc, self.index, self.player
        )
    }
}

impl std::ops::Index<&Selection> for Game {
    type Output = Deck;

    fn index(&self, index: &Selection) -> &Self::Output {
        match index.loc {
            Explorer => &self.explorer,
            Shop => &self.shop,
            _ => &self.players[index.player][index.loc],
        }
    }
}
impl std::ops::Index<&Selection> for &mut Game {
    type Output = Deck;

    fn index<'a>(&'a self, index: &Selection) -> &'a Self::Output {
        match index.loc {
            Explorer => &self.explorer,
            Shop => &self.shop,
            _ => &self.players[index.player][index.loc],
        }
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
    pub fn player(&self, player: PlayerId, loc: Location) -> Option<usize> {
        if self.player == player && self.loc == loc {
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
    pub fn next_right(&mut self) {
        *self = Selection {
            player: self.player,
            loc: self.loc.next_right(),
            index: 0,
        };
    }
    pub fn next_left(&mut self) {
        *self = Selection {
            player: self.player,
            loc: self.loc.next_left(),
            index: 0,
        };
    }
    // todo
    pub fn next_up(&mut self, current_player: &PlayerId) {
        let (loc, player) = self.loc.next_up(&self.player, current_player);
        *self = Selection {
            player,
            loc,
            index: 0,
        };
    }
    // todo
    pub fn next_down(&mut self, current_player: &PlayerId) {
        let (loc, player) = self.loc.next_down(&self.player, current_player);
        *self = Selection {
            player,
            loc,
            index: 0,
        };
    }
}
