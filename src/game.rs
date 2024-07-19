use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use Rank::{Ace, Jack, King, Queen};

#[derive(EnumIter, Debug, Copy, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl Display for Suit {
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

#[derive(EnumIter, Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
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

impl Display for Rank {
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

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
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

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Since 10 takes up 2 chars while the rest are 1, we right align and pad with an extra space
        write!(f, "{:>3}", format!("{}{}", self.rank, self.suit))
    }
}

pub struct MaybeCard(pub Option<Card>);

impl Display for MaybeCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(card) = &self.0 {
            Display::fmt(card, f)
        } else {
            write!(f, "   ")
        }
    }
}

pub enum EndState {
    Win,
    Loss,
}

pub trait State: Eq + Hash {
    type Action: Clone + Debug;

    fn end_status(&self) -> Option<EndState>;

    fn possible_actions(&self) -> Vec<Self::Action>;

    fn act(&self, action: &Self::Action) -> Self;

    fn evaluate(&self, print_components: bool) -> f32;
}
