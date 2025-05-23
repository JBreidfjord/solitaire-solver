use std::io;
use std::time::{Duration, Instant};

use peak_alloc::PeakAlloc;

use crate::dfs::optimal_dfs;
use crate::fortune::display_moves;
use crate::game::State;
use crate::minimax::best_move;
use crate::states::{fortune_5, fortune_6};

mod cribbage;
mod dfs;
mod fortune;
mod game;
mod minimax;
mod russian;
mod states;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn wait_for_key() {
    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
}

// fn display_moves(state: &ProletariatsPatience) {
//     let legal_moves = state.legal_moves();
//     if legal_moves.len() <= 5 {
//         for mv in legal_moves {
//             println!("{mv}");
//         }
//     } else {
//         let left_col_width = legal_moves
//             .chunks(2)
//             .map(|moves| moves[0].to_string().len())
//             .max()
//             .expect("Vec should have at least one chunk")
//             - 3;
//         for moves in legal_moves.chunks(2) {
//             match moves.len() {
//                 1 => println!("{}", moves[0]),
//                 2 => println!(
//                     "{:<left_col_width$} | {}",
//                     format!("{}", moves[0]),
//                     moves[1]
//                 ),
//                 _ => unreachable!("Chunk size is 2"),
//             };
//         }
//     }
// }

fn run() {
    // let initial_state = state_0001();
    // let initial_state = russian_3();
    let initial_state = fortune_6();

    let max_search_time = Some(Duration::from_secs(5));
    let max_depth = Some(10);
    let target_score = 100.;
    let start = Instant::now();
    let mut path = Vec::new();

    // Choose the best move using the minimax algorithm, lowering the depth until a win is found
    let mut state = initial_state.clone();
    while !state.is_terminal() {
        if let Some((m, _)) = best_move(&state, max_depth, max_search_time) {
            print!("\x1B[2J\x1B[1;1H"); // Clear the screen
            println!("\n{m}");
            state = state.act(&m);
            println!(
                "Score: {}. Took {}s",
                state.evaluate(true),
                start.elapsed().as_secs_f32()
            );
            println!("{state}");
            display_moves(&state.possible_actions());
            path.push(m)
            // display_moves(&state);
        } else {
            println!("No best move");
            break;
        }
    }

    println!("Path:");
    for mv in path {
        println!("{mv}");
    }
    // state.display_history();
    println!(
        "Final Score: {}. Took {}s",
        state.evaluate(true),
        start.elapsed().as_secs_f32()
    );
}

fn run_dfs() {
    // let mut state = russian_3();
    let mut state = fortune_5();
    // if let Some(path) = dfs_r(state.clone(), Some(89)) {
    if let Some(path) = optimal_dfs(state.clone()) {
        println!("\nFound solution with {} moves", path.len());
        wait_for_key();

        for mv in path {
            print!("\x1B[2J\x1B[1;1H"); // Clear the screen
            state = state.act(&mv);
            println!("Score: {}", state.evaluate(true));
            println!("\n{mv:?}\n");
            println!("{state:?}");
            wait_for_key();
        }
    } else {
        println!("\nNo solution found");
    }
}

fn play() {
    let mut state = fortune_6();
    loop {
        let mut actions = state.possible_actions();
        actions.sort_by(|a, b| {
            let a_state = state.act(a);
            let b_state = state.act(b);
            a_state.evaluate(false).total_cmp(&b_state.evaluate(false))
        });
        actions.reverse();
        print!("\x1B[2J\x1B[1;1H"); // Clear the screen
        let max_depth = if actions.len() > 1 { Some(15) } else { Some(1) };
        let max_time = if actions.len() > 1 {
            Some(Duration::from_secs(10))
        } else {
            None
        };
        let mut best_idx = None;
        if let Some((best, _)) = best_move(&state, max_depth, max_time) {
            let idx = actions.iter().position(|m| m == &best).unwrap();
            best_idx = Some(idx);
            println!("Best move is {idx}: {best}");
        } else {
            println!("Could not determine best move");
        }
        println!("Score: {}", state.evaluate(true));
        println!("{state}");
        if state.is_terminal() {
            if state.is_win() {
                println!("You win!");
            } else {
                println!("Game over!");
            }
            wait_for_key();
        } else if actions.is_empty() {
            println!("No moves remaining, game over!");
            wait_for_key();
        } else {
            display_moves(&actions);
            println!("Enter move index:");
            let mut input_line = String::new();
            io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            if input_line.trim().is_empty() && best_idx.is_some() {
                input_line = best_idx.unwrap().to_string();
            }
            let move_index: usize = input_line.trim().parse().expect("Input not an integer");

            state = state.act(&actions[move_index]);
        }
    }
}

fn main() {
    // Spawn a thread with a larger stack size
    // let builder = thread::Builder::new().stack_size(64 * 1024 * 1024); // 64MB of stack space
    // let handler = builder.spawn(run_dfs).unwrap();
    // handler.join().unwrap();

    run()
    // play()
}
