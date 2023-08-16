use crate::game::Game;
use crate::console_io::prompt_map;


pub struct Human;

impl super::Player for Human {
    fn get_move(&self, game: &Game) -> (usize, usize) {
        fn parse_move(input: &str) -> Result<(usize, usize), String> {
            let mut rowcol = input.split(",").map(|s| s.trim());
            let row = rowcol.next()
                .and_then(|s| s.parse::<usize>().ok())
                .ok_or("Could not parse row")?;
            let col = rowcol.next()
                .and_then(|s| s.parse::<usize>().ok())
                .ok_or("Could not parse column")?;
            Ok((row, col))

        }
        prompt_map("Enter your move: ", |s| {
            parse_move(s).and_then(|(row, col)| {
                if game.is_valid_move(row, col){
                    Ok((row, col))
                } else {
                    Err("Cell is already occupied".to_string())
                }
            })
        })
    }
}