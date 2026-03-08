use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerId {
    First,
    Second,
}

impl PlayerId {
    pub const fn new(id: usize) -> Self {
        if id == 0 {
            Self::First
        } else if id == 1 {
            Self::Second
        } else {
            panic!("Only 0 and 1 can be converted to PlayerId")
        }
    }
    pub fn random() -> Self {
        match rand::random_range(0..2) {
            0 => Self::First,
            1 => Self::Second,
            _ => unreachable!(),
        }
    }
}

impl<T> std::ops::Index<PlayerId> for [T; 2] {
    type Output = T;
    fn index(&self, index: PlayerId) -> &Self::Output {
        &self[index.to_usize()]
    }
}
impl<T> std::ops::IndexMut<PlayerId> for [T; 2] {
    fn index_mut(&mut self, index: PlayerId) -> &mut Self::Output {
        &mut self[index.to_usize()]
    }
}
impl<T> std::ops::Index<&PlayerId> for [T; 2] {
    type Output = T;
    fn index(&self, index: &PlayerId) -> &Self::Output {
        &self[index.to_usize()]
    }
}
impl<T> std::ops::IndexMut<&PlayerId> for [T; 2] {
    fn index_mut(&mut self, index: &PlayerId) -> &mut Self::Output {
        &mut self[index.to_usize()]
    }
}
impl<T> std::ops::Index<&mut PlayerId> for [T; 2] {
    type Output = T;
    fn index(&self, index: &mut PlayerId) -> &Self::Output {
        &self[index.to_usize()]
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerId::First => write!(f, "1st"),
            PlayerId::Second => write!(f, "2nd"),
        }
    }
}

impl PlayerId {
    pub const fn to_usize(&self) -> usize {
        match self {
            PlayerId::First => 0,
            PlayerId::Second => 1,
        }
    }
    pub const fn other(&self) -> Self {
        match self {
            PlayerId::First => Self::Second,
            PlayerId::Second => Self::First,
        }
    }
}
