use std::collections::HashSet;
use std::fmt::Debug;

use mctser::{Action, Player};

use crate::game::State;
use crate::PEAK_ALLOC;
use crate::russian::EndState;

pub fn dfs<P, S, A>(state: &S) -> Option<Vec<A>>
where
    P: Player<EndState>,
    S: State<P, EndState, A> + Clone,
    A: Action + Debug,
{
    let mut state = state.clone();
    let actions = state.possible_actions();
    let mut seen = HashSet::new(); // TODO: replace with lru cache
    let mut path = Vec::new();
    let mut searched = 0;
    for root_mv in actions {
        println!("Checking root {root_mv:?}");
        let mut stack = vec![root_mv];
        while let Some(mv) = stack.pop() {
            println!("\nChecking {mv:?}, currently stacked: {}", stack.len());
            println!("{stack:?}");
            state = state.act(&mv);
            if !seen.contains(&state) {
                searched += 1;
                // print!("\r{searched}");
                seen.insert(state.clone());
                path.push(mv.clone());
                if let Some(end_state) = state.end_status() {
                    match end_state {
                        EndState::Win => return Some(path),
                        EndState::Loss => {
                            let last_move = path.pop().unwrap();
                            println!("Reverting {mv:?} for loss, last move: {last_move:?}");
                            state = state.revert(&mv);
                        }
                    }
                } else {
                    for child_mv in state.possible_actions() {
                        stack.push(child_mv);
                    }
                }
            } else {
                let last_move = path.pop().unwrap();
                println!("Reverting {mv:?} for seen, last move: {last_move:?}");
                state = state.revert(&mv);
            }
        }

        // Revert the initial move
        if let Some(last_mv) = path.pop() {
            state = state.revert(&last_mv);
        }
    }

    None
}

pub fn dfs_r<P, S, A>(state: S) -> Option<Vec<A>>
where
    P: Player<EndState>,
    S: State<P, EndState, A> + Clone + Debug,
    A: Action + Debug,
{
    fn backtrack<P, S, A>(
        state: S,
        path: Vec<A>,
        cache: &mut HashSet<S>,
        depth: usize,
    ) -> Option<Vec<A>>
    where
        P: Player<EndState>,
        S: State<P, EndState, A> + Clone + Debug,
        A: Action + Debug,
    {
        // If the insert returns `false`, we've already seen this state so we can skip
        if !cache.insert(state.clone()) {
            return None;
        }
        // println!("\nChecking state {state:?} with path {path:?}");
        // io::stdin()
        //     .read_line(&mut String::new())
        //     .expect("Failed to read line");

        if let Some(end_state) = state.end_status() {
            return match end_state {
                EndState::Win => Some(path),
                EndState::Loss => None,
            };
        }

        let mut path = path.clone();
        for mv in state.possible_actions() {
            let next_state = state.act(&mv);
            path.push(mv.clone());

            print!(
                "\rDepth: {depth}\tRAM: {}MB",
                PEAK_ALLOC.current_usage_as_mb()
            );
            let result = backtrack(next_state, path.clone(), cache, depth + 1);
            if result.is_some() {
                return result;
            }

            path.pop();
            // We don't need to revert, since this is the original unchanged state
            // state.revert(&mv);
        }

        None
    }

    let mut cache = HashSet::new(); // TODO: replace with lru cache
    backtrack(state, vec![], &mut cache, 0)
}
