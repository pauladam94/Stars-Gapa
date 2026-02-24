use crate::{
    action::{Action, Condition},
    faction::Factions,
    selection::Location,
};
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use std::{fmt::Display, slice::Iter};

#[derive(Debug)]
enum OutPost {
    OutPost,
    NotOutpost,
}

#[derive(Debug)]
pub struct CardInfo {
    /// Name of a [Card]
    name: &'static str,
    /// All the action(s) of a [Card]
    actions: Vec<Action>,
    /// Price of a [Card]
    pub gold: u32,
    /// Faction(s) of a [Card]
    pub faction: Factions,
}

#[derive(Debug)]
pub enum Card {
    Ship(CardInfo),
    OutPost { life: u32, info: CardInfo },
    Base { life: u32, info: CardInfo },
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let actions = &self.get_info().actions;
        for (i, action) in actions.iter().enumerate() {
            write!(f, "{}", action)?;
            if i != actions.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Card {
    /// Creates a [CardWidget] from a [Card]
    /// Remark: [CardWidget] implements the [Widget] trait and can be displayed
    pub const fn widget<'a>(&'a self) -> CardWidget<'a> {
        CardWidget {
            card: self,
            selected: false,
        }
    }
    /// Gets the [CardInfo] a given [Card]
    pub const fn get_info<'a>(&'a self) -> &'a CardInfo {
        let (Card::Ship(info) | Card::OutPost { info, .. } | Card::Base { info, .. }) = self;
        info
    }
    /// Gets the [CardInfo] a given [Card] in a mutable manner
    pub const fn get_mut_info<'a>(&'a mut self) -> &'a mut CardInfo {
        let (Card::Ship(info) | Card::OutPost { info, .. } | Card::Base { info, .. }) = self;
        info
    }
    /// Creates an iterator over [Action] from a [Card]
    pub fn iter(&self) -> Iter<'_, Action> {
        self.get_info().actions.iter()
    }
}
use Action::*;
impl Card {
    pub const EMPTY_CARD: Self = Self::Ship(CardInfo {
        name: "",
        actions: vec![],
        gold: 0,
        faction: Factions::NONE,
    });
    pub fn with_name(mut self, name: &'static str) -> Self {
        let info = self.get_mut_info();
        info.name = name;
        self
    }
    pub fn viper() -> Self {
        Self::Ship(CardInfo {
            name: "Viper",
            actions: vec![Attack(1)],
            gold: 1,
            faction: Factions::NONE,
        })
    }
    pub fn scout() -> Self {
        Self::Ship(CardInfo {
            name: "Scout",
            actions: vec![Gold(1)],
            gold: 1,
            faction: Factions::NONE,
        })
    }
    pub fn explorer() -> Self {
        Self::Ship(CardInfo {
            name: "Explorer",
            actions: vec![
                Action::Gold(2),
                Action::Complex {
                    condition: Condition::Scrap(Location::CurrentCard),
                    result: vec![Attack(2)],
                },
            ],
            gold: 2,
            faction: Factions::NONE,
        })
    }
}

#[derive(Debug)]
pub struct CardWidget<'a> {
    selected: bool,
    card: &'a Card,
}

impl CardWidget<'_> {
    pub fn selected(self) -> Self {
        Self {
            selected: true,
            ..self
        }
    }
    pub fn set_selection(self, selected: bool) -> Self {
        Self { selected, ..self }
    }
}

impl<'a> Widget for CardWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (Card::Ship(info) | Card::OutPost { info, .. } | Card::Base { info, .. }) = self.card;
        Paragraph::new(format!("{}", self.card))
            .centered()
            .block(
                Block::bordered()
                    .title_top(
                        Line::from(if info.gold == 0 {
                            String::from("")
                        } else {
                            format!("{}", Action::Gold(info.gold))
                        })
                        .right_aligned(),
                    )
                    .title_top(Line::from(info.name).centered())
                    .title_top(Line::from(format!("{}", info.faction)).left_aligned())
                    .border_style(if self.selected {
                        Style::new().red()
                    } else {
                        Style::new()
                    }),
            )
            .render(area, buf);
    }
}
