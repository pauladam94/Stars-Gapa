use crate::{action::ActionLoc, player_id::PlayerId, selection::Location};

#[derive(Debug)]
pub enum State {
    Playing,
    Scraping { nb: u32, loc: ActionLoc },
    Discarding { nb: u32, loc: ActionLoc },
    // TODO: do something with this state
    WonBy(PlayerId),
    Copy,
}

impl State {
    pub const START_GAME: Self = Self::Playing;
}
