use std::fmt::Debug;
use std::hash::Hash;

use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use crate::game::{Card, EndState, Rank, State, Suit};
use crate::game::Rank::{Jack, Ten};

// Game rules:
// Play cards from the tableau to the stack one at a time, scoring points based on the card (see 'Scoring')
// Get 61 or more points to win
// The stack total may not exceed 31
// When calculating the stack total, face cards are each worth 10
// If you can play a card, you must
// If you cannot, start a new stack

// Scoring:
// First card played to the stack is a Jack: +2 points
// Stack total is exactly 15: +2 points
// Stack total is exactly 31: +2 points
// Set of 2, 3, or 4 of the same card: +2/+6/+12 points
// Run of 3 to 7 cards, in any order, such as K-J-Q or 2-4-3-A: +3 to +7 points

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CribbageSolitaire {
    score: u8,
    stack: Vec<Card>,
    tableau: [Vec<Card>; 4],
    history: Vec<Move>,
}

impl CribbageSolitaire {
    pub fn new(tableau: [Vec<Card>; 4]) -> Self {
        Self {
            tableau,
            ..Self::default()
        }
    }

    pub fn default() -> Self {
        Self {
            score: 0,
            stack: Vec::new(),
            tableau: Self::random_tableau(),
            history: Vec::new(),
        }
    }

    pub fn from_str(input: &str) -> Self {
        // TODO: support parsing other properties besides tableau
        let input = input.trim();
        let mut tableau = [vec![], vec![], vec![], vec![]];
        let mut rank = None;
        let mut suit = None;
        let mut col_idx = 0;
        for (idx, char) in input.char_indices() {
            // TODO: we need to handle gaps properly, not just skip whitespace
            if char.is_whitespace() {
                continue;
            }

            if char == '0' && input.chars().nth(idx - 1) == Some('1') {
                rank = Some(Ten);
            } else if let Ok(new_rank) = Rank::try_from(char.to_string()) {
                rank = Some(new_rank);
            }

            if let Ok(new_suit) = Suit::try_from(char.to_string()) {
                suit = Some(new_suit);
            }

            if rank.is_some() && suit.is_some() {
                tableau[col_idx].push(Card {
                    rank: rank.unwrap(),
                    suit: suit.unwrap(),
                });
                rank = None;
                suit = None;
                col_idx += 1;
            }
        }

        Self {
            tableau,
            ..Self::default()
        }
    }

    pub fn apply_move(&self, mv: Move) -> Self {
        let mut history = self.history.clone();
        history.push(mv);

        match mv {
            Move::NewStack => Self {
                score: self.score,
                stack: Vec::new(),
                tableau: self.tableau.clone(),
                history,
            },
            Move::Add { column, card } => {
                // Verify that this card came from the top of the given column
                let top_card = self
                    .peek_column(column)
                    .expect("Given column does not have any cards");
                if *top_card != card {
                    panic!("Given card does not match top card from given column")
                }

                let mut score = self.score;
                // If the first card played to the stack is a Jack: +2 points
                if self.stack.is_empty() && card.rank == Jack {
                    score += 2;
                }
                // If the stack total is exactly 15 or 31: +2 points
                let new_stack_total = self.stack_total() + card.value();
                if new_stack_total == 15 || new_stack_total == 31 {
                    score += 2;
                }
                // If there is a set of 2, 3, or 4 of the same card: +2/+6/+12 points
                let card_matches = self
                    .stack
                    .iter()
                    .rev()
                    .take_while(|c| c.rank == card.rank)
                    .count();
                score += match card_matches {
                    0 => 0,
                    1 => 2,
                    2 => 6,
                    3 => 12,
                    _ => unreachable!("There can' be more than 4 of the same card"),
                };
                // TODO: check for runs

                let mut stack = self.stack.clone();
                stack.push(card);

                let mut tableau = self.tableau.clone();
                tableau[column as usize].pop();

                Self {
                    score,
                    stack,
                    tableau,
                    history,
                }
            }
        }
    }

    fn random_tableau() -> [Vec<Card>; 4] {
        let mut cards = Card::default_deck();

        cards.shuffle(&mut thread_rng());

        let chunk_size = cards.len() / 4;
        [
            cards.split_off(cards.len() - chunk_size),
            cards.split_off(cards.len() - chunk_size),
            cards.split_off(cards.len() - chunk_size),
            cards.split_off(cards.len() - chunk_size),
        ]
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        // Possible actions are:
        // - Playing the top card of each column if the stack value after
        //   playing the card will be less than 31
        // - If there are no cards to play, the only move is to start a new stack
        let mut moves = Vec::new();
        let stack_total = self.stack_total();
        for column in Column::iter() {
            if let Some(card) = self.peek_column(column).copied() {
                if stack_total + card.value() <= 31 {
                    moves.push(Move::Add { column, card });
                }
            }
        }

        if moves.is_empty() {
            moves.push(Move::NewStack);
        }

        moves
    }

    pub fn is_terminal(&self) -> bool {
        self.remaining_cards() == 0
    }

    pub fn display_state(&self) {
        println!("Stack:\n{:?}", self.stack);
        println!("Tableau:");
        for column in Column::iter() {
            println!("{:?}", self.tableau[column as usize]);
        }
    }

    pub fn display_history(&self) {
        println!("History:");
        for mv in &self.history {
            println!("{mv:?}");
        }
    }

    pub fn score(&self) -> u8 {
        self.score
    }

    pub fn stack(&self) -> &Vec<Card> {
        &self.stack
    }

    pub fn tableau(&self) -> &[Vec<Card>; 4] {
        &self.tableau
    }

    pub fn history(&self) -> &Vec<Move> {
        &self.history
    }

    pub fn remaining_cards(&self) -> usize {
        self.tableau.iter().map(|col| col.len()).sum()
    }

    pub fn peek_column(&self, column: Column) -> Option<&Card> {
        self.tableau[column as usize].last()
    }

    pub fn stack_total(&self) -> u8 {
        self.stack.iter().map(|card| card.value()).sum()
    }
}

#[derive(EnumIter, Copy, Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum Column {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum Move {
    NewStack,
    Add { column: Column, card: Card },
}

impl State for CribbageSolitaire {
    type Action = Move;

    fn end_status(&self) -> Option<EndState> {
        if self.is_terminal() {
            if self.score >= 61 {
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
        self.apply_move(*action)
    }

    fn evaluate(&self, _: bool) -> f32 {
        self.score as f32
    }
}

#[cfg(test)]
mod test {
    use crate::cribbage::CribbageSolitaire;
    use crate::states::state_0001;

    #[test]
    fn test_parser() {
        let input = "\
7H  7D  KC  10C
8S  3H  10D 6D
KD  JS  10S 8C
4C  AC  AS  6S
JH  KH  3C  QH
9D  6C  QD  4H
5D  5H  3D  QC
9S  5S  8D  7S
JC  9C  4S  JD
9H  7C  4D  6H
2C  KS  2D  5C
10H AC  3S  QS
2S  AH  8H  2H";
        let state = CribbageSolitaire::from_str(input);
        assert_eq!(state, state_0001());
    }
}
