
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mark {
    X,
    O
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Draw,
    Win(Mark)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    grid_cells: [[Option<Mark>; 3]; 3],
    mark: Mark
}

impl Game {
    pub fn new(starting_mark: Mark) -> Self {
        Self {
            grid_cells: Default::default(),
            mark: starting_mark
        }
    }

    pub fn get_mark(&self) -> Mark {
        self.mark
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Mark> {
        self.grid_cells[x][y]
    }

    pub fn apply_move(&self, x: usize, y: usize) -> Self {
        let mut new_grid_cells = self.grid_cells.clone();
        new_grid_cells[x][y] = Some(self.mark);
        Self {
            grid_cells: new_grid_cells,
            mark: match self.mark {
                Mark::X => Mark::O,
                Mark::O => Mark::X
            }
        }
    }

    pub fn is_valid_move(&self, x: usize, y: usize) -> bool {
        self.grid_cells.get(x)
            .and_then(|row| row.get(y))
            .map(|cell| cell.is_none())
            .unwrap_or(false)
    }

    pub fn get_game_state(&self) -> GameState {
        pub fn matching<I: Iterator<Item = Option<Mark>>>(mut iter: I) -> Option<Mark> {
            let first = iter.next()?;
            if iter.all(|x| x == first) {
                first
            } else {
                None
            }
        }

        // check rows
        let rows = (0..3).find_map(|y| matching((0..3).map(|x| self.grid_cells[x][y])));

        // check columns
        let cols = (0..3).find_map(|x| matching((0..3).map(|y| self.grid_cells[x][y])));

        // check diagonals
        let main_diag = matching((0..3).map(|i| self.grid_cells[i][i]));
        let anti_diag = matching((0..3).map(|i| self.grid_cells[i][2 - i]));

        if let Some(mark) = rows.or(cols).or(main_diag).or(anti_diag) {
            return GameState::Win(mark)
        }

        if (0..3).all(|x| (0..3).all(|y| self.grid_cells[x][y].is_some())) {
            return GameState::Draw
        }

        GameState::InProgress
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_mark(mark: Option<Mark>) -> String {
            match mark {
                Some(Mark::X) => "X".to_string(),
                Some(Mark::O) => "O".to_string(),
                None => " ".to_string()
            }
        }

        fn fmt_row_i<I: Iterator<Item = Option<Mark>>>(i: u32, mut iter: I) -> String {
            let first = fmt_mark(iter.next().unwrap());
            let second = fmt_mark(iter.next().unwrap());
            let third = fmt_mark(iter.next().unwrap());
            format!(" {i:<2}| {} | {} | {} |", first, second, third)
        }

        let rows = (0..3).map(|y| fmt_row_i(y as u32, (0..3).map(|x| self.get_cell(x, y))));
        let divider = "---+---+---+---+";
        let header =  format!(" {} | 0 | 1 | 2 |", fmt_mark(Some(self.mark)));
        let rows = rows.map(|row| format!("{}\n{}", row, divider));
        let mut result = format!("{}\n{}\n", header, divider);
        for row in rows {
            result.push_str(&format!("{}\n", row)); 
        }
        write!(f, "{}", result)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_row_win() {
        let test_moves = [
            ((0, 0), GameState::InProgress),
            ((1, 0), GameState::InProgress),
            ((0, 1), GameState::InProgress),
            ((1, 1), GameState::InProgress),
            ((0, 2), GameState::Win(Mark::X)),
        ];

        let mut game = Game::new(Mark::X);

        for ((x, y), expected_state) in test_moves.iter() {
            game = game.apply_move(*x, *y);
            assert_eq!(game.get_game_state(), *expected_state);
        }

    }

    #[test]
    fn test_column_win() {
        let test_moves = [
            ((0, 0), GameState::InProgress),
            ((0, 1), GameState::InProgress),
            ((1, 0), GameState::InProgress),
            ((1, 1), GameState::InProgress),
            ((2, 0), GameState::Win(Mark::X)),
        ];

        let mut game = Game::new(Mark::X);

        for ((x, y), expected_state) in test_moves.iter() {
            game = game.apply_move(*x, *y);
            assert_eq!(game.get_game_state(), *expected_state);
        }

    }

    #[test]
    fn test_diagonal_win() {
        let test_moves = [
            ((0, 0), GameState::InProgress),
            ((0, 1), GameState::InProgress),
            ((1, 1), GameState::InProgress),
            ((1, 2), GameState::InProgress),
            ((2, 2), GameState::Win(Mark::X)),
        ];

        let mut game = Game::new(Mark::X);

        for ((x, y), expected_state) in test_moves.iter() {
            game = game.apply_move(*x, *y);
            assert_eq!(game.get_game_state(), *expected_state);
        }

    }

    #[test]
    fn test_draw() {
        let test_moves = [
            ((0, 0), GameState::InProgress),
            ((0, 1), GameState::InProgress),
            ((0, 2), GameState::InProgress),
            ((1, 1), GameState::InProgress),
            ((1, 0), GameState::InProgress),
            ((1, 2), GameState::InProgress),
            ((2, 1), GameState::InProgress),
            ((2, 0), GameState::InProgress),
            ((2, 2), GameState::Draw),
        ];

        let mut game = Game::new(Mark::X);

        for ((x, y), expected_state) in test_moves.iter() {
            game = game.apply_move(*x, *y);
            assert_eq!(game.get_game_state(), *expected_state);
        }
    }
    
}