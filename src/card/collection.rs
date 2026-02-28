use crate::{
    action::{Action, ActionLoc, Condition},
    card::{Card, CardInfo},
    faction::{Faction, Factions},
};

use Action::*;
use Faction::*;
impl Card {
    pub const EMPTY_SHIP: Self = Self::Ship(CardInfo::DEFAULT);
    pub const fn ship() -> Self {
        Self::EMPTY_SHIP
    }
    pub const fn outpost() -> Self {
        Self::OutPost {
            life: 0,
            info: CardInfo::DEFAULT,
        }
    }
    pub const fn base() -> Self {
        Self::Base {
            life: 0,
            info: CardInfo::DEFAULT,
        }
    }
    pub const fn with_life(mut self, life: u32) -> Self {
        let new_life = life;
        match &mut self {
            Card::Ship(_) => panic!(),
            Card::OutPost { life, info } | Card::Base { life, info } => *life = new_life,
        }
        self
    }
    pub const fn with_name(mut self, name: &'static str) -> Self {
        let info = self.get_mut_info();
        info.name = name;
        self
    }
    pub fn with_action(mut self, action: Action) -> Self {
        self.get_mut_info().actions.push(action);
        self
    }
    pub fn with_faction(mut self, faction: Faction) -> Self {
        self.get_mut_info().faction.push(faction);
        self
    }
    pub fn with_faction_condition(mut self, faction: Faction, action: Action) -> Self {
        self.get_mut_info().actions.push(Complex {
            condition: Condition::FactionPlayed(Factions::new(vec![faction])),
            result: vec![action],
        });
        self
    }
    pub fn when_scraped(mut self, action: Action) -> Self {
        self.get_mut_info().actions.push(Complex {
            condition: Condition::Action(Box::new(Action::Scrap {
                loc: ActionLoc::CurrentCard,
                nb: 1,
            })),
            result: vec![action],
        });
        self
    }
    pub fn when_faction_played(mut self, action: Action) -> Self {
        if let Some(my_faction) = self.get_info().faction.first().clone().cloned() {
            self.get_mut_info().actions.push(Complex {
                condition: Condition::FactionPlayed(Factions::new(vec![my_faction.clone()])),
                result: vec![action],
            })
        } else {
            panic!("This ship has no faction but an action was addded with a faction condition")
        }
        self
    }
    pub const fn costing(mut self, gold: u32) -> Self {
        self.get_mut_info().gold = gold;
        self
    }

    // STARTERS

    pub fn viper() -> Self {
        Self::ship()
            .costing(1)
            .with_name("Viper")
            .with_action(Attack(1))
    }
    pub fn scout() -> Self {
        Self::ship()
            .costing(1)
            .with_name("Scout")
            .with_action(Gold(1))
    }
    pub fn explorer() -> Self {
        Self::ship()
            .costing(2)
            .with_name("Explorer")
            .with_action(Gold(2))
            .when_scraped(Attack(2))
    }

    // TRADES
    pub fn cutter() -> Self {
        Self::ship()
            .costing(2)
            .with_name("Cutter")
            .with_action(Gold(2))
            .with_action(Authority(4))
            .with_faction(Trade)
            .with_faction_condition(Trade, Attack(4))
    }

    // Machines
    pub fn trade_bot() -> Self {
        Self::ship()
            .costing(1)
            .with_name("Trade Bot")
            .with_action(Gold(1))
            .with_action(Scrap {
                loc: ActionLoc::DiscardOrHand,
                nb: 1,
            })
            .with_faction(Machine)
            .with_faction_condition(Machine, Attack(2))
    }
    pub fn missile_bot() -> Self {
        Self::ship()
            .costing(2)
            .with_name("Missile Bot")
            .with_faction(Machine)
            .with_action(Attack(2))
            .with_action(Scrap {
                loc: ActionLoc::DiscardOrHand,
                nb: 1,
            })
            .with_faction_condition(Machine, Attack(2))
    }
    pub fn brain_world() -> Self {
        Self::outpost()
            .costing(8)
            .with_life(6)
            .with_name("Blob World")
            .with_action(Scrap {
                loc: ActionLoc::DiscardOrHand,
                nb: 2,
            })
            .with_faction(Machine)
    }

    // Star
    pub fn corvette() -> Self {
        Self::ship()
            .costing(2)
            .with_name("Corvette")
            .with_faction(Star)
            .with_action(Attack(1))
            .with_action(Draw(1))
            .with_faction_condition(Star, Attack(2))
    }
    pub fn federation_shuttle() -> Self {
        Self::ship()
            .costing(1)
            .with_name("fedaration shuttle")
            .with_faction(Trade)
            .with_action(Gold(2))
            .with_faction_condition(Trade, Authority(4))
    }
    pub fn imperial_fighter() -> Self {
        Self::ship()
            .costing(1)
            .with_name("Imperial fighter")
            .with_faction(Star)
            .with_action(Attack(2))
            .with_action(OpponentDiscard(1))
            .with_faction_condition(Star, Attack(2))
    }

    // BLOBS
    pub fn blob_fighter() -> Self {
        Self::ship()
            .costing(1)
            .with_name("Blob fighter")
            .with_faction(Blob)
            .with_action(Attack(3))
            .with_faction_condition(Blob, Draw(1))
    }
    pub fn battle_pod() -> Self {
        Self::ship()
            .costing(2)
            .with_name("Battle pod")
            .with_action(Attack(4))
            .with_action(Scrap {
                loc: ActionLoc::Shop,
                nb: 1,
            })
            .when_faction_played(Attack(2))
    }
    pub fn trade_pod() -> Self {
        Self::ship()
            .costing(2)
            .with_name("Trade Pod")
            .with_faction(Blob)
            .with_action(Gold(3))
            .when_faction_played(Attack(2))
    }
    pub fn blob_wheel() -> Self {
        Self::base()
            .costing(3)
            .with_name("Blob Wheel")
            .with_faction(Blob)
            .with_action(Attack(1))
            .when_scraped(Gold(3))
    }
}
