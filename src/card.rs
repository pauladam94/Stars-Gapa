use crate::{
    action::{Action, BASE_STR, OUTPOST_STR},
    faction::Factions,
};
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};
use std::{fmt::Display, slice::Iter};

pub mod collection;

#[derive(Debug)]
pub enum Life {
    OutPost(u32),
    Base(u32),
}
#[derive(Debug, Default)]
pub struct Card {
    /// Name of a [Card]
    name: &'static str,
    /// All the action(s) of a [Card]
    actions: Vec<Action>,
    /// Price of a [Card]
    pub gold: u32,
    /// Faction(s) of a [Card]
    pub faction: Factions,
    pub life: Option<Life>,
}

impl Card {
    pub const DEFAULT: Self = Card {
        name: "",
        actions: vec![],
        gold: 0,
        faction: Factions::NONE,
        life: None,
    };
    pub const fn is_ship(&self) -> bool {
        self.life.is_none()
    }
    pub const fn is_base(&self) -> bool {
        match self.life {
            Some(Life::Base(_)) => true,
            _ => false,
        }
    }
    pub const fn is_outpost(&self) -> bool {
        match self.life {
            Some(Life::OutPost(_)) => true,
            _ => false,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let actions = &self.actions;
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
    /// Creates an iterator over [Action] from a [Card]
    pub fn iter(&self) -> Iter<'_, Action> {
        self.actions.iter()
    }
    pub fn initials<'a>(&'a self) -> impl Display {
        CardInitials(self.name)
    }
}

pub struct CardInitials<'a>(&'a str);
impl Display for CardInitials<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.0.split(' ') {
            write!(f, "{}.", s.chars().nth(0).unwrap())?;
        }
        Ok(())
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
        let card = self.card;
        let gold_string = if card.gold == 0 {
            String::from("")
        } else {
            format!("{}", Action::Gold(card.gold))
        };
        let life_string = match card.life {
            Some(Life::Base(i)) => format!("{i}{}", BASE_STR),
            Some(Life::OutPost(i)) => format!("{i}{}", OUTPOST_STR),
            None => format!(""),
        };
        let name_string =
            if card.name.len() + life_string.chars().count() + 2 >= area.width as usize{
                format!("{}", card.initials())
            } else {
                String::from(card.name)
            };
        Paragraph::new(format!("{}", self.card))
            .centered()
            .style(Style::new())
            .centered()
            .block(
                Block::bordered()
                    .border_type(if self.selected {
                        BorderType::Double
                    } else {
                        BorderType::Rounded
                    })
                    .title_top(Line::from(gold_string).right_aligned())
                    .title_bottom(life_string)
                    .title_bottom(Line::from(name_string).right_aligned())
                    .title_top(Line::from(format!("{}", card.faction)).left_aligned())
                    .border_style(if self.selected {
                        Style::new().green()
                    } else {
                        Style::new()
                    }),
            )
            .render(area, buf);
    }
}
