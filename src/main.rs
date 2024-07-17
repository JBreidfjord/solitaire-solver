use std::{io, thread};
use std::rc::Rc;
use std::time::{Duration, Instant};

use mctser::GameState;
use peak_alloc::PeakAlloc;

use crate::dfs::optimal_dfs;
use crate::game::State;
use crate::minimax::best_move;
use crate::russian::ProletariatsPatience;
use crate::states::russian_3;

mod dfs;
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

fn display_moves(state: &ProletariatsPatience) {
    let legal_moves = state.legal_moves();
    if legal_moves.len() <= 5 {
        for mv in legal_moves {
            println!("{mv}");
        }
    } else {
        let left_col_width = legal_moves
            .chunks(2)
            .map(|moves| moves[0].to_string().len())
            .max()
            .expect("Vec should have at least one chunk")
            - 3;
        for moves in legal_moves.chunks(2) {
            match moves.len() {
                1 => println!("{}", moves[0]),
                2 => println!(
                    "{:<left_col_width$} | {}",
                    format!("{}", moves[0]),
                    moves[1]
                ),
                _ => unreachable!("Chunk size is 2"),
            };
        }
    }
}

fn run() {
    // let initial_state = state_0001();
    let initial_state = russian_3();

    let max_search_time = Duration::from_secs(60);
    let max_depth = Some(7);
    let target_score = 100.;
    let start = Instant::now();

    // Choose the best move using the minimax algorithm, lowering the depth until a win is found
    let mut state = initial_state.clone();
    while !state.is_terminal() {
        // TODO: How can we better terminate the search when it stalls? E.g. how would we detect we've been stuck at some maximum?
        if let Some((m, path)) = best_move(&state, max_depth, None) {
            print!("\x1B[2J\x1B[1;1H"); // Clear the screen
            println!("\n{m}");
            state = state.apply_move(m);
            println!(
                "Score: {}. Took {}s",
                state.evaluate(true),
                start.elapsed().as_secs_f32()
            );
            println!("{state}");
            println!("{}", state.is_win());
            display_moves(&state);
        } else {
            println!("No best move");
            break;
        }
    }

    // state.display_history();
    println!(
        "Final Score: {}. Took {}s",
        state.evaluate(true),
        start.elapsed().as_secs_f32()
    );
}

fn run_dfs() {
    let mut state = russian_3();
    // if let Some(path) = dfs_r(state.clone(), Some(89)) {
    if let Some(path) = optimal_dfs(state.clone()) {
        println!("\nFound solution with {} moves", path.len());
        wait_for_key();

        for mv in path {
            print!("\x1B[2J\x1B[1;1H"); // Clear the screen
            state = state.apply_move(mv.clone());
            println!("Score: {}", state.evaluate(true));
            println!("\n{mv}\n");
            println!("{state}");
            wait_for_key();
        }
    } else {
        println!("\nNo solution found");
    }
}

fn run_mcts() {
    // We use `RC` to store the game status. Create a new game and pass it to the search tree through `RC::clone()`.
    let mut game = Rc::new(russian_3());
    let mut search_tree = mctser::SearchTree::new(game.clone());

    let start = Instant::now();
    while game.end_status().is_none() {
        // Make 1000 simulations to find the best move
        let selected = search_tree.search(3).unwrap();
        // Step forward to the next state using the action provided by the search tree
        search_tree.renew(&selected).unwrap();
        // Get current game state after the move
        game = search_tree.get_game_state();
        println!(
            "Found move: {selected:?} | New score: {}",
            game.evaluate(true)
        );
    }
    // game.display_history();
    println!(
        "\nFinal Score: {}. Took {}s",
        game.evaluate(true),
        start.elapsed().as_secs_f32()
    );
}

fn main() {
    // Spawn a thread with a larger stack size
    let builder = thread::Builder::new().stack_size(32 * 1024 * 1024); // 32MB of stack space
    let handler = builder.spawn(run_dfs).unwrap();
    handler.join().unwrap();
}
