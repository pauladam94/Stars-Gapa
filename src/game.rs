use crate::action::ActionLoc;
use crate::card::Card;
use crate::deck::Deck;
use crate::event::Event;
use crate::input::Input;
use crate::player::Player;
use crate::player_id::PlayerId;
use crate::selection::GamePosition;
use crate::selection::Location;
use crate::state::State;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Margin;
use ratatui::prelude::Buffer;
use ratatui::prelude::Rect;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;
use std::fmt::Display;

#[derive(Debug)]
pub struct Game {
    pub players: [Player; 2],
    pub deck: Deck,
    pub explorer: Deck,
    pub shop: Deck,
    pub position: GamePosition,
    pub selection: Vec<GamePosition>,
    pub state: State,
    pub current_player: PlayerId,
}
impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::starter_complete_deck();
        let mut shop = Deck::EMPTY;
        let mut explorer = Deck::EMPTY;
        explorer.push(Card::explorer());
        // Here we suppose that the deck have at least 5 elements
        for _ in 0..5 {
            shop.push(deck.remove_random().unwrap());
        }
        let mut players = [Player::default(), Player::default()];
        players[0].draw_hand();
        Self {
            players,
            deck,
            explorer,
            shop,
            position: GamePosition::default(),
            current_player: PlayerId::First,
            state: State::START_GAME,
            selection: vec![],
        }
    }
    /// Pass to the next turn in a [Game]
    /// - ending the turn of a player
    ///   - money set to 0
    ///   - discard played card (only ships)
    ///   - discard hand
    ///   - checks if the next player has to discard
    /// - beginning the turn of the next one
    ///    - draw his hand
    pub fn next_turn(&mut self) {
        let [fst, snd] = &mut self.players;
        let [current, opponent] = match self.current_player {
            PlayerId::First => [fst, snd],
            PlayerId::Second => [snd, fst],
        };

        // Set current player money to 0
        current.gold = 0;

        // Discard its played cards unless it is the Base and Outpost
        //
        // This loop has to be in reverse for the indices to be stable
        // even when removing element from the played cards
        for i in (0..current.played.len()).rev() {
            if current.played[i].is_ship() {
                current.discard.push(current.played.remove(i))
            }
        }
        // Discard its hand cards
        for _ in 0..current.hand.len() {
            current.discard.push(current.hand.remove_last())
        }

        // Change the State if opponent has to discard
        if current.opponent_discard != 0 {
            self.state = State::Discarding {
                nb: current.opponent_discard,
                loc: ActionLoc::Hand,
            };
        } else {
            self.state = State::Playing;
        }

        // Make the new current_player draw his hand
        opponent.draw_hand();

        // Change the current player
        self.current_player = self.current_player.other();
    }

    /// Get the event from doing an action
    /// thas usually comes from pressing the enter button.
    pub fn do_action(&mut self) -> Option<Event> {
        use Event::*;
        use State::*;
        let attack = self.players[self.current_player].attack;
        match self.state {
            Playing => {
                if attack == 0 {
                    Some(NextTurn)
                } else {
                    Some(Attack)
                }
            }
            Scraping { .. } | Discarding { .. } | Copy => Some(Choose(self.selection.clone())),
            WonBy(_) => None,
        }
    }

    /// Get the event from doing a selection
    /// that usually comes from pressing the space bar.
    pub fn do_selection(&mut self) -> Option<Event> {
        use Location::*;
        use State::*;
        match self.state {
            Copy | Discarding { .. } | Scraping { .. } => {
                if let Some(index) = self.selection.iter().position(|pos| pos == &self.position) {
                    self.selection.remove(index);
                } else {
                    self.selection.push(self.position.clone());
                }
                return None;
            }
            _ => (),
        }
        match self.position.loc {
            Explorer => Some(Event::BuyExplorer),
            Shop => Some(Event::Buy(self.position.index)),
            Hand => Some(Event::Play(self.position.index)),
            Played => Some(Event::Activate(self.position.index)),
            Discard => None,
            DrawPile => None,
        }
    }
    /// Given the [Game] state
    /// returns an [Event] from an [Input]
    pub fn get_event(&mut self, input: Input) -> Option<Event> {
        use Input::*;
        let event = match input {
            Enter => self.do_action(),
            Space => self.do_selection(),
            Left => {
                if self.position.index == 0 {
                    self.position.next_left();
                } else {
                    self.position.index -= 1;
                }
                None
            }
            Right => {
                if self.position.index + 1 >= self[&self.position].len() {
                    self.position.next_right();
                } else {
                    self.position.index += 1;
                }
                None
            }
            Up => {
                self.position.next_up(&self.current_player);
                None
            }
            Down => {
                self.position.next_down(&self.current_player);
                None
            }
            Other => None,
        };
        if self.position.player == self.current_player.other()
            && self.position.loc == Location::Hand
        {
            self.position.loc = Location::Played;
            self.position.index = 0;
        }
        event
    }
    pub fn interact(&mut self, input: Input) {
        if let Some(event) = self.get_event(input) {
            self.apply_event(event);
        }
    }
    pub fn event_is_valid(&self, event: &Event) -> bool {
        use Event::*;
        use State::*;
        match (&self.state, event) {
            (Playing, Buy(_)) => true,
            (Playing, BuyExplorer) => true,
            (Playing, Play(_)) => true,
            (Playing, Activate(_)) => true,
            (Playing, Attack) => true,
            (Playing, NextTurn) => true,
            (Discarding { .. }, Choose(_)) => true,
            (Copy, Choose(_)) => true,
            _ => false,
        }
    }
    pub fn apply_event(&mut self, event: Event) {
        // Verify first that the event is valid given the context of the [Game]
        if !self.event_is_valid(&event) {
            return;
        }
        use Event::*;
        use PlayerId::*;
        use State::*;
        match event {
            Buy(index) => {
                let _ = self.players[self.current_player].buy_from_shop(
                    &mut self.deck,
                    &mut self.shop,
                    index,
                );
            }
            BuyExplorer => {
                let _ = self.players[self.current_player].buy_card(Card::explorer());
            }
            Play(index) => {
                let player = &mut self.players[self.position.player];
                if !player.hand.is_empty() && (0..player.hand.len()).contains(&index) {
                    player.play_card(index, &mut self.state);

                    if index == player.hand.len() && !player.hand.is_empty() {
                        self.position.index = player.hand.len() - 1;
                    }
                } else {
                    self.position.index = 0;
                }
            }
            Choose(positions) => match &self.state {
                // todo finish this
                Discarding { nb, loc } => {
                    if positions.len() as u32 > *nb {
                        return;
                    }
                    // todo do the discarding of cards
                    self.selection.clear();
                    self.state = State::Playing;
                }
                Scraping { nb, loc } => {
                    // Check that the right number of scraps has been done
                    if positions.len() as u32 > *nb {
                        return;
                    }
                    // Check that the discard has been done at the right place
                    for pos in &positions {
                        if !loc.contains(&pos.loc) {
                            return;
                        }
                    }
                    // todo do the scraping of cards
                    self.selection.clear();
                    self.state = State::Playing;
                }
                Copy => {
                    if positions.len() != 1 {
                        return;
                    }
                }
                _ => (),
            },
            Attack => {
                let [fst, snd] = &mut self.players;
                let [player, opponent] = match self.current_player {
                    First => [fst, snd],
                    Second => [snd, fst],
                };

                let attack = player.attack;
                if opponent.authority < attack {
                    opponent.authority = 0;
                    self.state = State::WonBy(self.current_player)
                } else {
                    opponent.authority -= attack;
                }
                player.attack = 0;
            }
            NextTurn => self.next_turn(),
            Activate(index) => {
                // todo activate a card
                let player = &mut self.players[self.current_player];
                player.activate_played_card(index, &mut self.state);
            }
        }
    }
    pub fn status_line(&self) -> impl Display {
        StatusLine(self)
    }
}

pub struct StatusLine<'a>(&'a Game);
impl Display for StatusLine<'_> {
    // todo: better adapt on the state of the game and precise
    // position of cursor
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Location::*;
        use State::*;
        let Game {
            players,
            deck,
            explorer,
            shop,
            position,
            state,
            current_player,
            selection,
        } = self.0;

        if let State::WonBy(playerid) = state {
            write!(f, "\t>>>>\t{} Player WON\t<<<<\t", playerid)?;
            return Ok(());
        }

        write!(f, "[->][↑][<-][↓] move")?;
        write!(f, " | ")?;
        write!(f, "[enter] ")?;
        let width_enter = 20;
        match state {
            Playing => {
                if players[current_player].attack == 0 {
                    write!(f, "{:^width$}", "Next Turn", width = width_enter)?;
                } else {
                    write!(f, "{:^width$}", "Attack Opponent", width = width_enter)?;
                }
            }
            Scraping { nb, loc } => write!(
                f,
                "{:^width$}",
                format!("Scrap {} cards selectioned", selection.len()),
                width = width_enter,
            )?,
            Discarding { nb, loc } => write!(
                f,
                "{:^width$}",
                format!("Discard {} cards selectioned", selection.len()),
                width = width_enter
            )?,
            Copy => write!(
                f,
                "{:^width$}",
                "Copy Ship selectioned",
                width = width_enter
            )?,
            _ => (),
        }
        write!(f, " | ")?;
        write!(f, "[space] ")?;

        match state {
            Playing => match position.loc {
                Explorer => write!(f, "Buy an Explorer")?,
                Shop => write!(f, "Buy a card from the shop")?,
                Hand => write!(f, "Play a card from your hand")?,
                Played => write!(f, "Activate this card")?,
                Discard => write!(f, "")?,
                DrawPile => write!(f, "")?,
            },
            Scraping { nb, loc } => write!(f, "Scrapping {} cards at {}", nb, loc)?,
            Discarding { nb, loc } => write!(f, "Discarding {} cards at {}", nb, loc)?,
            Copy => write!(f, "")?,
            _ => (),
        }

        Ok(())
    }
}

/* Global Layout of the Game

----------------------------------------|-----------|
| Played hand opponent Player | discard | draw_pile |
|----------|------------------|---------|-----------|
| Explorer |       Shop       |    info players     |
|----------|------------------|---------------------|
|  Played Hand current player |     discard         |
|-----------------------------|---------------------|
|    Hand current Player      |      draw pile      |
|-----------------------------|---------------------|

Rmk: the left colum is about 5 times bigger than the right column
*/

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        use Constraint::*;
        Block::bordered().render(area, buf);
        let area = area.inner(Margin::new(1, 1));

        // Layout the Grid
        let horizontal = Layout::horizontal([Fill(5), Fill(1)]);
        let vertical = Layout::vertical([Fill(1), Fill(1), Fill(1), Fill(1)]);
        let status_line_layout = Layout::vertical([Fill(1), Length(1)]);
        let shop_layout = Layout::horizontal([Fill(1), Fill(5)]);
        let discard_draw_pile_layout = Layout::vertical([Fill(1), Fill(1)]);
        let info_players_layout = Layout::vertical([Fill(1), Fill(1)]);

        let layout = status_line_layout.split(area);
        let (all_elements, status_line) = (layout[0], layout[1]);

        let layout = horizontal.split(all_elements);
        let (left, right) = (layout[0], layout[1]);

        let layout = vertical.split(left);
        let (played_op, explorer_shop, played_cur, hand_cur) =
            (layout[0], layout[1], layout[2], layout[3]);

        let layout = vertical.split(right);
        let (discard_draw_pile_op, info_players, discard_cur, draw_pile_cur) =
            (layout[0], layout[1], layout[2], layout[3]);

        let layout = shop_layout.split(explorer_shop);
        let (explorer, shop) = (layout[0], layout[1]);

        let layout = discard_draw_pile_layout.split(discard_draw_pile_op);
        let (draw_pile_op, discard_op) = (layout[0], layout[1]);

        let layout = info_players_layout.split(info_players);
        let info_players = [layout[1], layout[0]];

        let played_players = [played_cur, played_op];
        let hand_players = [hand_cur, Rect::new(0, 0, 0, 0)];
        let discard_players = [discard_cur, discard_op];
        let draw_pile_players = [draw_pile_cur, draw_pile_op];
        // Layout the Grid

        // Shop
        self.explorer
            .widget()
            .set_name("Explorer")
            .set_selection(&self, Location::Explorer, PlayerId::First)
            .render(explorer, buf);
        self.shop
            .widget()
            .set_name("Shop")
            .set_selection(&self, Location::Shop, PlayerId::First)
            .render(shop, buf);

        // The two players
        for id in 0..2 {
            // 0 is at the bottom is the current player
            // 1 is at the top
            let played = played_players[id];
            let hand = hand_players[id];
            let discard = discard_players[id];
            let draw_pile = draw_pile_players[id];
            let info = info_players[id];
            let id = if id == 0 {
                self.current_player
            } else {
                self.current_player.other()
            };
            // the current player
            let player = &self.players[id];

            // Played Hand of the player
            player
                .played
                .widget()
                .set_name(&format!("Played {} player", id))
                .set_selection(self, Location::Played, id)
                .render(played, buf);
            // Hand of the player
            player
                .hand
                .widget()
                .set_name(&format!("Hand {} player", id))
                .set_selection(self, Location::Hand, id)
                .render(hand, buf);

            // Draw info about current player
            Paragraph::new(format!("{}", player))
                .centered()
                .render(info, buf);
            // Draw pile of the current player
            if player
                .draw_pile
                .widget()
                .set_name(&format!("Draw Pile {} player", id))
                .set_max_cols(6)
                .set_selection(self, Location::DrawPile, id)
                .hidden()
                .render(draw_pile, buf)
            {
                return;
            }
            // Discard of the current player
            if player
                .discard
                .widget()
                .set_name(&format!("Discard {} player", id))
                .set_max_cols(6)
                .set_selection(self, Location::Discard, id)
                .hidden()
                .render(discard, buf)
            {
                return;
            }
        }

        // Status Line
        Paragraph::new(format!("{}", self.status_line())).render(status_line, buf);
    }
}
