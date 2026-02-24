use std::fmt::Display;

use crate::player_id::PlayerId;

#[derive(Debug)]
pub enum Location {
    Discard,
    Played,
    Hand,
    DrawPile,
    CurrentCard,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Discard => write!(f, "discard"),
            Location::Played => write!(f, "played cards"),
            Location::Hand => write!(f, "hand"),
            Location::DrawPile => write!(f, "draw pile"),
            Location::CurrentCard => write!(f, "currrent card"),
        }
    }
}

pub mod player_id {}

#[derive(Debug)]
pub enum Selection {
    Shop {
        index: usize,
    },
    ShopExplorer,
    ActionButton,
    Deck {
        player: PlayerId,
        kind: Location,
        index: usize,
    },
}

impl Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Selection::Shop { index } => write!(f, "Inside shop at index {}", index),
            Selection::ShopExplorer => write!(f, "Shopping for explorer"),
            Selection::ActionButton => write!(f, "Wanting to do some action"),
            Selection::Deck {
                player,
                kind,
                index,
            } => write!(f, "Inside {} at index {} of Player {}", kind, index, player),
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::ActionButton
    }
}

#[derive(Default)]
pub struct SelectionInfo {
    pub selection_players: [Option<usize>; 2],
    pub selection_explorer: bool,
    pub selection_shop: Option<usize>,
    pub selection_action_button: bool,
}

impl Selection {
    pub fn get(&self) -> SelectionInfo {
        let mut selection = SelectionInfo::default();
        match self {
            Selection::Shop { index } => selection.selection_shop = Some(*index),
            Selection::ShopExplorer => selection.selection_explorer = true,
            Selection::ActionButton => selection.selection_action_button = true,
            Selection::Deck {
                index,
                player,
                kind,
            } => selection.selection_players[player] = Some(*index),
        };
        selection
    }
}
