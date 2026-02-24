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
        use ratatui::crossterm::event::KeyCode::*;
        match (event.code, &mut self.selection) {
            (Enter | Char(' '), Shop { index }) => {
                self.players[self.current_player].buy_from_shop(
                    &mut self.deck,
                    &mut self.shop,
                    *index,
                );
            }
            (Enter | Char(' '), ShopExplorer) => {
                self.players[self.current_player].buy_card(Card::explorer());
            }
            (Enter | Char(' '), ActionButton) => (),
            (
                Enter | Char(' '),
                Deck {
                    index,
                    player,
                    kind,
                },
            ) => match kind {
                Discard => todo!(),
                Played => todo!(),
                Hand => self.players[*player].play_card(*index),
                DrawPile => todo!(),
                CurrentCard => todo!(),
            },
            (Left, Shop { index }) => {
                if index == &0 {
                    self.selection = ShopExplorer
                } else {
                    *index -= 1
                }
            }
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
                    index,
                    player,
                    kind,
                },
            ) => *index -= 1,
            (Right, Shop { index }) => *index += 1,
            (Right, ShopExplorer) => self.selection = Shop { index: 0 },
            (Right, ActionButton) => (), // Nothing to do
            (
                Right,
                Deck {
                    index,
                    player,
                    kind,
                },
            ) => *index += 1,
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
                    self.selection = Shop { index: 0 }
                } else {
                    ()
                }
            }
            (Down, Shop { .. }) => {
                self.selection = Deck {
                    player: self.current_player,
                    kind: Location::Hand,
                    index: 0,
                }
            }
            (Down, ShopExplorer) => {
                self.selection = Deck {
                    player: self.current_player,
                    kind: Location::Hand,
                    index: 0,
                }
            }
            (Down, ActionButton) => (),
            (
                Down,
                Deck {
                    player,
                    kind,
                    index,
                },
            ) => {
                if player == &self.current_player {
                } else {
                    self.selection = ShopExplorer;
                }
            }
            (_, _) => (),
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
                    .constraints([Fill(1), Fill(1), Fill(1), Fill(1)])
                    .split(right);
                let (info, discard, draw_pile, action_button) =
                    (layout[0], layout[1], layout[2], layout[3]);

                // Draw info about player
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
                Card::EMPTY_CARD
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
