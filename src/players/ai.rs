use rand::seq::SliceRandom;

use crate::game::{Game, GameState};


pub struct AI;

impl super::Player for AI {
    fn get_move(&self, game: &Game) -> (usize, usize) {
        let mut moves = (0..3).flat_map(|x| (0..3).map(move |y| (x, y)))
            .filter(|&(x, y)| game.is_valid_move(x, y))
            .map(|(x, y)| (x, y, score_move((x, y), game)))
            .collect::<Vec<_>>();

        moves.shuffle(&mut rand::thread_rng());
        moves.sort_by_key(|&(_, _, score)| score);
        let (x, y, score) = moves.last().unwrap();
        println!("AI chooses move {x}, {y} with score {score}");
        (*x, *y)
    }
}

fn score_move((x, y): (usize, usize), game: &Game) -> i32 {
    let game = game.apply_move(x, y);
    match game.get_game_state() {
        GameState::Win(m) if m != game.get_mark() => 1,
        GameState::Win(_) => -1,
        GameState::Draw => 0,
        GameState::InProgress => {
            let scores = (0..3).flat_map(|x| (0..3).map(move |y| (x, y)))
                .filter(|&(x, y)| game.is_valid_move(x, y))
                .map(|(x, y)| score_move((x, y), &game));
            -scores.max().unwrap_or(0)
        }
    } 
}