use std::time::{Duration, Instant};

use crate::game::State;

/// Minimax algorithm for a single-player game
fn minimax<S: State>(game_state: &S, depth: usize) -> (f32, Vec<<S as State>::Action>) {
    if depth == 0 || game_state.end_status().is_some() {
        if let Some(end_state) = game_state.end_status() {
            if end_state.is_win() {
                println!("game is won, score: {}", game_state.evaluate(false));
            }
        }
        return (game_state.evaluate(false), vec![]);
    }

    let legal_moves = game_state.possible_actions();
    let mut max_eval = f32::MIN;
    let mut best_path = Vec::new();

    for m in legal_moves {
        let simulated_state = game_state.act(&m);
        let (eval, path) = minimax(&simulated_state, depth - 1);
        if eval > max_eval {
            max_eval = eval;
            best_path = vec![m];
            best_path.extend(path);
        }
    }

    (max_eval, best_path)
}

/// Function to choose the best move using minimax algorithm
pub fn best_move<S>(
    game_state: &S,
    max_depth: Option<usize>,
    search_time: Option<Duration>,
) -> Option<(<S as State>::Action, Vec<<S as State>::Action>)>
where
    S: State,
{
    let max_depth = max_depth.or(Some(usize::MAX)).unwrap();
    let search_time = search_time.or(Some(Duration::MAX)).unwrap();

    let mut best_move = None;
    let mut best_score = f32::MIN;
    let mut best_path = Vec::new();

    let time = Instant::now();
    for depth in 1..=max_depth {
        let legal_moves = game_state.possible_actions();
        if legal_moves.len() == 1 {
            return Some((legal_moves[0].clone(), vec![]));
        }
        for m in legal_moves {
            let simulated_state = game_state.act(&m);
            let (score, path) = minimax(&simulated_state, depth - 1);
            if score >= best_score {
                best_score = score;
                best_move = Some(m);
                best_path = path;
            }
            if time.elapsed() > search_time {
                return best_move.map(|m| (m, best_path));
            }
        }
    }

    best_move.map(|m| (m, best_path))
}
