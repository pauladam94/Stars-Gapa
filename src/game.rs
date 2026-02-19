use crate::types::Player;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;

#[derive(Debug)]
pub struct Game {
    pub(crate) player1: Player,
    pub(crate) player2: Player,
}

impl Game {
    pub fn new() -> Self {
        todo!()
    }
    pub fn handle(&mut self, event: Event) {
        match event {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Key(key_event) => todo!(),
            Event::Mouse(mouse_event) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        }
    }
    pub fn draw(&mut self, frame: &mut Frame) {
        let block = Block::bordered().title("Welcome");
        let greeting = Paragraph::new("Hello, Ratatui! 🐭")
            .centered()
            .yellow()
            .block(block);

        frame.render_widget(greeting, frame.area());
    }
}
