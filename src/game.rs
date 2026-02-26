use crate::card::Card;
use crate::deck::Deck;
use crate::player::Player;
use crate::player_id::PlayerId;
use crate::selection::Location;
use crate::selection::Selection;
use crate::selection::SelectionInfo;
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
    deck: Deck,
    shop: Deck,
    selection: Selection,
    current_player: PlayerId,
}
impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::starter_complete_deck();
        let mut shop = Deck::EMPTY;
        // Here we suppose that the deck have at least 5 elements
        for _ in 0..5 {
            shop.push(deck.remove_random().unwrap());
        }
        let mut players = [Player::default(), Player::default()];
        players[0].draw_hand();
        Self {
            players,
            deck,
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
        use Location::*;
        use Selection::*;
        // use ratatui::crossterm::event::KeyCode::*;

        enum KeyCode {
            EnterOrSpace,
            Left,
            Right,
            Up,
            Down,
        }
        use KeyCode::*;
        let key_code = match event.code {
            ratatui::crossterm::event::KeyCode::Char(' ')
            | ratatui::crossterm::event::KeyCode::Enter => EnterOrSpace,
            ratatui::crossterm::event::KeyCode::Left => Left,
            ratatui::crossterm::event::KeyCode::Right => Right,
            ratatui::crossterm::event::KeyCode::Up => Up,
            ratatui::crossterm::event::KeyCode::Down => Down,
            _ => {
                return;
            }
        };

        match (key_code, &mut self.selection) {
            (EnterOrSpace, Shop { index }) => {
                self.players[self.current_player].buy_from_shop(
                    &mut self.deck,
                    &mut self.shop,
                    *index,
                );
            }
            (EnterOrSpace, ShopExplorer) => {
                self.players[self.current_player].buy_card(Card::explorer());
            }
            (EnterOrSpace, ActionButton) => {
                let attack = self.players[self.current_player].get_attack();

                if attack == 0 {
                    self.next_turn();
                } else {
                    self.players[self.current_player.other()].authority -= attack;
                }
            }
            (
                EnterOrSpace,
                Deck {
                    index,
                    player,
                    kind: Hand,
                },
            ) => {
                self.players[*player].play_card(*index);
                if self.players[player].hand.is_empty() {
                    self.selection = ActionButton;
                }
            }
            (
                EnterOrSpace,
                Deck {
                    index,
                    player,
                    kind,
                },
            ) => (),
            (Left, Shop { index: 0 }) => self.selection = ShopExplorer,
            (Left, Shop { index }) => *index -= 1,
            (Left, ShopExplorer) => (), // Nothing to do
            (Left, ActionButton) => {
                if !self.players[self.current_player.to_usize()].hand.is_empty() {
                    self.selection = Deck {
                        index: self.players[self.current_player.to_usize()].hand.len() - 1,
                        player: self.current_player,
                        kind: Location::Hand,
                    }
                }
            }
            (
                Left,
                Deck {
                    index: 0,
                    player,
                    kind,
                },
            ) => (),
            (
                Left,
                Deck {
                    index,
                    player,
                    kind,
                },
            ) => *index -= 1,
            (Right, Shop { index }) => {
                if *index != self.shop.len() - 1 {
                    *index += 1;
                }
            }
            (Right, ShopExplorer) => self.selection = Shop { index: 0 },
            (Right, ActionButton) => (), // Nothing to do
            (
                Right,
                Deck {
                    index,
                    player: PlayerId::First,
                    kind: Location::Hand,
                },
            ) => {
                if *index + 1 == self.players[0].hand.len() {
                    self.selection = ActionButton;
                } else {
                    *index += 1;
                }
            }
            (
                Right,
                Deck {
                    player,
                    kind,
                    index,
                },
            ) => {
                if *index + 1 != self.players[player][kind].len() {
                    *index += 1;
                }
            }
            (Up, Shop { .. }) => {
                self.selection = Deck {
                    player: self.current_player.other(),
                    kind: Location::Hand,
                    index: 0,
                }
            }
            (Up, ShopExplorer) => {
                self.selection = Deck {
                    player: self.current_player.other(),
                    kind: Location::Hand,
                    index: 0,
                }
            }
            (Up, ActionButton) => self.selection = Shop { index: 0 },
            (
                Up,
                Deck {
                    player,
                    kind,
                    index,
                },
            ) => {
                if player == &self.current_player {
                    use Location::*;
                    match kind {
                        DiscardOrHand | Played => self.selection = Shop { index: 0 },
                        Hand => {
                            *kind = Played;
                            *index = 0;
                        }
                        DrawPile => {
                            *index = 0;
                            *kind = DiscardOrHand;
                        }
                        CurrentCard => (),
                    }
                } else {
                    ()
                }
            }
            (Down, Shop { .. } | ShopExplorer) => {
                if !self.players[self.current_player].played.is_empty() {
                    self.selection = Deck {
                        player: self.current_player,
                        kind: Location::Played,
                        index: 0,
                    }
                } else if !self.players[self.current_player].hand.is_empty() {
                    self.selection = Deck {
                        player: self.current_player,
                        kind: Location::Hand,
                        index: 0,
                    }
                } else {
                    self.selection = ActionButton;
                }
            }
            (Down, ActionButton) => (),
            (
                Down,
                Deck {
                    player: PlayerId::First,
                    kind: Location::Played,
                    ..
                },
            ) => {
                if !self.players[self.current_player].hand.is_empty() {
                    self.selection = Deck {
                        player: PlayerId::First,
                        kind: Location::Hand,
                        index: 0,
                    }
                } else {
                    self.selection = ActionButton
                }
            }
            (
                Down,
                Deck {
                    player: PlayerId::First,
                    kind: Location::Hand,
                    ..
                },
            ) => self.selection = ActionButton,
            (
                Down,
                Deck {
                    player: PlayerId::First,
                    ..
                },
            ) => self.selection = ActionButton,
            (
                Down,
                Deck {
                    player: PlayerId::Second,
                    kind,
                    index,
                },
            ) => self.selection = ShopExplorer,
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

        let SelectionInfo {
            selection_players,
            selection_explorer,
            selection_shop,
            selection_action_button,
        } = self.selection.get();

        // Shop
        {
            let shop_layout = Layout::default()
                .direction(Horizontal)
                .constraints([Fill(1), Fill(1), Fill(5)])
                .split(shop);
            let (explorer, shop) = (shop_layout[0], shop_layout[2]);
            Card::explorer()
                .widget()
                .set_selection(selection_explorer)
                .render(explorer, buf);
            self.shop
                .widget()
                .set_name("Shop")
                .set_selection(selection_shop)
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
                    .set_selection(None)
                    .render(played, buf);
                // Hand of the player
                let selection = selection_players[self.current_player];
                player
                    .hand
                    .widget()
                    .set_name("Hand")
                    .set_selection(selection)
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
                    .set_selection(None)
                    .hidden()
                    .render(draw_pile, buf);
                // Discard of the current player
                player
                    .discard
                    .widget()
                    .set_name("Discard")
                    .set_selection(None)
                    .hidden()
                    .render(discard, buf);
                // Action Button
                Card::EMPTY_SHIP
                    .with_name("Action Button")
                    .widget()
                    .set_selection(selection_action_button)
                    .render(action_button, buf);
            }
        }

        // Status Line
        Paragraph::new(format!("{}", self.selection)).render(status_line, buf);
    }
}
