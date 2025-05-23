use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use Rank::{Ace, Jack, King, Queen};

use crate::game::Rank::{Eight, Five, Four, Nine, Seven, Six, Ten, Three, Two};
use crate::game::Suit::{Club, Diamond, Heart, Spade};

#[derive(EnumIter, Debug, Copy, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl TryFrom<String> for Suit {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "H" => Ok(Heart),
            "D" => Ok(Diamond),
            "C" => Ok(Club),
            "S" => Ok(Spade),
            _ => Err(format!("Could not find conversion case for input {value}")),
        }
    }
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

impl TryFrom<String> for Rank {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "A" => Ok(Ace),
            "J" => Ok(Jack),
            "Q" => Ok(Queen),
            "K" => Ok(King),
            "2" => Ok(Two),
            "3" => Ok(Three),
            "4" => Ok(Four),
            "5" => Ok(Five),
            "6" => Ok(Six),
            "7" => Ok(Seven),
            "8" => Ok(Eight),
            "9" => Ok(Nine),
            "10" => Ok(Ten),
            _ => Err(format!("Could not find conversion case for input {value}")),
        }
    }
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

impl EndState {
    pub fn is_win(&self) -> bool {
        matches!(self, EndState::Win)
    }
}

pub trait State: Eq + Hash {
    type Action: Clone + Debug;

    fn end_status(&self) -> Option<EndState>;

    fn possible_actions(&self) -> Vec<Self::Action>;

    fn act(&self, action: &Self::Action) -> Self;

    fn evaluate(&self, print_components: bool) -> f32;
}
