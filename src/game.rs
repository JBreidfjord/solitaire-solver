use std::fmt::{Debug, Formatter};
use std::hash::Hash;

use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use Rank::{Ace, Jack, King, Queen};

#[derive(EnumIter, Copy, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl Debug for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let suit_char = match self {
            Suit::Heart => '\u{2665}',
            Suit::Diamond => '\u{2666}',
            Suit::Spade => '\u{2664}',
            Suit::Club => '\u{2667}',
        };
        write!(f, "{suit_char}")
    }
}

#[derive(EnumIter, Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack,
    Queen,
    King,
}

impl Debug for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rank = match self {
            Ace => "A",
            Jack => "J",
            Queen => "Q",
            King => "K",
            _ => &format!("{}", *self as usize),
        };
        write!(f, "{rank}")
    }
}

#[derive(Eq, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn value(&self) -> u8 {
        match self.rank {
            Jack | Queen | King => 10,
            _ => self.rank as u8,
        }
    }

    pub fn color(&self) -> Color {
        match self.suit {
            Suit::Club | Suit::Spade => Color::Black,
            Suit::Diamond | Suit::Heart => Color::Red,
        }
    }

    pub fn default_deck() -> Vec<Card> {
        Suit::iter()
            .flat_map(|suit| Rank::iter().map(move |rank| Card { suit, rank }))
            .collect::<Vec<_>>()
    }

    pub fn is_face(&self) -> bool {
        self.rank >= Jack
    }

    pub fn is_face_or_ace(&self) -> bool {
        self.rank == Ace || self.is_face()
    }

    pub fn can_stack_on(&self, other: &Card) -> bool {
        // TODO: This can change based on the game
        match (self.is_face_or_ace(), other.is_face_or_ace()) {
            (true, true) => self.suit == other.suit, // Face cards can stack by suit
            (false, false) => {
                self.color() != other.color() && self.rank as usize == (other.rank as usize) - 1
            } // Number cards stack by alternating color and decreasing value
            _ => false,
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.rank, self.suit)
    }
}

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

    pub fn revert_move(&self, mv: Move) -> Self {
        todo!()
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

pub enum EndState {
    Win(u8),
    Loss(u8),
}

pub struct Player;

impl mctser::EndStatus for EndState {}
impl mctser::Action for Move {}

impl mctser::Player<EndState> for Player {
    fn reward_when_outcome_is(&self, outcome: &EndState) -> f32 {
        match outcome {
            EndState::Win(score) => *score as f32,
            EndState::Loss(score) => *score as f32,
        }
    }
}

impl mctser::GameState<Player, EndState, Move> for CribbageSolitaire {
    fn player(&self) -> Player {
        Player
    }

    fn end_status(&self) -> Option<EndState> {
        if self.is_terminal() {
            if self.score >= 61 {
                Some(EndState::Win(self.score))
            } else {
                Some(EndState::Loss(self.score))
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
}

pub trait State<P, E, A>: mctser::GameState<P, E, A> + Eq + Hash
where
    P: mctser::Player<E>,
    E: mctser::EndStatus,
    A: mctser::Action,
{
    fn evaluate(&self) -> f32;

    fn revert(&self, action: &A) -> Self;
}

impl State<Player, EndState, Move> for CribbageSolitaire {
    fn evaluate(&self) -> f32 {
        self.score as f32
    }

    fn revert(&self, action: &Move) -> Self {
        self.revert_move(*action)
    }
}
