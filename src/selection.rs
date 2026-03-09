use crate::{deck::Deck, game::Game, player_id::PlayerId};
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

#[derive(PartialEq, Eq)]
enum TopOrBot {
    Top,
    Bot,
}
impl TopOrBot {
    pub fn from(player: &PlayerId, current_player: &PlayerId) -> Self {
        if player == current_player {
            TopOrBot::Bot
        } else {
            TopOrBot::Top
        }
    }
    pub fn to_player(&self, current_player: &PlayerId) -> PlayerId {
        use TopOrBot::*;
        match self {
            Top => current_player.other(),
            Bot => *current_player,
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
    const fn next_down_position(&self, pos: &TopOrBot) -> (Location, TopOrBot) {
        use TopOrBot::*;
        match (self, pos) {
            (Explorer, _) | (Shop, _) => (Played, Bot),
            (Hand, Top) => (Played, Top),
            (Played, Top) => (Shop, Bot),
            (DrawPile, Top) => (Discard, Top),
            (Discard, Top) => (Discard, Bot),
            (Discard, Bot) => (DrawPile, Bot),
            (DrawPile, Bot) => (DrawPile, Bot),
            (Played, Bot) => (Hand, Bot),
            (Hand, Bot) => (Hand, Bot),
        }
    }
    const fn next_up_position(&self, pos: &TopOrBot) -> (Location, TopOrBot) {
        use TopOrBot::*;
        match (self, pos) {
            (Explorer, _) | (Shop, _) => (Played, Top),
            (Hand, Bot) => (Played, Bot),
            (Hand, Top) => (Hand, Top),
            (Played, Bot) => (Shop, Bot),
            (Played, Top) => (Hand, Top),
            (Discard, Bot) => (Discard, Top),
            (Discard, Top) => (DrawPile, Top),
            (DrawPile, Bot) => (Discard, Bot),
            (DrawPile, Top) => (DrawPile, Top),
        }
    }
    pub fn next_down(&self, player: &PlayerId, current_player: &PlayerId) -> (Location, PlayerId) {
        let pos = TopOrBot::from(player, current_player);
        let (loc, new_pos) = self.next_down_position(&pos);
        (loc, new_pos.to_player(current_player))
    }
    pub fn next_up(&self, player: &PlayerId, current_player: &PlayerId) -> (Location, PlayerId) {
        let pos = TopOrBot::from(player, current_player);
        let (loc, new_pos) = self.next_up_position(&pos);
        (loc, new_pos.to_player(current_player))
    }
    pub fn random() -> Self {
        match rand::random_range(0..6) {
            0 => Explorer,
            1 => Shop,
            2 => Hand,
            3 => Played,
            4 => Discard,
            5 => DrawPile,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod location_test {
    use crate::player_id::PlayerId::*;
    use crate::selection::Location::*;

    #[test]
    fn mouvement_test() {
        assert_eq!(Shop.next_up(&First, &First), (Played, Second));
        assert_eq!(Shop.next_down(&First, &First), (Played, First));
        assert_eq!(Shop.next_up(&First, &Second), (Played, First));
        assert_eq!(Played.next_down(&Second, &First), (Shop, First));

        assert_eq!(Hand.next_up(&Second, &Second), (Played, Second));
        assert_eq!(Hand.next_up(&First, &First), (Played, First));

        assert_eq!(Played.next_up(&Second, &Second), (Shop, Second));
        assert_eq!(Played.next_down(&Second, &Second), (Hand, Second));
        assert_eq!(Played.next_up(&First, &First), (Shop, First));

        assert_eq!(Discard.next_up(&Second, &Second), (Discard, First));
        assert_eq!(DrawPile.next_up(&Second, &Second), (Discard, Second));
        assert_eq!(DrawPile.next_down(&Second, &Second), (DrawPile, Second));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GamePosition {
    pub player: PlayerId,
    pub loc: Location,
    pub index: usize,
}

impl Display for GamePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at index {} of {} player",
            self.loc, self.index, self.player
        )
    }
}

impl Default for GamePosition {
    fn default() -> Self {
        Self {
            player: PlayerId::First,
            loc: Location::Explorer,
            index: 0,
        }
    }
}

impl GamePosition {
    pub fn next_right(&mut self) {
        *self = GamePosition {
            player: self.player,
            loc: self.loc.next_right(),
            index: 0,
        }
    }

    // todo that you never arrive in state (Hand of second player)
    // which is not visible
    pub fn next_left(&mut self) {
        *self = GamePosition {
            player: self.player,
            loc: self.loc.next_left(),
            index: 0,
        }
    }
    pub fn next_up(&mut self, current_player: &PlayerId) {
        let (loc, player) = self.loc.next_up(&self.player, current_player);
        *self = GamePosition {
            player,
            loc,
            index: 0,
        }
    }
    pub fn next_down(&mut self, current_player: &PlayerId) {
        let (loc, player) = self.loc.next_down(&self.player, current_player);
        *self = GamePosition {
            player,
            loc,
            index: 0,
        }
    }
    pub fn random() -> Self {
        Self {
            player: PlayerId::random(),
            loc: Location::random(),
            index: rand::random_range(0..10),
        }
    }
}
