use crate::{
    action::{ATTACK_STR, AUTHORITY_STR, Action, GOLD_STR},
    card::Card,
    deck::Deck,
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
            self.draw_card()
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
                    self.draw_card();
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

    /// Draw a [Card] from the `draw_pile` of a [Player] to put in it's `hand`
    pub fn draw_card(&mut self) {
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
