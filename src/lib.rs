pub mod action;

pub mod card;
pub mod deck;
pub mod faction;
pub mod game;
pub mod player;
pub mod player_id;

pub mod selection;

pub fn add_modulo(lhs: &mut u32, rhs: u32, modulo: u32) {
    if *lhs + rhs >= modulo {
        *lhs = modulo - rhs
    } else {
        *lhs += rhs
    }
}
pub fn sub_modulo(lhs: &mut u32, rhs: u32, modulo: u32) {
    if *lhs + rhs >= modulo {
        *lhs = modulo - rhs
    } else {
        *lhs += rhs
    }
}
