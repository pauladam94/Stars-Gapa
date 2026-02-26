use crate::{
    action::{ATTACK_STR, AUTHORITY_STR, Action, GOLD_STR},
    card::Card,
    deck::Deck,
    selection::Location,
};
use std::fmt::Display;

#[derive(Debug)]
pub struct Player {
    pub played: Deck,
    pub hand: Deck,
    pub draw_pile: Deck,
    pub discard: Deck,
    pub gold: u32,
    pub authority: u32,
    pub attack: u32,
}

impl Player {
    /// Tries to buy the [Card] at a specific index in the shop for a [Player]
    /// returns Ok(())  if the player can buy it :
    ///                     - buy the card
    ///                     - refresh the shop
    /// returns Err(()) otherwise.
    pub fn buy_from_shop(
        &mut self,
        // Complete deck
        deck: &mut Deck,
        shop: &mut Deck,
        index: usize,
    ) -> Result<(), ()> {
        let info_card = shop[index].get_info();
        if self.gold >= info_card.gold {
            self.gold -= info_card.gold;
            self.discard.push(std::mem::replace(
                &mut shop[index],
                deck.remove_random().unwrap(),
            ));
            Ok(())
        } else {
            Err(())
        }
    }

    /// Tries to buy a [Card] by a [Player]
    /// - returns Ok(())  if the player can buy it : do it
    /// - returns Err(()) otherwise.
    pub fn buy_card(&mut self, card: Card) -> Result<(), ()> {
        let price_card = card.get_info().gold;
        if self.gold >= price_card {
            self.gold -= price_card;
            self.discard.push(card);
            Ok(())
        } else {
            Err(())
        }
    }

    /// Gets the attack of a [Player] and set it to 0
    pub fn get_attack(&mut self) -> u32 {
        let attack = self.attack;
        self.attack = 0;
        attack
    }

    /// Draw a complete hand of 5 cards for a [Player]
    pub fn draw_hand(&mut self) {
        for _ in 0..5 {
            self.draw_random_card()
        }
    }

    /// Apply an [Action] to a [Player]
    /// changing for example the money the player has.
    pub fn apply_action(&mut self, action: &Action) {
        use Action::*;
        match action {
            Gold(i) => self.gold += i,
            Attack(i) => self.attack += i,
            Authority(i) => self.authority += i,
            Discard(_) => todo!(),
            Scrap { loc, nb } => todo!(),
            Draw(i) => {
                for _ in 0..*i {
                    self.draw_random_card();
                }
            }
            OpponentDiscard(i) => (),
            Complex { condition, result } => (),
        }
    }

    /// Apply the [Action] of a [Card] to a [Player]
    pub fn apply_card(&mut self, card: &Card) {
        for action in card.iter() {
            self.apply_action(action)
        }
    }

    /// Draw a random [Card] from the `draw_pile` of a [Player] to put in it's `hand`
    ///
    /// If the draw_pile is empty then the `discard` is put inside the
    /// `draw_pile` before drawing a new card
    pub fn draw_random_card(&mut self) {
        if self.draw_pile.is_empty() {
            for _ in 0..self.discard.len() {
                self.draw_pile.push(self.discard.remove_last());
            }
            // If the draw pile is still empty
            // there is no more card to draw
            if self.draw_pile.is_empty() {
                return;
            }
        }

        if let Ok(card) = self.draw_pile.remove_random() {
            self.hand.push(card)
        }
    }

    /// Play a [Card] of a [Player] at a specific index
    pub fn play_card(&mut self, index: usize) {
        let card = self.hand.remove(index);
        self.apply_card(&card);
        self.played.push(card);
    }
    pub fn by_card(&mut self) {}
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}\n{}{}\n{}{}",
            self.gold, GOLD_STR, self.attack, ATTACK_STR, self.authority, AUTHORITY_STR
        )
    }
}
impl Default for Player {
    fn default() -> Self {
        Self {
            hand: Deck::EMPTY,
            draw_pile: Deck::starter_deck_player(),
            discard: Deck::EMPTY,
            gold: 0,
            attack: 0,
            authority: 50,
            played: Deck::EMPTY,
        }
    }
}

/// Implementation of the operation : `player`[`index`]
/// - `player` a [Player]
/// - `index` a [Location]
impl std::ops::IndexMut<&Location> for Player {
    fn index_mut(&mut self, index: &Location) -> &mut Self::Output {
        use Location::*;
        match index {
            DiscardOrHand => &mut self.discard,
            Played => &mut self.played,
            Hand => &mut self.hand,
            DrawPile => &mut self.draw_pile,
            CurrentCard => panic!("Cannot Index Player with location \"current card\""),
        }
    }
}
/// Implementation of the operation : `player`[`index`]
/// - `player` a [Player]
/// - `index` a [Location]
impl std::ops::IndexMut<Location> for Player {
    fn index_mut(&mut self, index: Location) -> &mut Self::Output {
        &mut self[&index]
    }
}
/// Implementation of the operation : `player`[`index`]
/// - `player` a [Player]
/// - `index` a [Location]
impl std::ops::Index<&mut Location> for Player {
    type Output = Deck;

    fn index(&self, index: &mut Location) -> &Self::Output {
        let index: &Location = index;
        &self[index]
    }
}
/// Implementation of the operation : `player`[`index`]
/// - `player` a [Player]
/// - `index` a [Location]
impl std::ops::Index<&Location> for Player {
    type Output = Deck;

    fn index(&self, index: &Location) -> &Self::Output {
        use Location::*;
        match index {
            DiscardOrHand => &self.discard,
            Played => &self.played,
            Hand => &self.hand,
            DrawPile => &self.draw_pile,
            CurrentCard => panic!("Cannot Index Player with location \"current card\""),
        }
    }
}
/// Implementation of the operation : `player`[`index`]
/// - `player` a [Player]
/// - `index` a [Location]
impl std::ops::Index<Location> for Player {
    type Output = Deck;

    fn index(&self, index: Location) -> &Self::Output {
        &self[&index]
    }
}
