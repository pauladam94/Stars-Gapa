use crate::{
    action::{ATTACK_STR, AUTHORITY_STR, Action, GOLD_STR},
    card::Card,
    faction::Faction,
    game::Game,
    player_id::PlayerId,
    selection::{GamePosition, Location},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::{Buffer, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
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

        // Trade
        for _ in 0..3 {
            deck.push(Card::cutter());
        }
        // Machine
        for _ in 0..3 {
            deck.push(Card::trade_bot());
            deck.push(Card::missile_bot());
        }
        deck.push(Card::brain_world());

        // Star
        for _ in 0..3 {
            deck.push(Card::corvette());
            deck.push(Card::federation_shuttle());
            deck.push(Card::imperial_fighter());
        }

        // Blob
        for _ in 0..3 {
            deck.push(Card::blob_fighter());
            deck.push(Card::battle_pod());
            deck.push(Card::trade_pod());
            deck.push(Card::blob_wheel());
        }

        Self(deck)
    }

    pub fn widget<'a, 'b>(&'a self) -> DeckWidget<'a, 'b> {
        DeckWidget {
            name: "",
            hidden: false,
            deck: self,
            selection: vec![],
            max_cols: None,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Card> {
        self.0.iter()
    }
    pub fn get_stats(&self) -> impl Display {
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
            Complex {
                cond: condition,
                action: result,
            } => (),
            Copy => (),
            Or(action, action1) => (),
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
            "{:.2}{GOLD_STR}\n{:.2}{ATTACK_STR}\n{:.2}{AUTHORITY_STR}\n{} cards",
            self.mean_gold(),
            self.mean_attack(),
            self.mean_authority(),
            self.nb_cards
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
    selection: Vec<usize>,
    hidden: bool,
    max_cols: Option<usize>,
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
    pub fn set_max_cols(self, max_cols: usize) -> DeckWidget<'a, 'b> {
        DeckWidget {
            max_cols: Some(max_cols),
            ..self
        }
    }
    pub fn set_selection(mut self, game: &Game, loc: Location, playerid: PlayerId) -> Self {
        use Location::*;
        pub fn get_index(pos: &GamePosition, loc: Location, playerid: PlayerId) -> Option<usize> {
            if (pos.player == playerid || pos.loc == Shop || pos.loc == Explorer) && pos.loc == loc
            {
                Some(pos.index)
            } else {
                None
            }
        }
        if let Some(i) = get_index(&game.position, loc, playerid) {
            self.selection.push(i)
        }
        for select in &game.selection {
            if let Some(i) = get_index(select, loc, playerid) {
                self.selection.push(i)
            }
        }
        self
    }
}

impl DeckWidget<'_, '_> {
    /// Renders a Deck
    ///
    /// If this Deck necessitate a popup it will return [true]
    /// else it returns [fals€]
    pub fn render(mut self, mut area: Rect, buf: &mut Buffer) -> bool
    where
        Self: Sized,
    {
        let mut need_popup = false;
        if self.hidden && !self.selection.is_empty() {
            area = buf.area;
            self.hidden = false;
            Clear.render(area, buf);
            need_popup = true;
        }
        Block::bordered()
            .title_top(Line::from(self.name).left_aligned())
            .border_type(if !self.selection.is_empty() {
                BorderType::Plain
            } else {
                BorderType::LightDoubleDashed
            })
            .border_style(if !self.selection.is_empty() {
                Style::new().blue()
            } else {
                Style::new()
            })
            .render(area, buf);
        let area = area.inner(Margin::new(1, 1));
        if self.hidden {
            Paragraph::new(format!("{}", self.deck.get_stats()))
                .centered()
                .render(area, buf);
            return false;
        }

        let layout: Vec<Rect> = if let Some(max_cols) = self.max_cols {
            let col_constraints = (0..max_cols).map(|_| Constraint::Fill(1));
            let row_constraints =
                (0..(self.deck.len() / max_cols + 1)).map(|_| Constraint::Fill(1));
            let horizontal = Layout::horizontal(col_constraints).spacing(1);
            let vertical = Layout::vertical(row_constraints).spacing(1);

            let rows = vertical.split(area);
            rows.iter()
                .flat_map(|&row| horizontal.split(row).to_vec())
                .collect()
        } else {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Fill(1); self.deck.len()])
                .split(area)
                .into_iter()
                .map(|r| r.clone())
                .collect()
        };

        for (i, card) in (0..layout.len()).zip(self.deck.iter()) {
            card.widget()
                .set_selection(self.selection.contains(&i))
                .render(layout[i], buf);
        }
        need_popup
    }
}
