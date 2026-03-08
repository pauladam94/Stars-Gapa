use crate::{
    action::{Action, ActionLoc, Condition, Data},
    card::{Card, Life},
    faction::{Faction, Factions},
};

use Action::*;
use Faction::*;
impl Card {
    pub const EMPTY_SHIP: Self = Card::DEFAULT;
    pub const fn ship() -> Self {
        Self::EMPTY_SHIP
    }
    pub const fn outpost() -> Self {
        let mut outpost = Card::ship();
        outpost.life = Some(Life::OutPost(0));
        outpost
    }
    pub const fn base() -> Self {
        let mut base = Card::ship();
        base.life = Some(Life::Base(0));
        base
    }
    pub const fn with_life(mut self, life: u32) -> Self {
        match &mut self.life {
            Some(Life::OutPost(past_life) | Life::Base(past_life)) => *past_life = life,
            None => panic!("Card is not a base or outpost cannot assign a life parameter"),
        }
        self
    }
    pub const fn with_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }
    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }
    pub fn with_faction(mut self, faction: Faction) -> Self {
        self.faction.push(faction);
        self
    }
    pub fn with_faction_condition(mut self, faction: Faction, action: Action) -> Self {
        self.actions.push(Complex {
            cond: Condition::FactionPlayed(Factions::new(vec![faction])),
            action: vec![action],
        });
        self
    }
    pub fn when_scraped(mut self, action: Action) -> Self {
        // TODO: push insides action when already a scrap action
        self.actions.push(Complex {
            cond: Condition::Action(Box::new(Action::Scrap {
                loc: ActionLoc::CurrentCard,
                nb: 1,
            })),
            action: vec![action],
        });
        self
    }
    pub fn when_faction_played(mut self, action: Action) -> Self {
        if let Some(my_faction) = self.faction.first().clone().cloned() {
            self.actions.push(Complex {
                cond: Condition::FactionPlayed(Factions::new(vec![my_faction.clone()])),
                action: vec![action],
            })
        } else {
            panic!("This ship has no faction but an action was addded with a faction condition")
        }
        self
    }
    pub const fn costing(mut self, gold: u32) -> Self {
        self.gold = gold;
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
    pub fn embassy_yacht() -> Self {
        Self::ship()
            .costing(3)
            .with_name("Embassy Yacht")
            .with_faction(Trade)
            .with_action(Authority(3))
            .with_action(Gold(2))
            .with_action(Complex {
                cond: Condition::GreaterThan(Data::NbBases, 2),
                action: vec![Draw(2)],
            })
    }
    pub fn trading_post() -> Self {
        Self::outpost()
            .costing(3)
            .with_name("Trading Post")
            .with_faction(Trade)
            .with_life(4)
            .with_action(Authority(1).or(Gold(1)))
            .when_scraped(Attack(3))
    }
    pub fn barter_world() -> Self {
        Self::base()
            .costing(4)
            .with_name("Barter World")
            .with_faction(Trade)
            .with_life(4)
            .with_action(Authority(2).or(Gold(2)))
            .when_scraped(Attack(5))
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
            .when_faction_played(Attack(2))
    }
    pub fn battle_station() -> Self {
        Self::outpost()
            .costing(3)
            .with_name("Battle Station")
            .with_faction(Machine)
            .with_life(5)
            .when_scraped(Attack(5))
    }
    pub fn supply_bot() -> Self {
        Self::ship()
            .costing(3)
            .with_name("Supply Bot")
            .with_faction(Machine)
            .with_action(Gold(2))
            .with_action(Scrap {
                loc: ActionLoc::DiscardOrHand,
                nb: 1,
            })
            .when_faction_played(Attack(2))
    }
    pub fn brain_world() -> Self {
        Self::outpost()
            .costing(8)
            .with_life(6)
            .with_name("Blob World")
            .with_action(Complex {
                cond: Condition::Action(Box::new(Scrap {
                    loc: ActionLoc::DiscardOrHand,
                    nb: 2,
                })),
                action: vec![Draw(2)],
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
            .when_faction_played(Attack(2))
    }
    pub fn federation_shuttle() -> Self {
        Self::ship()
            .costing(1)
            .with_name("Federation Shuttle")
            .with_faction(Trade)
            .with_action(Gold(2))
            .when_faction_played(Authority(4))
    }
    pub fn imperial_fighter() -> Self {
        Self::ship()
            .costing(1)
            .with_name("Imperial fighter")
            .with_faction(Star)
            .with_action(Attack(2))
            .with_action(OpponentDiscard(1))
            .when_faction_played(Attack(2))
    }
    pub fn imperial_frigate() -> Self {
        Self::ship()
            .costing(3)
            .with_name("Imperial Frigate")
            .with_faction(Star)
            .with_action(Attack(4))
            .with_action(OpponentDiscard(1))
            .when_faction_played(Attack(2))
            .when_scraped(Draw(1))
    }
    pub fn survey_ship() -> Self {
        Self::ship()
            .costing(3)
            .with_name("Survey Ship")
            .with_faction(Star)
            .with_action(Gold(1))
            .with_action(Draw(1))
            .when_scraped(OpponentDiscard(1))
    }

    // BLOBS
    pub fn blob_fighter() -> Self {
        Self::ship()
            .costing(1)
            .with_name("Blob Fighter")
            .with_faction(Blob)
            .with_action(Attack(3))
            .when_faction_played(Draw(1))
    }
    pub fn battle_pod() -> Self {
        Self::ship()
            .costing(2)
            .with_name("Battle Pod")
            .with_faction(Blob)
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
            .with_life(3)
            .with_action(Attack(1))
            .when_scraped(Gold(3))
    }
    pub fn ram() -> Self {
        Self::ship()
            .costing(3)
            .with_name("Ram")
            .with_faction(Blob)
            .with_action(Attack(5))
            .when_faction_played(Attack(2))
            .when_scraped(Gold(3))
    }
}
