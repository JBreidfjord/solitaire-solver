use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use derivative::Derivative;
use mctser::{GameState, Player as _Player};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::game::{Card, MaybeCard, Rank, State};

// Rules:
// - Number cards are stacked by alternating colour and decreasing value,
//   and can be moved as a stack of any size.
// - Face cards are stacked by suit in any order, and can also be moved as a stack. However,
//   a completed stack of face cards placed directly on the board will become immovable.
// - To win, sort the dealt cards into four completed stacks of number cards and four.
//   completed stacks of face cards.
// - The free cell can store a single card of any type.

// TODO: custom implementation of Hash that doesn't include history
#[derive(Derivative, Clone, Eq, Serialize, Deserialize)]
#[derivative(Debug, Hash, PartialEq)]
pub struct ProletariatsPatience {
    free_cell: Option<Card>,
    tableau: [Vec<Card>; 9],
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    #[derivative(Debug = "ignore")]
    history: Vec<Move>,
}

impl ProletariatsPatience {
    pub fn new(tableau: [Vec<Card>; 9]) -> Self {
        Self {
            tableau,
            ..Self::default()
        }
    }

    pub fn default() -> Self {
        Self {
            free_cell: None,
            tableau: Self::random_tableau(),
            history: Vec::new(),
        }
    }

    pub fn apply_move(&self, mv: Move) -> Self {
        let mut history = self.history.clone();
        history.push(mv.clone());

        let mut tableau = self.tableau.clone();
        match mv {
            Move::Free { card, from } => {
                // if card.suit == Spade && card.rank == Ace {
                //     println!("Freeing Ace {mv:?}");
                // }

                let top_card = tableau[from].pop().unwrap();
                // TODO: We shouldn't need to validate these here; rely on tests and trust the code
                if card != top_card {
                    println!("{mv:?}");
                    panic!("Tried to free card that didn't match top card of given column");
                }
                if self.free_cell.is_some() {
                    println!("{self:?}");
                    println!("{mv:?}");
                    panic!("Tried to place a card in free cell but it was not empty");
                }
                Self {
                    free_cell: Some(card),
                    tableau,
                    history,
                }
            }
            Move::Unfree { card, to } => {
                // if card.suit == Spade && card.rank == Ace {
                //     println!("Unfreeing Ace {mv:?}");
                // }

                if self.free_cell.is_none() {
                    panic!("Tried to remove a card from free cell but there wasn't one");
                }
                tableau[to].push(card);
                Self {
                    free_cell: None,
                    tableau,
                    history,
                }
            }
            Move::Stack {
                ref cards,
                from,
                to,
            } => {
                // for card in cards {
                //     if card.suit == Spade && card.rank == Ace {
                //         println!("Moving Ace {:?}", &mv);
                //     }
                // }

                let moved_cards = tableau[from]
                    .drain((tableau[from].len() - cards.len())..)
                    .collect::<Vec<_>>();
                if &moved_cards != cards {
                    panic!("Moved cards don't match given cards {moved_cards:?} != {cards:?}");
                }
                tableau[to].extend(cards);
                Self {
                    free_cell: self.free_cell,
                    tableau,
                    history,
                }
            }
        }
    }

    pub fn revert_move(&self, mv: &Move) -> Self {
        let mut history = self.history.clone();
        let last_move = history.pop().expect("Tried to revert a root state");
        if &last_move != mv {
            panic!("Tried to revert {mv:?} but last move was {last_move:?}");
        }

        let mut tableau = self.tableau.clone();
        match mv {
            Move::Free { card, from } => {
                if self.free_cell.is_none() {
                    panic!("Tried to remove a card from free cell but there wasn't one");
                }
                tableau[*from].push(*card);
                Self {
                    free_cell: None,
                    tableau,
                    history,
                }
            }
            Move::Unfree { card, to } => {
                let top_card = tableau[*to].pop().unwrap();
                if card != &top_card {
                    println!("{mv:?}");
                    panic!(
                        "Tried to revert unfreeing card that didn't match top card of given column"
                    );
                }
                if self.free_cell.is_some() {
                    println!("{self:?}");
                    println!("{mv:?}");
                    panic!("Tried to revert card into free cell but it was not empty");
                }
                Self {
                    free_cell: Some(*card),
                    tableau,
                    history,
                }
            }
            Move::Stack { cards, from, to } => {
                tableau[*to].truncate(tableau[*to].len() - cards.len());
                tableau[*from].extend(cards);
                Self {
                    free_cell: self.free_cell,
                    tableau,
                    history,
                }
            }
        }
    }

    fn random_tableau() -> [Vec<Card>; 9] {
        let mut cards = Card::default_deck()
            .into_iter()
            .filter(|card| {
                card.rank != Rank::Two
                    && card.rank != Rank::Three
                    && card.rank != Rank::Four
                    && card.rank != Rank::Five
            })
            .collect::<Vec<_>>();

        cards.shuffle(&mut thread_rng());

        let chunk_size = cards.len() / 9;
        [
            cards.split_off(cards.len() - chunk_size),
            cards.split_off(cards.len() - chunk_size),
            cards.split_off(cards.len() - chunk_size),
            cards.split_off(cards.len() - chunk_size),
            cards.split_off(cards.len() - chunk_size),
            cards.split_off(cards.len() - chunk_size),
            cards.split_off(cards.len() - chunk_size),
            cards.split_off(cards.len() - chunk_size),
            cards.split_off(cards.len() - chunk_size),
        ]
    }

    /// Panics if the tableau isn't valid.
    /// Meant to be used after manually constructing a deck to ensure no mistakes were made.
    pub fn validate_tableau(&self) {
        let mut seen = HashSet::new();
        if let Some(free_card) = &self.free_cell {
            seen.insert(free_card);
        }
        for column in &self.tableau {
            for card in column {
                // This deck doesn't contain 2, 3, 4, or 5
                if card.rank <= Rank::Five && card.rank >= Rank::Two {
                    panic!("Found card with invalid rank: {card:?}");
                }
                // We don't want any duplicates
                if !seen.insert(card) {
                    panic!("Found duplicate card: {card:?}");
                }
            }
        }

        // There should be exactly 36 cards
        if seen.len() != 36 {
            panic!("Found invalid amount of cards: {}", seen.len());
        }
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        self.validate_tableau(); // TODO: remove

        let mut moves = Vec::new();

        // Legal moves:
        // - We can move all top cards to the free cell, given that:
        //   - They aren't in a completed stack of face cards
        //     - Completed means a stack of 4 on the board (not stacked)
        //   - The free cell is empty
        // - We can move the card from the free cell anywhere it can stack
        // - We can move stacks of cards if they themselves are stackable and there is a stackable destination
        //   - For each column, starting at the top card:
        //     - Determine the stackable depth, that is the number of cards that form a valid stack
        //     - Then, starting at the bottom of the stack (i.e. the deepest card):
        //       - Check for valid destinations (e.g. top card is stackable or empty column)
        for (from_idx, from_col) in self.tableau.iter().enumerate() {
            // No moves can be made from this column (completed face stacks are locked)
            if from_col.is_empty() || Self::is_completed_face_stack(from_col) {
                continue;
            }
            let top_card = from_col.last().expect("Vec was checked to be non-empty");

            // We can move the top card to the free cell given that the free cell is empty and
            // this column isn't a completed face card stack (since those can't ever be moved)
            if self.free_cell.is_none() {
                moves.push(Move::Free {
                    card: *top_card,
                    from: from_idx,
                });
            }

            // Find the stackable cards on top of this column
            // TODO: Could we do this with take_while?
            let mut stack = vec![top_card];
            let mut prev_card = top_card;
            for card in from_col[..from_col.len() - 1].iter().rev() {
                if Self::can_stack(prev_card, card) {
                    stack.push(card);
                    prev_card = card;
                } else {
                    // If we hit a card that isn't part of the stack, we're done
                    break;
                }
            }
            let stack_is_entire_col = stack.len() == from_col.len();

            // We now have `n` valid stacks, where `n` is the length of the stack,
            // so we look for valid destinations for all of them
            while !stack.is_empty() {
                let first_card = stack.last().expect("Vec was checked to be non-empty");
                let mut moved_to_empty_column = false; // all empty column moves are the same
                for (to_idx, to_col) in self.tableau.iter().enumerate() {
                    if from_idx == to_idx {
                        continue;
                    }

                    // We don't want moves that just move a stack from one empty column to another,
                    // so if this stack is the entire column (and therefore would leave it empty),
                    // and the dest is empty, we skip
                    if to_col.is_empty() && stack_is_entire_col {
                        continue;
                    }

                    // This is a valid destination if our first card can stack on the top card,
                    // or if the column is empty
                    if (!moved_to_empty_column && to_col.is_empty())
                        || (!to_col.is_empty()
                            && first_card.can_stack_on(
                                to_col.last().expect("Vec was checked to be non-empty"),
                            ))
                    {
                        moves.push(Move::Stack {
                            cards: stack.iter().rev().copied().copied().collect(),
                            from: from_idx,
                            to: to_idx,
                        });
                        moved_to_empty_column = true;
                    }
                }

                stack.pop();
            }
        }

        // Generate moves for card in free cell
        if let Some(free_card) = self.free_cell {
            for (to_idx, to_col) in self.tableau.iter().enumerate() {
                // This is a valid destination if the free card can stack on the top card,
                // or if the column is empty
                if to_col.is_empty()
                    || free_card
                        .can_stack_on(to_col.last().expect("Vec was checked to be non-empty"))
                {
                    moves.push(Move::Unfree {
                        card: free_card,
                        to: to_idx,
                    });
                }
            }
        }

        // Remove all moves we used in the last `n` turns to prevent cycles
        // TODO: There's probably a better way to handle cycles than this
        moves.retain(|m| {
            !self
                .history
                .iter()
                .rev()
                .take(8)
                .collect::<Vec<_>>()
                .contains(&m)
        });

        if !self.history.is_empty() {
            let last_move = self
                .history
                .last()
                .expect("History was checked to be non-empty");

            // Remove moves that are just moving the same cards as the last move
            let last_cards = match last_move {
                Move::Free { card, .. } | Move::Unfree { card, .. } => &vec![*card],
                Move::Stack { cards, .. } => cards,
            };
            moves.retain(|m| {
                let cards = match m {
                    Move::Free { card, .. } | Move::Unfree { card, .. } => &vec![*card],
                    Move::Stack { cards, .. } => cards,
                };
                cards != last_cards
            });

            // If we just freed a card, we can't unfree it immediately
            if last_move.is_free() {
                moves.retain(|m| !m.is_unfree());
            }
        }

        // TODO: similarly we can remove moves that are reversions of the previous move
        // TODO: Should we remove _all_ moves that could just be undone next turn?

        if moves.len() > 148 {
            println!("move len: {}", moves.len());
        }

        // Sort descending so the "best" moves are at the front
        moves.sort_unstable();
        moves.reverse();

        moves
    }

    fn can_stack(top: &Card, bottom: &Card) -> bool {
        match (top.is_face_or_ace(), bottom.is_face_or_ace()) {
            (true, true) => top.suit == bottom.suit, // Face cards can stack by suit
            (false, false) => {
                top.color() != bottom.color() && top.rank as usize == (bottom.rank as usize) - 1
            } // Number cards stack by alternating color and decreasing value
            _ => false,
        }
    }

    pub fn is_win(&self) -> bool {
        let mut number_stacks = 0;
        let mut face_stacks = 0;

        for column in &self.tableau {
            if Self::is_completed_number_stack(column) {
                number_stacks += 1;
            } else if Self::is_completed_face_stack(column) {
                face_stacks += 1;
            }
        }

        number_stacks == 4 && face_stacks == 4
    }

    fn is_completed_number_stack(stack: &[Card]) -> bool {
        if stack.len() != 5 {
            return false;
        }

        let expected_ranks = [Rank::Ten, Rank::Nine, Rank::Eight, Rank::Seven, Rank::Six];
        for i in 0..5 {
            if stack[i].rank != expected_ranks[i]
                || (i > 0 && stack[i].color() == stack[i - 1].color())
            {
                return false;
            }
        }

        true
    }

    fn is_completed_face_stack(stack: &[Card]) -> bool {
        if stack.len() != 4 {
            return false;
        }

        let suit = stack[0].suit;
        for card in stack {
            if !card.is_face_or_ace() || card.suit != suit {
                return false;
            }
        }

        true
    }

    pub fn is_terminal(&self) -> bool {
        self.is_win() || self.legal_moves().is_empty()
    }

    pub fn display_history(&self) {
        for mv in &self.history {
            println!("{mv}");
        }
    }

    pub fn tableau(&self) -> &[Vec<Card>; 9] {
        &self.tableau
    }

    pub fn history(&self) -> &Vec<Move> {
        &self.history
    }

    pub fn peek_column(&self, column: usize) -> Option<&Card> {
        self.tableau[column].last()
    }

    fn heuristic_score(&self, print_components: bool) -> f32 {
        // Weights for the heuristic components
        let weight_completed_stacks = 10.0; // 10.0
        let weight_card_depth_penalty = 1.0;
        let weight_stack_progress = 2.0;
        let weight_moves_available = 0.5; // 1.0
        let weight_empty_columns = 0.0; // 5.0
        let weight_free_cell = 0.0; // 2.0
        let weight_repetition_penalty = 1.0; // 5.0
        let weight_depth_penalty = 0.25; // 0.5

        let mut score = 0.0;

        // Number of Completed Stacks
        if weight_completed_stacks > 0.0 {
            let mut completed_number_stacks = 0;
            let mut completed_face_stacks = 0;
            for column in &self.tableau {
                if column.len() == 5 && Self::is_completed_number_stack(column) {
                    completed_number_stacks += 1;
                }
                if column.len() == 4 && Self::is_completed_face_stack(column) {
                    completed_face_stacks += 1;
                }
            }
            if print_components {
                println!(
                    "Completed stacks: {} * {weight_completed_stacks} = {}",
                    completed_number_stacks + completed_face_stacks,
                    (completed_number_stacks + completed_face_stacks) as f32
                        * weight_completed_stacks
                );
            }
            score +=
                (completed_number_stacks + completed_face_stacks) as f32 * weight_completed_stacks;
        }

        // Depth of cards and progress of stacks
        if weight_card_depth_penalty > 0.0 || weight_stack_progress > 0.0 {
            let mut stack_size = 0;
            let mut depth_error = 0;

            for column in &self.tableau {
                for cards in column.windows(2).rev() {
                    if cards[1].can_stack_on(&cards[0]) {
                        stack_size += 1;
                    } else {
                        break;
                    }
                }

                for (depth, card) in column.iter().rev().enumerate() {
                    if card.is_face_or_ace() {
                        continue;
                    }
                    let goal_depth = card.rank as usize - 6;
                    depth_error += goal_depth.abs_diff(depth);
                }
            }

            if print_components {
                println!(
                    "Stack progress: {stack_size} * {weight_stack_progress} = {}",
                    stack_size as f32 * weight_stack_progress
                );
            }
            score += stack_size as f32 * weight_stack_progress;

            if print_components {
                println!(
                    "Depth error: {depth_error} * -{weight_card_depth_penalty} = {}",
                    depth_error as f32 * -weight_card_depth_penalty
                );
            }
            score -= depth_error as f32 * weight_card_depth_penalty;
        }

        // Number of Moves Available
        let legal_moves = self.legal_moves();
        if print_components {
            println!(
                "Moves available: {} * {weight_moves_available} = {}",
                legal_moves.len(),
                legal_moves.len() as f32 * weight_moves_available
            );
        }
        score += legal_moves.len() as f32 * weight_moves_available;

        // Empty Columns
        if weight_empty_columns > 0.0 {
            let empty_columns = self
                .tableau
                .iter()
                .filter(|column| column.is_empty())
                .count();
            if print_components {
                println!(
                    "Empty columns: {empty_columns} * {weight_empty_columns} = {}",
                    empty_columns as f32 * weight_empty_columns
                );
            }
            score += empty_columns as f32 * weight_empty_columns;
        }

        // Free Cell Utilization
        if weight_free_cell > 0.0 {
            if self.free_cell.is_none() {
                score += weight_free_cell;
            } else {
                score -= weight_free_cell;
            }

            if print_components {
                println!(
                    "Free cell: {:?} * {weight_free_cell} = {}",
                    self.free_cell,
                    if self.free_cell.is_none() {
                        weight_free_cell
                    } else {
                        -weight_free_cell
                    }
                );
            }
        }

        // Penalize repetitive moves
        if weight_repetition_penalty > 0.0 {
            let mut move_counts = HashMap::new();
            for mv in &self.history {
                *move_counts.entry(mv).or_insert(0) += 1;
            }

            let repetitions = move_counts.values().sum::<usize>() - move_counts.len();
            if print_components {
                println!(
                    "Repetition: {repetitions} * -{weight_repetition_penalty} = {}",
                    repetitions as f32 * -weight_repetition_penalty,
                )
            }
            score -= repetitions as f32 * weight_repetition_penalty;
        }

        if print_components {
            println!(
                "Depth: {} * -{weight_depth_penalty} = {}",
                self.history.len(),
                self.history.len() as f32 * -weight_depth_penalty,
            )
        }
        score -= self.history.len() as f32 * weight_depth_penalty;

        score
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum Move {
    Stack {
        cards: Vec<Card>,
        from: usize,
        to: usize,
    },
    Free {
        card: Card,
        from: usize,
    },
    Unfree {
        card: Card,
        to: usize,
    },
}

impl Move {
    pub fn is_free(&self) -> bool {
        match *self {
            Move::Free { .. } => true,
            _ => false,
        }
    }

    pub fn is_unfree(&self) -> bool {
        match *self {
            Move::Unfree { .. } => true,
            _ => false,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        // Bigger stacks > smaller stacks > unfree > free
        match self {
            Move::Free { .. } => match other {
                Move::Free { .. } => Ordering::Equal,
                _ => Ordering::Less,
            },
            Move::Unfree { .. } => match other {
                Move::Unfree { .. } => Ordering::Equal,
                Move::Free { .. } => Ordering::Greater,
                Move::Stack { .. } => Ordering::Less,
            },
            Move::Stack { cards, .. } => match other {
                Move::Stack {
                    cards: other_cards, ..
                } => cards.len().cmp(&other_cards.len()),
                _ => Ordering::Greater,
            },
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Free { card, from } => write!(f, "{from}->\u{1F0A0}\t{card}"),
            Move::Unfree { card, to } => write!(f, "\u{1F0A0}->{to}\t{card}"),
            Move::Stack { cards, from, to } => {
                write!(
                    f,
                    "{from}->{to}\t{}",
                    cards
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum EndState {
    Win,
    Loss,
}

pub struct Player;

impl mctser::EndStatus for EndState {}
impl mctser::Action for Move {}

impl mctser::Player<EndState> for Player {
    fn reward_when_outcome_is(&self, outcome: &EndState) -> f32 {
        match outcome {
            EndState::Win => 1000.,
            EndState::Loss => -1000.,
        }
    }
}

impl GameState<Player, EndState, Move> for ProletariatsPatience {
    fn player(&self) -> Player {
        Player
    }

    fn end_status(&self) -> Option<EndState> {
        if self.is_terminal() {
            if self.is_win() {
                Some(EndState::Win)
            } else {
                Some(EndState::Loss)
            }
        } else {
            None
        }
    }

    fn possible_actions(&self) -> Vec<Move> {
        self.legal_moves()
    }

    fn act(&self, action: &Move) -> Self {
        self.apply_move(action.to_owned())
    }
}

impl State<Player, EndState, Move> for ProletariatsPatience {
    fn evaluate(&self, print_components: bool) -> f32 {
        match self.end_status() {
            Some(end_state) => self.player().reward_when_outcome_is(&end_state),
            None => self.heuristic_score(print_components),
        }
    }

    fn revert(&self, action: &Move) -> Self {
        self.revert_move(action)
    }
}

impl Display for ProletariatsPatience {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_col_len = self
            .tableau
            .iter()
            .map(|col| col.len())
            .max()
            .expect("Tableau should not be fully empty");

        for i in 0..max_col_len {
            let row = format!(
                "{} {} {} {} {} {} {} {} {}",
                MaybeCard(self.tableau[0].get(i).copied()),
                MaybeCard(self.tableau[1].get(i).copied()),
                MaybeCard(self.tableau[2].get(i).copied()),
                MaybeCard(self.tableau[3].get(i).copied()),
                MaybeCard(self.tableau[4].get(i).copied()),
                MaybeCard(self.tableau[5].get(i).copied()),
                MaybeCard(self.tableau[6].get(i).copied()),
                MaybeCard(self.tableau[7].get(i).copied()),
                MaybeCard(self.tableau[8].get(i).copied()),
            );
            let free_cell = if i == 0 {
                if let Some(free_card) = self.free_cell {
                    format!("\t{free_card}")
                } else {
                    format!("\t{:^3}", '\u{1F0A0}')
                }
            } else {
                "".to_owned()
            };
            writeln!(f, "{row}{free_cell}")?;
        }

        Ok(())
    }
}

// TODO: Add test cases, especially for generating and applying moves
#[cfg(test)]
mod test {
    use crate::game::Rank::*;
    use crate::game::Suit::*;
    use crate::states::{russian, russian_2};

    use super::*;

    #[test]
    fn test_legal_moves() {
        let state = russian();
        let legal_moves = state.legal_moves();

        let free_moves = [
            Move::Free {
                card: Card {
                    suit: Spade,
                    rank: Nine,
                },
                from: 0,
            },
            Move::Free {
                card: Card {
                    suit: Club,
                    rank: King,
                },
                from: 1,
            },
            Move::Free {
                card: Card {
                    suit: Diamond,
                    rank: Six,
                },
                from: 2,
            },
            Move::Free {
                card: Card {
                    suit: Diamond,
                    rank: Seven,
                },
                from: 3,
            },
            Move::Free {
                card: Card {
                    suit: Spade,
                    rank: Ten,
                },
                from: 4,
            },
            Move::Free {
                card: Card {
                    suit: Spade,
                    rank: Ace,
                },
                from: 5,
            },
            Move::Free {
                card: Card {
                    suit: Club,
                    rank: Jack,
                },
                from: 6,
            },
            Move::Free {
                card: Card {
                    suit: Heart,
                    rank: Nine,
                },
                from: 7,
            },
            Move::Free {
                card: Card {
                    suit: Diamond,
                    rank: Ace,
                },
                from: 8,
            },
        ];
        for free_move in &free_moves {
            assert!(legal_moves.contains(free_move));
        }

        let stack_moves = [
            Move::Stack {
                cards: vec![Card {
                    suit: Heart,
                    rank: Nine,
                }],
                from: 7,
                to: 4,
            },
            Move::Stack {
                cards: vec![Card {
                    suit: Club,
                    rank: King,
                }],
                from: 1,
                to: 6,
            },
            Move::Stack {
                cards: vec![Card {
                    suit: Club,
                    rank: Jack,
                }],
                from: 6,
                to: 1,
            },
        ];
        for stack_move in &stack_moves {
            assert!(legal_moves.contains(stack_move));
        }

        let additional_moves = legal_moves
            .iter()
            .filter(|m| !free_moves.contains(m) && !stack_moves.contains(m))
            .collect::<Vec<_>>();
        assert_eq!(
            legal_moves.len(),
            free_moves.len() + stack_moves.len(),
            "Additional moves: {additional_moves:?}"
        )
    }

    #[test]
    fn revert_unapplies_action() {
        let state = russian_2();

        for mv in state.legal_moves() {
            let child_state = state.apply_move(mv.clone());
            let reverted_state = child_state.revert_move(&mv);
            assert_eq!(reverted_state, state);
        }
    }
}
