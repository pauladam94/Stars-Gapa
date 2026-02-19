use std::fmt::Display;

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
            Faction::Blob => write!(f, "B"),
            Faction::Trade => write!(f, "T"),
            Faction::Star => write!(f, "S"),
            Faction::Machine => write!(f, "M"),
        }
    }
}
