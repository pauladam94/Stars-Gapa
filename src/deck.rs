use crate::{
    action::{ATTACK_STR, AUTHORITY_STR, Action, GOLD_STR},
    card::Card,
    faction::Faction,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::{Buffer, Rect},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};
use std::fmt::Display;

#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub const EMPTY: Self = Self(Vec::new());

    /// Remove a random card from a [Deck]
    /// return Err(())         if the deck is empty
    /// return Ok(random_card) otherwise
    ///
    /// Remark: we use a uniform distribution
    pub fn remove_random(&mut self) -> Result<Card, ()> {
        if self.is_empty() {
            return Err(());
        } else {
            let index = rand::random_range(0..self.len());
            Ok(self.remove(index))
        }
    }
    /// Remove a [Card] from a [Deck] at a spefic index
    /// Panic if `index` is out of bound
    pub fn remove(&mut self, index: usize) -> Card {
        self.0.remove(index)
    }
    /// Remove a [Card] from a [Deck] at a spefic index
    /// Panic if `index` is out of bound
    ///
    /// Remark: this function has a O(1) complexity
    pub fn remove_last(&mut self) -> Card {
        self.0.remove(self.0.len() - 1)
    }
    /// Add the a [Card] to a [Deck]
    pub fn push(&mut self, card: Card) {
        self.0.push(card)
    }
    /// Pick the card at an `index` from a [Deck]
    pub fn pick_at(&mut self, index: usize, from: &mut Deck) {
        self.0.push(from.0.remove(index))
    }
    pub fn len(&self) -> usize {
        let Deck(cards) = self;
        cards.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn starter_deck_player() -> Self {
        let mut deck = vec![];
        for _ in 0..2 {
            deck.push(Card::viper());
        }
        for _ in 0..8 {
            deck.push(Card::scout());
        }
        Self(deck)
    }
    pub fn starter_complete_deck() -> Self {
        let mut deck = vec![];

        for _ in 0..2 {
            deck.push(Card::viper());
        }
        for _ in 0..2 {
            deck.push(Card::scout());
        }
        // Machine
        deck.push(Card::brain_world());
        // Trade
        for _ in 0..3 {
            deck.push(Card::cutter());
        }

        // Block
        for _ in 0..3 {
            deck.push(Card::blob_wheel())
        }

        Self(deck)
    }

    pub fn widget<'a, 'b>(&'a self) -> DeckWidget<'a, 'b> {
        DeckWidget {
            name: "",
            selection: None,
            hidden: false,
            deck: self,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Card> {
        self.0.iter()
    }
    pub fn get_stats(&self) -> DeckStats {
        DeckStats::default().analyze_deck(self)
    }
}

#[derive(Debug, Default)]
pub struct DeckStats {
    gold: f32,
    attack: f32,
    authority: f32,
    discard: f32,
    scrap: f32,
    opponentdiscard: f32,
    draw: f32,
    nb_cards: f32,
    factions: [u8; 4],
}

impl DeckStats {
    pub fn analyze_deck(self, deck: &Deck) -> Self {
        deck.iter()
            .fold(self, |stats, card| stats.analyze_card(card))
    }
    pub fn analyze_card(mut self, card: &Card) -> Self {
        self.nb_cards += 1.;
        let stats = card
            .get_info()
            .faction
            .iter()
            .fold(self, |stats, faction| stats.analyze_faction(faction));
        card.iter()
            .fold(stats, |stats, action| stats.analyze_action(action))
    }
    pub fn analyze_action(mut self, action: &Action) -> Self {
        use Action::*;
        match action {
            Gold(i) => self.gold += *i as f32,
            Attack(i) => self.attack += *i as f32,
            Authority(i) => self.authority += *i as f32,
            Discard(i) => self.discard += *i as f32,
            Scrap { loc, nb } => self.scrap += *nb as f32,
            Draw(i) => self.draw += *i as f32,
            OpponentDiscard(i) => self.opponentdiscard += *i as f32,
            Complex { condition, result } => (),
        }
        self
    }
    pub fn analyze_faction(mut self, faction: &Faction) -> Self {
        self.factions[faction.to_usize()] += 1;
        self
    }

    pub const fn mean_attack(&self) -> f32 {
        self.attack / self.nb_cards
    }
    pub const fn mean_gold(&self) -> f32 {
        self.gold / self.nb_cards
    }
    pub const fn mean_authority(&self) -> f32 {
        self.authority / self.nb_cards
    }
}

impl std::ops::Sub<DeckStats> for DeckStats {
    type Output = DeckStats;

    fn sub(self, rhs: DeckStats) -> Self::Output {
        // todo
        self
    }
}

impl Display for DeckStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}\n{}{}\n{}{}",
            self.mean_gold(),
            GOLD_STR,
            self.mean_attack(),
            ATTACK_STR,
            self.mean_authority(),
            AUTHORITY_STR
        )
    }
}

impl std::ops::Index<usize> for Deck {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl std::ops::IndexMut<usize> for Deck {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub struct DeckWidget<'a, 'b> {
    name: &'b str,
    selection: Option<usize>,
    hidden: bool,
    deck: &'a Deck,
}

impl<'a, 'b> DeckWidget<'a, 'b> {
    pub fn hidden(self) -> Self {
        Self {
            hidden: true,
            ..self
        }
    }
    pub fn set_name<'c>(self, name: &'c str) -> DeckWidget<'a, 'c> {
        DeckWidget { name: name, ..self }
    }
    pub fn set_selection(self, selection: Option<usize>) -> Self {
        Self { selection, ..self }
    }
}

impl Widget for DeckWidget<'_, '_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .title_top(Line::from(self.name).left_aligned())
            .border_type(BorderType::LightDoubleDashed)
            .render(area, buf);
        let area = area.inner(Margin::new(1, 1));
        if self.hidden {
            Paragraph::new(format!("{}", self.deck.get_stats()))
                .centered()
                .block(Block::bordered())
                .render(area, buf);
            return;
        }
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(1); self.deck.len()])
            .split(area);

        for (i, card) in (0..layout.len()).zip(self.deck.iter()) {
            card.widget()
                .set_selection(self.selection.is_some() && self.selection.unwrap() == i)
                .render(layout[i], buf);
        }
    }
}
