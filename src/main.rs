use ratatui::crossterm::event::{poll, read};
use stars_gapa::types::modname::modname::Game;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = modname::modname::Game::new();

    ratatui::run(|terminal| {
        loop {
            if poll(Duration::from_millis(100))? {
                game.handle(read()?)
            }
            terminal.draw(|frame| game.draw(frame))?;
        }
    })
}
