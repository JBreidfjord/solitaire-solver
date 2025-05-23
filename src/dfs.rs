use std::collections::HashSet;
use std::fmt::Debug;
use std::io;
use std::time::{Duration, Instant};

use crate::game::{EndState, State};
use crate::PEAK_ALLOC;

pub fn dfs_r<S: State + Clone + Debug>(
    state: S,
    max_depth: Option<usize>,
    max_search_time: Option<Duration>,
) -> Option<Vec<<S as State>::Action>> {
    fn backtrack<S: State + Clone + Debug>(
        state: S,
        path: Vec<<S as State>::Action>,
        cache: &mut HashSet<S>,
        depth: usize,
        search_start_time: Instant,
        max_depth: usize,
        max_search_time: Duration,
    ) -> Option<Vec<<S as State>::Action>> {
        if depth >= max_depth || search_start_time.elapsed() > max_search_time {
            return None;
        }
        // If the insert returns `false`, we've already seen this state so we can skip
        if !cache.insert(state.clone()) {
            return None;
        }
        if depth == max_depth {
            // println!("\nChecking state {state:?} with path {path:?}\n");
            println!("\n Checking state {state:?}\n");
            for action in state.possible_actions() {
                println!("{action:?}");
            }
            io::stdin()
                .read_line(&mut String::new())
                .expect("Failed to read line");
        }

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
                "\rDepth: {depth}\tRAM: {:.2}MB",
                PEAK_ALLOC.current_usage_as_mb()
            );
            let result = backtrack(
                next_state,
                path.clone(),
                cache,
                depth + 1,
                search_start_time,
                max_depth,
                max_search_time,
            );
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
    backtrack(
        state,
        vec![],
        &mut cache,
        0,
        Instant::now(),
        max_depth.unwrap_or(usize::MAX),
        max_search_time.unwrap_or(Duration::MAX),
    )
}

pub fn optimal_dfs<S: State + Clone + Debug>(state: S) -> Option<Vec<<S as State>::Action>> {
    let max_search_time = Duration::from_secs(60);
    let mut best_depth = usize::MAX;
    // let mut best_depth = 1000;
    let mut best_path = None;
    loop {
        let start = Instant::now();
        if let Some(path) = dfs_r(state.clone(), Some(best_depth), Some(max_search_time)) {
            println!(
                "\nFound solution with {} moves in {:.2}s\n",
                path.len(),
                start.elapsed().as_secs_f32()
            );
            best_depth = path.len();
            best_path = Some(path);
        } else {
            println!("\nNo solution found for max depth {best_depth}");
            break;
        }
    }

    best_path
}
