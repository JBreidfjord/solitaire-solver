use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

use mctser::GameState;
use peak_alloc::PeakAlloc;

use crate::dfs::dfs_r;
use crate::game::State;
use crate::minimax::best_move;
use crate::states::russian_2;

mod dfs;
mod game;
mod minimax;
mod russian;
mod states;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn run() {
    // let initial_state = state_0001();
    let initial_state = russian_2();

    let max_search_time = Duration::from_secs(60);
    let max_depth = Some(6);
    let target_score = 100.;
    let start = Instant::now();

    // Choose the best move using the minimax algorithm, lowering the depth until a win is found
    let mut state = initial_state.clone();
    while !state.is_terminal() {
        // TODO: How can we better terminate the search when it stalls? E.g. how would we detect we've been stuck at some maximum?
        if let Some((m, path)) = best_move(&state, max_depth, None) {
            println!("{m:?}");
            state = state.apply_move(m);
            println!(
                "Score: {}. Took {}s",
                state.evaluate(),
                start.elapsed().as_secs_f32()
            );
        } else {
            println!("No best move");
            break;
        }
    }

    // state.display_history();
    println!(
        "Final Score: {}. Took {}s",
        state.evaluate(),
        start.elapsed().as_secs_f32()
    );
}

fn run_dfs() {
    let state = russian_2();
    if let Some(path) = dfs_r(state) {
        for mv in path {
            println!("{mv:?}");
        }
    } else {
        println!("No solution found");
    }
}

fn run_mcts() {
    // We use `RC` to store the game status. Create a new game and pass it to the search tree through `RC::clone()`.
    let mut game = Rc::new(russian_2());
    let mut search_tree = mctser::SearchTree::new(game.clone());

    let start = Instant::now();
    while game.end_status().is_none() {
        // Make 1000 simulations to find the best move
        let selected = search_tree.search(3).unwrap();
        // Step forward to the next state using the action provided by the search tree
        search_tree.renew(&selected).unwrap();
        // Get current game state after the move
        game = search_tree.get_game_state();
        println!("Found move: {selected:?} | New score: {}", game.evaluate());
    }
    // game.display_history();
    println!(
        "\nFinal Score: {}. Took {}s",
        game.evaluate(),
        start.elapsed().as_secs_f32()
    );
}

fn main() {
    // Spawn a thread with a larger stack size
    let builder = thread::Builder::new().stack_size(32 * 1024 * 1024); // 32MB of stack space
    let handler = builder.spawn(run_dfs).unwrap();
    handler.join().unwrap();
    // run_dfs()
}
