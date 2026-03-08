use crate::selection::GamePosition;

pub enum Event {
    /// Buy a card from the shop at a given index
    Buy(usize),
    /// Buy an explorer
    BuyExplorer,
    /// Buy a card at a given index in the hand [Deck] of a [Player]
    Play(usize),
    /// Activate a card at a given index in the played [Deck]
    /// of a [Player]
    Activate(usize),
    /// Choose multiple cards anywhere in the [Game]
    /// for any action
    Choose(Vec<GamePosition>),
    /// Triggers an Attack against the opponent
    Attack,
    /// Go to the next turn
    NextTurn,
}

impl Event {
    pub fn random() -> Self {
        use Event::*;
        match rand::random_range(0..6) {
            0 => Buy(rand::random_range(0..5)),
            1 => BuyExplorer,
            2 => Play(rand::random_range(0..5)),
            3 => Activate(rand::random_range(0..15)),
            4 => Choose(
                (0..rand::random_range(0..2))
                    .map(|_| GamePosition::random())
                    .collect(),
            ),
            5 => Attack,
            6 => NextTurn,
            _ => unreachable!(),
        }
    }
}
