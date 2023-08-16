use game::{Game, Mark, GameState};
use players::Player;
use console_io::prompt_map;

mod game;
mod players;
mod console_io;

fn main() {
    draw_title();
    let (px, po) = player_select();
    let mut scores = Scores::default();
    let mut round = 1;

    loop {
        let rnd_title = format!("Round {round}");
        println!("{rnd_title}");
        println!("{}", "=".repeat(rnd_title.len()));

        let winner = play_round(px.as_ref(), po.as_ref());

        match winner {
            Some(Mark::X) => scores.x += 1,
            Some(Mark::O) => scores.o += 1,
            None => scores.draws += 1,
        }

        draw_end_of_round(winner, &scores);
        if !play_again() {
            break;
        }
        round += 1;
    }
    draw_cya();
}

#[derive(Clone, Default)]
struct Scores {
    x: u32,
    o: u32,
    draws: u32,
}

fn draw_title() {
    println!("Let's play Tic Tac Toe!");
    println!("=======================");
    println!();
}

fn player_select() -> (Box<dyn Player>, Box<dyn Player>) {
    fn select(title: &str) -> Box<dyn Player> {
        let selection = select_menu(title, &[
            "Human",
            "AI",
        ]);
        println!();
        match selection {
            0 => Box::new(players::Human),
            1 => Box::new(players::AI),
            _ => unreachable!(),
        }
    }

    fn select_menu(title: &str, options: &[&str]) -> usize {
        println!("{title}");
        println!("{}", "=".repeat(title.len()));
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        prompt_map("> ", |s| {
            let selection = s.parse::<usize>().map_err(|_| "Not a valid number")?;
            if selection < 1 || selection > options.len() {
                return Err("Not a valid selection");
            }
            Ok(selection - 1)
        })
    }

    let px = select("Select Player X");
    let po = select("Select Player O");
    (px, po)
}

fn draw_end_of_round(winner: Option<Mark>, scores: &Scores) { 
    let win_str = match winner {
        Some(Mark::X) => "X wins!",
        Some(Mark::O) => "O wins!",
        None => "It's a draw!",
    };
    println!("\n{win_str}");
    println!("{}", "=".repeat(win_str.len()));
    println!(
        "X has {} wins, O has {} wins, and there have been {} draws.",
        scores.x, scores.o, scores.draws
    );
}

fn draw_cya() {
    println!("===================");
    println!("Thanks for playing!");
    println!("===================");
}


fn play_again() -> bool {
    let selection = prompt_map("Play again? (y/n) ", |s| match s {
        "y" => Ok(true),
        "n" => Ok(false),
        _ => Err("please enter 'y' or 'n'"),
    });
    println!();
    selection
}

fn random_mark() -> Mark {
    if rand::random() {
        Mark::X
    } else {
        Mark::O
    }
}

fn play_round(px: &dyn Player, po: &dyn Player) -> Option<Mark> {
    let start_mark = random_mark();
    println!("{} starts the round!", match start_mark {
        Mark::X => "X",
        Mark::O => "O",
    });
    let mut game = Game::new(start_mark);
    let winner = loop {
        println!("\n{}", game);
        let player = match game.get_mark() {
            Mark::X => px,
            Mark::O => po,
        };
        println!("{}'s turn", match game.get_mark() {
            Mark::X => "X",
            Mark::O => "O",
        });
        let (x, y) = player.get_move(&game);
        game = game.apply_move(x, y);

        match game.get_game_state() {
            GameState::InProgress => continue,
            GameState::Draw => break None,
            GameState::Win(winner) => break Some(winner),
        }
    };

    println!("\n{}", game);
    winner
}