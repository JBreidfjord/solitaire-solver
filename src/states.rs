use crate::game::{Card, CribbageSolitaire};
use crate::game::Rank::{
    Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two,
};
use crate::game::Suit::{Club, Diamond, Heart, Spade};
use crate::russian::ProletariatsPatience;

pub fn state_0001() -> CribbageSolitaire {
    // TODO: correct the suits
    CribbageSolitaire::new([
        vec![
            Card {
                suit: Heart,
                rank: Seven,
            },
            Card {
                suit: Spade,
                rank: Eight,
            },
            Card {
                suit: Diamond,
                rank: King,
            },
            Card {
                suit: Club,
                rank: Four,
            },
            Card {
                suit: Heart,
                rank: Jack,
            },
            Card {
                suit: Diamond,
                rank: Nine,
            },
            Card {
                suit: Diamond,
                rank: Five,
            },
            Card {
                suit: Spade,
                rank: Nine,
            },
            Card {
                suit: Club,
                rank: Jack,
            },
            Card {
                suit: Heart,
                rank: Nine,
            },
            Card {
                suit: Club,
                rank: Two,
            },
            Card {
                suit: Heart,
                rank: Ten,
            },
            Card {
                suit: Spade,
                rank: Two,
            },
        ],
        vec![
            Card {
                suit: Diamond,
                rank: Seven,
            },
            Card {
                suit: Heart,
                rank: Three,
            },
            Card {
                suit: Spade,
                rank: Jack,
            },
            Card {
                suit: Club,
                rank: Ace,
            },
            Card {
                suit: Heart,
                rank: King,
            },
            Card {
                suit: Club,
                rank: Six,
            },
            Card {
                suit: Heart,
                rank: Five,
            },
            Card {
                suit: Spade,
                rank: Five,
            },
            Card {
                suit: Club,
                rank: Nine,
            },
            Card {
                suit: Club,
                rank: Seven,
            },
            Card {
                suit: Spade,
                rank: King,
            },
            Card {
                suit: Club,
                rank: Ace,
            },
            Card {
                suit: Heart,
                rank: Ace,
            },
        ],
        vec![
            Card {
                suit: Club,
                rank: King,
            },
            Card {
                suit: Diamond,
                rank: Ten,
            },
            Card {
                suit: Spade,
                rank: Ten,
            },
            Card {
                suit: Spade,
                rank: Ace,
            },
            Card {
                suit: Club,
                rank: Three,
            },
            Card {
                suit: Diamond,
                rank: Queen,
            },
            Card {
                suit: Diamond,
                rank: Three,
            },
            Card {
                suit: Diamond,
                rank: Eight,
            },
            Card {
                suit: Spade,
                rank: Four,
            },
            Card {
                suit: Diamond,
                rank: Four,
            },
            Card {
                suit: Diamond,
                rank: Two,
            },
            Card {
                suit: Spade,
                rank: Three,
            },
            Card {
                suit: Heart,
                rank: Eight,
            },
        ],
        vec![
            Card {
                suit: Club,
                rank: Ten,
            },
            Card {
                suit: Diamond,
                rank: Six,
            },
            Card {
                suit: Club,
                rank: Eight,
            },
            Card {
                suit: Spade,
                rank: Six,
            },
            Card {
                suit: Heart,
                rank: Queen,
            },
            Card {
                suit: Heart,
                rank: Four,
            },
            Card {
                suit: Club,
                rank: Queen,
            },
            Card {
                suit: Spade,
                rank: Seven,
            },
            Card {
                suit: Diamond,
                rank: Jack,
            },
            Card {
                suit: Heart,
                rank: Six,
            },
            Card {
                suit: Club,
                rank: Five,
            },
            Card {
                suit: Spade,
                rank: Queen,
            },
            Card {
                suit: Heart,
                rank: Two,
            },
        ],
    ])
}

pub fn russian() -> ProletariatsPatience {
    let state = ProletariatsPatience::new([
        vec![
            Card {
                rank: Seven,
                suit: Heart,
            },
            Card {
                rank: Eight,
                suit: Club,
            },
            Card {
                rank: King,
                suit: Spade,
            },
            Card {
                rank: Nine,
                suit: Spade,
            },
        ],
        vec![
            Card {
                rank: Seven,
                suit: Spade,
            },
            Card {
                rank: Queen,
                suit: Heart,
            },
            Card {
                rank: Jack,
                suit: Diamond,
            },
            Card {
                rank: King,
                suit: Club,
            },
        ],
        vec![
            Card {
                rank: Nine,
                suit: Diamond,
            },
            Card {
                rank: King,
                suit: Diamond,
            },
            Card {
                rank: Nine,
                suit: Club,
            },
            Card {
                rank: Six,
                suit: Diamond,
            },
        ],
        vec![
            Card {
                rank: Ten,
                suit: Heart,
            },
            Card {
                rank: Six,
                suit: Heart,
            },
            Card {
                rank: Queen,
                suit: Diamond,
            },
            Card {
                rank: Seven,
                suit: Diamond,
            },
        ],
        vec![
            Card {
                rank: Seven,
                suit: Club,
            },
            Card {
                rank: Queen,
                suit: Club,
            },
            Card {
                rank: Six,
                suit: Spade,
            },
            Card {
                rank: Ten,
                suit: Spade,
            },
        ],
        vec![
            Card {
                rank: Jack,
                suit: Spade,
            },
            Card {
                rank: Queen,
                suit: Spade,
            },
            Card {
                rank: Ten,
                suit: Club,
            },
            Card {
                rank: Ace,
                suit: Spade,
            },
        ],
        vec![
            Card {
                rank: Eight,
                suit: Heart,
            },
            Card {
                rank: Ace,
                suit: Club,
            },
            Card {
                rank: Jack,
                suit: Heart,
            },
            Card {
                rank: Jack,
                suit: Club,
            },
        ],
        vec![
            Card {
                rank: Six,
                suit: Club,
            },
            Card {
                rank: Ace,
                suit: Heart,
            },
            Card {
                rank: Eight,
                suit: Spade,
            },
            Card {
                rank: Nine,
                suit: Heart,
            },
        ],
        vec![
            Card {
                rank: King,
                suit: Heart,
            },
            Card {
                rank: Eight,
                suit: Diamond,
            },
            Card {
                rank: Ten,
                suit: Diamond,
            },
            Card {
                rank: Ace,
                suit: Diamond,
            },
        ],
    ]);

    state.validate_tableau();
    state
}

pub fn russian_2() -> ProletariatsPatience {
    let state = ProletariatsPatience::new([
        vec![
            Card {
                rank: Six,
                suit: Club,
            },
            Card {
                rank: Queen,
                suit: Diamond,
            },
            Card {
                rank: Nine,
                suit: Club,
            },
            Card {
                rank: Ace,
                suit: Diamond,
            },
        ],
        vec![
            Card {
                rank: Seven,
                suit: Heart,
            },
            Card {
                rank: Jack,
                suit: Diamond,
            },
            Card {
                rank: Ace,
                suit: Club,
            },
            Card {
                rank: Ace,
                suit: Spade,
            },
        ],
        vec![
            Card {
                rank: Eight,
                suit: Club,
            },
            Card {
                rank: Seven,
                suit: Club,
            },
            Card {
                rank: Queen,
                suit: Club,
            },
            Card {
                rank: Ten,
                suit: Diamond,
            },
        ],
        vec![
            Card {
                rank: Eight,
                suit: Spade,
            },
            Card {
                rank: Jack,
                suit: Spade,
            },
            Card {
                rank: Six,
                suit: Spade,
            },
            Card {
                rank: Nine,
                suit: Heart,
            },
        ],
        vec![
            Card {
                rank: King,
                suit: Diamond,
            },
            Card {
                rank: Nine,
                suit: Spade,
            },
            Card {
                rank: Eight,
                suit: Diamond,
            },
            Card {
                rank: Queen,
                suit: Heart,
            },
        ],
        vec![
            Card {
                rank: Ten,
                suit: Club,
            },
            Card {
                rank: Six,
                // suit: Diamond, // maybe heart
                suit: Heart,
            },
            Card {
                rank: Nine,
                suit: Diamond,
            },
            Card {
                rank: King,
                suit: Heart,
            },
        ],
        vec![
            Card {
                rank: Eight,
                suit: Heart,
            },
            Card {
                rank: King,
                suit: Club,
            },
            Card {
                rank: Seven,
                suit: Diamond,
            },
            Card {
                rank: Jack,
                suit: Heart,
            },
        ],
        vec![
            Card {
                rank: Seven,
                suit: Spade,
            },
            Card {
                rank: Queen,
                suit: Spade,
            },
            Card {
                rank: Ace,
                suit: Heart,
            },
            Card {
                rank: Jack,
                suit: Club,
            },
        ],
        vec![
            Card {
                rank: Ten,
                suit: Spade,
            },
            Card {
                rank: Six,
                // suit: Heart, // maybe diamond
                suit: Diamond,
            },
            Card {
                rank: King,
                suit: Spade,
            },
            Card {
                rank: Ten,
                suit: Heart,
            },
        ],
    ]);

    state.validate_tableau();
    state
}
