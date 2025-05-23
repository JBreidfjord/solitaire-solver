use crate::cribbage::CribbageSolitaire;
use crate::fortune;
use crate::fortune::{FortunesFoundation, MajorArcana};
use crate::fortune::Suit::{Cups, Pentacles, Swords, Wands};
use crate::fortune::TarotCard::{Major, Minor};
use crate::game::Card;
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
                suit: Diamond, // maybe heart
                               // suit: Heart,
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
                suit: Heart, // maybe diamond
                             // suit: Diamond,
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

pub fn russian_3() -> ProletariatsPatience {
    let state = ProletariatsPatience::new([
        vec![
            Card {
                rank: Jack,
                suit: Club,
            },
            Card {
                rank: King,
                suit: Heart,
            },
            Card {
                rank: Nine,
                suit: Club,
            },
            Card {
                rank: Queen,
                suit: Spade,
            },
        ],
        vec![
            Card {
                rank: Nine,
                suit: Heart,
            },
            Card {
                rank: Ten,
                suit: Spade,
            },
            Card {
                rank: Ace,
                suit: Heart,
            },
            Card {
                rank: Seven,
                suit: Heart,
            },
        ],
        vec![
            Card {
                rank: Ace,
                suit: Diamond,
            },
            Card {
                rank: Six,
                suit: Club, // maybe spade
                            // suit: Spade,
            },
            Card {
                rank: King,
                suit: Diamond,
            },
            Card {
                rank: Nine,
                suit: Spade,
            },
        ],
        vec![
            Card {
                rank: Six,
                suit: Heart,
            },
            Card {
                rank: Eight,
                suit: Club,
            },
            Card {
                rank: Six,
                suit: Diamond,
            },
            Card {
                rank: Queen,
                suit: Club,
            },
        ],
        vec![
            Card {
                rank: Seven,
                suit: Diamond,
            },
            Card {
                rank: King,
                suit: Spade,
            },
            Card {
                rank: Ten,
                suit: Diamond,
            },
            Card {
                rank: Ten,
                suit: Heart,
            },
        ],
        vec![
            Card {
                rank: Ace,
                suit: Spade,
            },
            Card {
                rank: Eight,
                suit: Spade,
            },
            Card {
                rank: Jack,
                suit: Spade,
            },
            Card {
                rank: Nine,
                suit: Diamond,
            },
        ],
        vec![
            Card {
                rank: Jack,
                suit: Heart,
            },
            Card {
                rank: Eight,
                suit: Diamond,
            },
            Card {
                rank: Queen,
                suit: Heart,
            },
            Card {
                rank: King,
                suit: Club,
            },
        ],
        vec![
            Card {
                rank: Ten,
                suit: Club,
            },
            Card {
                rank: Six,
                suit: Spade, // maybe club
                             // suit: Club,
            },
            Card {
                rank: Jack,
                suit: Diamond,
            },
            Card {
                rank: Queen,
                suit: Diamond,
            },
        ],
        vec![
            Card {
                rank: Seven,
                suit: Club,
            },
            Card {
                rank: Ace,
                suit: Club,
            },
            Card {
                rank: Seven,
                suit: Spade,
            },
            Card {
                rank: Eight,
                suit: Heart,
            },
        ],
    ]);

    state.validate_tableau();
    state
}

pub fn fortune_1() -> FortunesFoundation {
    FortunesFoundation::new([
        vec![
            Minor(fortune::Card {
                rank: Ten,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 17 }),
            Minor(fortune::Card {
                rank: Five,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 3 }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Wands,
            }),
        ],
        vec![
            Major(MajorArcana { rank: 20 }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 5 }),
        ],
        vec![
            Major(MajorArcana { rank: 21 }),
            Minor(fortune::Card {
                rank: Six,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Three,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Three,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 14 }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Two,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 19 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 18 }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Wands,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Six,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 15 }),
            Major(MajorArcana { rank: 9 }),
            Minor(fortune::Card {
                rank: Two,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Three,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Swords,
            }),
        ],
        vec![
            // Empty column
        ],
        vec![
            Minor(fortune::Card {
                rank: Three,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Four,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 2 }),
            Major(MajorArcana { rank: 10 }),
        ],
        vec![
            Major(MajorArcana { rank: 4 }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Pentacles,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Five,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Four,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 1 }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 8 }),
            Major(MajorArcana { rank: 12 }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Seven,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 11 }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 13 }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Cups,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Four,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 16 }),
            Major(MajorArcana { rank: 6 }),
            Major(MajorArcana { rank: 0 }),
            Major(MajorArcana { rank: 7 }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Wands,
            }),
        ],
    ])
}

pub fn fortune_2() -> FortunesFoundation {
    FortunesFoundation::new([
        vec![
            Major(MajorArcana { rank: 9 }),
            Major(MajorArcana { rank: 3 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 4 }),
            Major(MajorArcana { rank: 20 }),
            Minor(fortune::Card {
                rank: King,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Cups,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Five,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Three,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 8 }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Wands,
            }),
        ],
        vec![
            Major(MajorArcana { rank: 18 }),
            Major(MajorArcana { rank: 14 }),
            Minor(fortune::Card {
                rank: Six,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Wands,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Three,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 1 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 6 }),
            Major(MajorArcana { rank: 21 }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Pentacles,
            }),
        ],
        vec![
            Major(MajorArcana { rank: 5 }),
            Major(MajorArcana { rank: 11 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 12 }),
        ],
        vec![
            // Empty column
        ],
        vec![
            Minor(fortune::Card {
                rank: Ten,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 13 }),
            Minor(fortune::Card {
                rank: Two,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 0 }),
            Minor(fortune::Card {
                rank: Three,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 15 }),
        ],
        vec![
            Major(MajorArcana { rank: 2 }),
            Minor(fortune::Card {
                rank: Three,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Pentacles,
            }),
        ],
        vec![
            Major(MajorArcana { rank: 10 }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 19 }),
            Minor(fortune::Card {
                rank: King,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Four,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 16 }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Pentacles,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Ten,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 7 }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Wands,
            }),
        ],
        vec![
            Major(MajorArcana { rank: 17 }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Cups,
            }),
        ],
    ])
}

pub fn fortune_3() -> FortunesFoundation {
    FortunesFoundation::new([
        vec![
            Major(MajorArcana { rank: 17 }),
            Major(MajorArcana { rank: 9 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 5 }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Cups,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Three,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 19 }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Seven,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 10 }),
            Major(MajorArcana { rank: 11 }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 14 }),
            Major(MajorArcana { rank: 8 }),
        ],
        vec![
            Major(MajorArcana { rank: 16 }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 4 }),
            Minor(fortune::Card {
                rank: Two,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Swords,
            }),
        ],
        vec![
            Major(MajorArcana { rank: 7 }),
            Major(MajorArcana { rank: 1 }),
            Major(MajorArcana { rank: 0 }),
            Minor(fortune::Card {
                rank: Five,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 21 }),
            Minor(fortune::Card {
                rank: King,
                suit: Cups,
            }),
        ],
        vec![
            // Empty column
        ],
        vec![
            Minor(fortune::Card {
                rank: Nine,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 13 }),
            Minor(fortune::Card {
                rank: Three,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 20 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 18 }),
            Major(MajorArcana { rank: 12 }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Jack,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Four,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Cups,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Queen,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 15 }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 3 }),
            Minor(fortune::Card {
                rank: Six,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Pentacles,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Two,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 2 }),
            Minor(fortune::Card {
                rank: Three,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Wands,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Three,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 6 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Cups,
            }),
        ],
    ])
}

pub fn fortune_4() -> FortunesFoundation {
    FortunesFoundation::new([
        vec![
            Major(MajorArcana { rank: 18 }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 9 }),
            Minor(fortune::Card {
                rank: King,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Pentacles,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Six,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Three,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 11 }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Four,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 16 }),
        ],
        vec![
            Major(MajorArcana { rank: 8 }),
            Major(MajorArcana { rank: 13 }),
            Minor(fortune::Card {
                rank: Three,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 5 }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Pentacles,
            }),
        ],
        vec![
            Major(MajorArcana { rank: 19 }),
            Major(MajorArcana { rank: 6 }),
            Major(MajorArcana { rank: 10 }),
            Minor(fortune::Card {
                rank: Five,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 15 }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Pentacles,
            }),
        ],
        vec![
            Major(MajorArcana { rank: 1 }),
            Minor(fortune::Card {
                rank: King,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 14 }),
            Major(MajorArcana { rank: 3 }),
            Minor(fortune::Card {
                rank: Three,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Pentacles,
            }),
        ],
        vec![
            // Empty column
        ],
        vec![
            Minor(fortune::Card {
                rank: Four,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 2 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 7 }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Pentacles,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Ten,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 21 }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Cups,
            }),
        ],
        vec![
            Major(MajorArcana { rank: 20 }),
            Minor(fortune::Card {
                rank: King,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 4 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Swords,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Two,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 17 }),
            Minor(fortune::Card {
                rank: King,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Swords,
            }),
        ],
        vec![
            Major(MajorArcana { rank: 12 }),
            Major(MajorArcana { rank: 0 }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Three,
                suit: Wands,
            }),
        ],
    ])
}

pub fn fortune_5() -> FortunesFoundation {
    FortunesFoundation::new([
        vec![
            Minor(fortune::Card {
                rank: Queen,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 10 }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 18 }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Three,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Four,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 2 }),
            Major(MajorArcana { rank: 11 }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Nine,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 16 }),
            Major(MajorArcana { rank: 5 }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Pentacles,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Two,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Wands,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Three,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 20 }),
            Major(MajorArcana { rank: 4 }),
            Minor(fortune::Card {
                rank: Two,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 21 }),
            Major(MajorArcana { rank: 17 }),
        ],
        vec![
            // Empty column
        ],
        vec![
            Minor(fortune::Card {
                rank: Six,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 19 }),
            Major(MajorArcana { rank: 3 }),
            Minor(fortune::Card {
                rank: Three,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 6 }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Three,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 14 }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 15 }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Wands,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Four,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 1 }),
            Minor(fortune::Card {
                rank: King,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 8 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Wands,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Four,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 13 }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 7 }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Swords,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Eight,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 0 }),
            Major(MajorArcana { rank: 9 }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 12 }),
            Minor(fortune::Card {
                rank: King,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Wands,
            }),
        ],
    ])
}

pub fn fortune_6() -> FortunesFoundation {
    FortunesFoundation::new([
        vec![
            Major(MajorArcana { rank: 12 }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 18 }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 7 }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Six,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 19 }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Two,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Four,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 11 }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Wands,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Five,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 8 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 9 }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Queen,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 20 }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Seven,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 21 }),
            Minor(fortune::Card {
                rank: Three,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 1 }),
            Minor(fortune::Card {
                rank: Three,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 2 }),
        ],
        vec![
            // Empty column
        ],
        vec![
            Minor(fortune::Card {
                rank: Queen,
                suit: Pentacles,
            }),
            Major(MajorArcana { rank: 13 }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 10 }),
            Minor(fortune::Card {
                rank: Seven,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Pentacles,
            }),
        ],
        vec![
            Major(MajorArcana { rank: 14 }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Six,
                suit: Pentacles,
            }),
            Minor(fortune::Card {
                rank: Four,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: King,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Two,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 4 }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Queen,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Three,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Jack,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Ten,
                suit: Cups,
            }),
            Major(MajorArcana { rank: 0 }),
            Minor(fortune::Card {
                rank: Four,
                suit: Swords,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: King,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Three,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Five,
                suit: Cups,
            }),
            Minor(fortune::Card {
                rank: Nine,
                suit: Wands,
            }),
            Major(MajorArcana { rank: 15 }),
            Minor(fortune::Card {
                rank: King,
                suit: Wands,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Cups,
            }),
        ],
        vec![
            Minor(fortune::Card {
                rank: Five,
                suit: Swords,
            }),
            Minor(fortune::Card {
                rank: Eight,
                suit: Swords,
            }),
            Major(MajorArcana { rank: 16 }),
            Major(MajorArcana { rank: 5 }),
            Major(MajorArcana { rank: 6 }),
            Major(MajorArcana { rank: 3 }),
            Major(MajorArcana { rank: 17 }),
        ],
    ])
}
