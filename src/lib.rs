pub mod action;

pub mod card;
pub mod deck;
pub mod faction;
pub mod game;
pub mod player;
pub mod player_id;

pub mod selection;

// pub fn add_modulo(lhs: &mut usize, rhs: usize, modulo: usize) {
//     if *lhs + rhs >= modulo {
//         *lhs = modulo - rhs
//     } else {
//         *lhs += rhs
//     }
// }
// pub fn sub_modulo(lhs: &mut usize, rhs: usize, modulo: usize) {
//     if &rhs > lhs {
//         *lhs = modulo - (rhs - *lhs);
//     } else {
//         *lhs -= rhs
//     }
// }
