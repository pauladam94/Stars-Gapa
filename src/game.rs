use crate::card::Card;
use crate::deck::Deck;
use crate::player::Player;
use crate::player_id::PlayerId;
use crate::selection::Location;
use crate::selection::Selection;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Margin;
use ratatui::prelude::Buffer;
use ratatui::prelude::Rect;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

#[derive(Debug)]
pub struct Game {
    pub players: [Player; 2],
    pub deck: Deck,
    pub explorer: Deck,
    pub shop: Deck,
    selection: Selection,
    current_player: PlayerId,
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
            shop, // todo pick cards from deck
            selection: Selection::default(),
            current_player: PlayerId::First,
        }
    }
    pub fn next_turn(&mut self) {
        // End the turn of the current player
        let cur = &mut self.players[self.current_player];
        for _ in 0..cur.played.len() {
            cur.discard.push(cur.played.remove_last())
        }
        for _ in 0..cur.hand.len() {
            cur.discard.push(cur.hand.remove_last())
        }

        // Change the player
        self.current_player = self.current_player.other();

        // Make the new current_player draw his hand
        self.players[self.current_player].draw_hand();
    }
}

impl Game {
    pub fn interact(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Mouse(_)
            | Event::Paste(_)
            | Event::Resize(_, _)
            | Event::FocusGained
            | Event::FocusLost => (),
        }
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        enum KeyCode {
            Enter,
            Space,
            Left,
            Right,
            Up,
            Down,
        }
        use KeyCode::*;
        let key_code = match event.code {
            ratatui::crossterm::event::KeyCode::Char(' ') => Space,
            ratatui::crossterm::event::KeyCode::Enter => Enter,
            ratatui::crossterm::event::KeyCode::Left => Left,
            ratatui::crossterm::event::KeyCode::Right => Right,
            ratatui::crossterm::event::KeyCode::Up => Up,
            ratatui::crossterm::event::KeyCode::Down => Down,
            _ => {
                return;
            }
        };

        match key_code {
            Enter | Space => todo!(),
            Left => {
                if self.selection.index == 0 {
                    self.selection.next_left();
                } else {
                    self.selection.index -= 1;
                }
            }
            Right => {
                if self.selection.index >= self[&self.selection].len() - 1 {
                    self.selection.next_right();
                } else {
                    self.selection.index += 1;
                }
            }
            Up => self.selection.next_up(&self.current_player),
            Down => self.selection.next_down(&self.current_player),
        }
    }
}

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        use Constraint::*;
        use Direction::*;
        Block::bordered().render(area, buf);
        let area = area.inner(Margin::new(1, 1));

        let layout = Layout::default()
            .direction(Vertical)
            .constraints([Fill(1), Fill(1), Fill(4), Length(1)])
            .split(area);

        let (top, shop, bottom, status_line) = (layout[0], layout[1], layout[2], layout[3]);
        let layout_players: [Rect; 2] = match self.current_player {
            PlayerId::First => [bottom, top],
            PlayerId::Second => [top, bottom],
        };

        // Shop
        {
            let shop_layout = Layout::default()
                .direction(Horizontal)
                .constraints([Fill(1), Fill(1), Fill(5)])
                .split(shop);
            let (explorer, shop) = (shop_layout[0], shop_layout[2]);
            self.explorer
                .widget()
                .set_name("Explorer")
                .set_selection(self.selection.explorer())
                .render(explorer, buf);
            self.shop
                .widget()
                .set_name("Shop")
                .set_selection(self.selection.shop())
                .render(shop, buf);
        }
        // Player Not playing right now
        {
            let layout = layout_players[self.current_player.other()];
            let player = &self.players[self.current_player.other()];
            Paragraph::new(format!("{}", player)).render(layout, buf);
        }
        // Current Player
        {
            let layout = layout_players[self.current_player];
            let player = &self.players[self.current_player];

            let layout = Layout::default()
                .direction(Horizontal)
                .constraints([Fill(6), Fill(1)])
                .split(layout);
            let (left, right) = (layout[0], layout[1]);

            // On the Left
            {
                let layout = Layout::default()
                    .direction(Vertical)
                    .constraints([Fill(1), Fill(1)])
                    .split(left);
                let (played, hand) = (layout[0], layout[1]);

                // Played Hand of the player
                player
                    .played
                    .widget()
                    .set_name("Played Cards")
                    .set_selection(self.selection.player(self.current_player, Location::Played))
                    .render(played, buf);
                // Hand of the player
                player
                    .hand
                    .widget()
                    .set_name("Hand")
                    .set_selection(self.selection.player(self.current_player, Location::Hand))
                    .render(hand, buf);
            }

            // On the Right
            {
                let layout = Layout::default()
                    .direction(Vertical)
                    .constraints([Length(3), Fill(1), Fill(1), Length(3)])
                    .split(right);
                let (info, discard, draw_pile, action_button) =
                    (layout[0], layout[1], layout[2], layout[3]);

                // Draw info about current player
                Paragraph::new(format!("{}", player)).render(info, buf);
                // Draw pile of the current player
                player
                    .draw_pile
                    .widget()
                    .set_name("Draw Pile")
                    .set_selection(
                        self.selection
                            .player(self.current_player, Location::DrawPile),
                    )
                    .hidden()
                    .render(draw_pile, buf);
                // Discard of the current player
                player
                    .discard
                    .widget()
                    .set_name("Discard")
                    .set_selection(
                        self.selection
                            .player(self.current_player, Location::Discard),
                    )
                    .hidden()
                    .render(discard, buf);
            }
        }

        // Status Line
        Paragraph::new(format!("{}", self.selection)).render(status_line, buf);
    }
}
