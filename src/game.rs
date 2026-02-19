use crate::cards::Deck;
use crate::player::Player;
use crate::selection::DeckKind::Hand;
use crate::selection::PlayerId;
use crate::selection::Selection;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyEvent;
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

#[derive(Debug)]
pub struct Game {
    pub(crate) player1: Player,
    pub(crate) player2: Player,
    deck: Deck,
    shop: Deck,
    selection: Selection,
    current_player: PlayerId,
}
impl Game {
    pub fn new() -> Self {
        Self {
            player1: Player::default(),
            player2: Player::default(),
            deck: Deck::starter_shop(),
            shop: todo!(),
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
        use Selection::*;
        use ratatui::crossterm::event::KeyCode::*;
        match (event.code, &mut self.selection) {
            (Enter | Char(' '), Shop { index }) => (),
            (Enter | Char(' '), ShopExplorer) => (),
            (Enter | Char(' '), ActionButton) => (),
            (
                Enter | Char(' '),
                Deck {
                    index,
                    player,
                    kind,
                },
            ) => (),
            (Left, Shop { index }) => (),
            (Left, ShopExplorer) => (),
            (Left, ActionButton) => (),
            (
                Left,
                Deck {
                    index,
                    player,
                    kind,
                },
            ) => (),
            (Right, Shop { index }) => (),
            (Right, ShopExplorer) => (),
            (Right, ActionButton) => (),
            (
                Right,
                Deck {
                    index,
                    player,
                    kind,
                },
            ) => (),
            (Up, Shop { index }) => (),
            (Up, ShopExplorer) => (),
            (Up, ActionButton) => (),
            (
                Up,
                Deck {
                    index,
                    player,
                    kind,
                },
            ) => (),
            (Down, Shop { index }) => (),
            (Down, ShopExplorer) => (),
            (Down, ActionButton) => (),
            (
                Down,
                Deck {
                    index,
                    player,
                    kind,
                },
            ) => (),
            (_, _) => (),
        }
        // match event.code {
        //     Enter | Char(' ') => todo!(),
        //     Left => match &mut self.selection {
        //         Shop { index } => *index -= 1,
        //         ShopExplorer => (),
        //         ActionButton => {
        //             self.selection = Deck {
        //                 index: 0,
        //                 player: self.current_player,
        //                 kind: Hand,
        //             }
        //         }
        //         Deck {
        //             index,
        //             player,
        //             kind,
        //         } => todo!(),
        //     },
        //     Right => todo!(),
        //     Up => todo!(),
        //     Down => todo!(),
        //     _ => (),
        // }
    }
}
impl Widget for Game {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
impl Game {
    pub fn view(&self, frame: &mut Frame) {
        let block = Block::bordered().title("Welcome");
        let greeting = Paragraph::new("Hello, Ratatui! 🐭")
            .centered()
            .yellow()
            .block(block);

        frame.render_widget(greeting, frame.area());
    }
}
