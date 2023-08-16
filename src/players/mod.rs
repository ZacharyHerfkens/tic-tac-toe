mod human;
mod ai;

use crate::game::*;
pub use human::Human;
pub use ai::AI;

pub trait Player {
    fn get_move(&self, game: &Game) -> (usize, usize);
}