use std::{fmt::Display, slice::Iter};

#[derive(Debug)]
pub struct Factions(Vec<Faction>);

impl Factions {
    pub const NONE: Self = Self(vec![]);
}
impl Display for Factions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for faction in self.0.iter() {
            write!(f, "{}", faction)?;
        }
        Ok(())
    }
}

impl Factions {
    pub fn iter(&self) -> Iter<'_, Faction> {
        self.0.iter()
    }
}
impl Faction {
    pub const fn to_usize(&self) -> usize {
        match self {
            Faction::Blob => 0,
            Faction::Trade => 1,
            Faction::Star => 2,
            Faction::Machine => 3,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Faction {
    Blob,
    Trade,
    Star,
    Machine,
}

impl Display for Faction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Faction::Blob => write!(f, "🐉"),
            Faction::Trade => write!(f, "🏦"),
            Faction::Star => write!(f, "🌟"),
            Faction::Machine => write!(f, "⚙️"),
        }
    }
}
