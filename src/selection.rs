#[derive(Debug)]
pub enum DeckKind {
    Discard,
    Played,
    Hand,
    DrawPile,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerId {
    First,
    Second,
}

#[derive(Debug)]
pub enum Selection {
    Shop {
        index: usize,
    },
    ShopExplorer,
    ActionButton,
    Deck {
        index: usize,
        player: PlayerId,
        kind: DeckKind,
    },
}

impl Default for Selection {
    fn default() -> Self {
        todo!()
    }
}
