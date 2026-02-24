use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, poll, read};
use ratatui::widgets::Widget;
use stars_gapa::game::Game;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new();

    ratatui::run(|terminal| {
        terminal.draw(|frame| (&game).render(frame.area(), frame.buffer_mut()))?;
        loop {
            if poll(Duration::from_millis(100))? {
                let event = read()?;
                if let Event::Key(KeyEvent {
                    code: KeyCode::Esc | KeyCode::Char('q'),
                    ..
                }) = event
                {
                    break;
                }
                game.interact(event);
                terminal.draw(|frame| (&game).render(frame.area(), frame.buffer_mut()))?;
            }
        }
        Ok(())
    })
}
