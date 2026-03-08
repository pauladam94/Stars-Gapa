use ratatui::crossterm::event::Event;

pub enum Input {
    Enter,
    Space,
    Left,
    Right,
    Up,
    Down,
    Other,
}
impl Input {
    pub fn from(event: Event) -> Self {
        use Input::*;
        if let Event::Key(key_event) = event {
            use ratatui::crossterm::event::KeyCode;
            match key_event.code {
                KeyCode::Char(' ') => Space,
                KeyCode::Enter => Enter,
                KeyCode::Left | KeyCode::Char('h') => Left,
                KeyCode::Right | KeyCode::Char('l') => Right,
                KeyCode::Up | KeyCode::Char('k') => Up,
                KeyCode::Down | KeyCode::Char('j') => Down,
                _ => Other,
            }
        } else {
            Other
        }
    }
    pub fn random() -> Self {
        use Input::*;
        match rand::random_range(0..6) {
            0 => Enter,
            1 => Space,
            2 => Left,
            3 => Right,
            4 => Up,
            5 => Down,
            _ => unreachable!(),
        }
    }
}
