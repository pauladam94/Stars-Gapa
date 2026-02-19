use ratatui::crossterm::event::{poll, read};
use stars_gapa::game::Game;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new();

    ratatui::run(|terminal| {
        loop {
            if poll(Duration::from_millis(100))? {
                game.interact(read()?)
            }
            terminal.draw(|frame| game.view(frame))?;
        }
    })
}
