use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use derivative::Derivative;

use crate::fortune::Suit::{Cups, Pentacles, Swords, Wands};
use crate::fortune::TarotCard::{Major, Minor};
use crate::game::{EndState, Rank, State};
use crate::game::Rank::Ace;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MajorArcana {
    pub rank: u8, // TODO: is this fine as just u8? Might be nice as a better type
}

impl MajorArcana {
    fn can_stack_on(&self, other: &MajorArcana) -> bool {
        self.rank.abs_diff(other.rank) == 1
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Suit {
    Wands,
    Swords,
    Cups,
    Pentacles,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let suit_char = match self {
            // Cups => '\u{2665}',
            // Pentacles => '\u{2666}',
            // Swords => '\u{2664}',
            // Wands => '\u{2667}',
            Cups => 'C',
            Pentacles => 'P',
            Swords => 'S',
            Wands => 'W',
        };
        write!(f, "{suit_char}")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TarotCard {
    Minor(Card),
    Major(MajorArcana),
}

impl TarotCard {
    pub fn is_major(&self) -> bool {
        matches!(self, Major(..))
    }

    pub fn can_stack_on(&self, other: &TarotCard) -> bool {
        match (self, other) {
            (Major(a), Major(b)) => a.can_stack_on(b),
            (Minor(a), Minor(b)) => {
                a.suit == b.suit && (a.rank as usize).abs_diff(b.rank as usize) == 1
            }
            (_, _) => false,
        }
    }
}

impl Display for TarotCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // Since 10 takes up 2 chars while the rest are 1, we right align and pad with an extra space
            Minor(card) => write!(f, "{:>3}", format!("{}{}", card.rank, card.suit)),
            Major(card) => write!(f, "{:>3}", format!("{}", card.rank)),
        }
    }
}

struct MaybeTarotCard(pub Option<TarotCard>);

impl Display for MaybeTarotCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(card) = &self.0 {
            Display::fmt(card, f)
        } else {
            write!(f, "   ")
        }
    }
}

/// Rules:
/// - Cards may be stacked by suit in increasing or decreasing order.
///  - Only one card may be moved at a time.
/// - To win move all cards to the foundations.
///  - The major arcana are built up from 0 and down from 21 until they meet.
///  - The minor arcana are built up by suit from A to K.
/// - A card may be placed above the minor arcana foundation,
///   but will block further minor arcana from moving there.
#[derive(Derivative, Clone, Eq)]
#[derivative(Debug, Hash, PartialEq)]
pub struct FortunesFoundation {
    minor_foundations: [Vec<Card>; 4],
    major_foundations: [Vec<MajorArcana>; 2],
    tableau: [Vec<TarotCard>; 11],
    free_cell: Option<TarotCard>,
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    #[derivative(Debug = "ignore")]
    history: Vec<Move>,
}
// TODO: try a custom impl of Hash that makes a compressed representation to take advantage of symmetry
// e.g. empty columns always on right since it doesn't matter _where_ the column is

impl FortunesFoundation {
    pub fn new(tableau: [Vec<TarotCard>; 11]) -> Self {
        Self {
            tableau,
            ..Self::default()
        }
    }

    fn default() -> Self {
        Self {
            minor_foundations: [
                vec![Card {
                    rank: Ace,
                    suit: Pentacles,
                }],
                vec![Card {
                    rank: Ace,
                    suit: Cups,
                }],
                vec![Card {
                    rank: Ace,
                    suit: Wands,
                }],
                vec![Card {
                    rank: Ace,
                    suit: Swords,
                }],
            ],
            major_foundations: [vec![], vec![]],
            tableau: [
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            free_cell: None,
            history: vec![],
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.tableau.iter().flatten().collect::<Vec<_>>().is_empty()
            || self.possible_actions().is_empty()
    }

    pub fn is_win(&self) -> bool {
        // We've won if all of the foundations are filled
        // TODO: should check if they're ordered too, but this is simpler for now
        for foundation in self.minor_foundations.iter() {
            if foundation.len() != 13 {
                return false;
            }
        }
        let major_foundation = self.major_foundations.iter().flatten().collect::<Vec<_>>();
        major_foundation.len() == 22
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Move {
    Stack {
        card: TarotCard,
        from: usize,
        to: usize,
    },
    Build {
        card: TarotCard,
        from: usize,
    },
    Free {
        card: TarotCard,
        from: usize,
    },
    Unfree {
        card: TarotCard,
        to: usize,
    },
}

impl Move {
    fn is_free(&self) -> bool {
        matches!(self, Move::Free { .. })
    }

    fn is_unfree(&self) -> bool {
        matches!(self, Move::Unfree { .. })
    }
}

impl State for FortunesFoundation {
    type Action = Move;

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

    /// Legal moves:
    /// - All top cards can be freed if the free cell is empty
    /// - A top card can move from one column to another if it is stackable on the destination
    /// - A top card can move to the foundation if it is +1 rank of the foundation of its suit
    ///  - Major arcana is a bit of a special case since it counts up _and_ down in 2 foundations
    /// - A free card can be unfreed if it can stack on any top card
    fn possible_actions(&self) -> Vec<Self::Action> {
        let mut moves = Vec::new();

        // Flag for only adding one instance of unfreeing to an empty col, since they're identical
        let mut added_empty_column_unfree = false;
        for (from_idx, from_col) in self.tableau.iter().enumerate() {
            if from_col.is_empty() {
                if !added_empty_column_unfree {
                    if let Some(free_card) = self.free_cell {
                        moves.push(Move::Unfree {
                            card: free_card,
                            to: from_idx,
                        });
                        added_empty_column_unfree = true;
                    }
                }

                continue;
            }
            let top_card = from_col.last().expect("Vec was checked to be non-empty");

            // Add moves related to the free cell
            match self.free_cell {
                Some(card) => {
                    if card.can_stack_on(top_card) {
                        moves.push(Move::Unfree { card, to: from_idx })
                    }
                }
                None => moves.push(Move::Free {
                    card: *top_card,
                    from: from_idx,
                }),
            }

            // Add moves related to building foundations
            // Since these moves are automatically performed in the real game, if we come across
            // one here, we'll return _only_ that move to force it
            // TODO: these shouldn't be moves, we should just replicate the auto-build
            match top_card {
                Major(inner) => {
                    // TODO: these conditions could be merged since the body is duplicated
                    // Add to the left major foundation if this is the first card or
                    // its rank is 1 rank higher than the current top of the foundation
                    if inner.rank == 0
                        || self.major_foundations[0]
                            .last()
                            .is_some_and(|fc| inner.rank as usize == fc.rank as usize + 1)
                    {
                        return vec![Move::Build {
                            card: *top_card,
                            from: from_idx,
                        }];
                    } else if inner.rank == 21
                        || self.major_foundations[1]
                            .last()
                            .is_some_and(|fc| inner.rank as usize == fc.rank as usize - 1)
                    {
                        // Similarly for the right, except rank 21 or 1 rank lower than current
                        return vec![Move::Build {
                            card: *top_card,
                            from: from_idx,
                        }];
                    }
                }

                Minor(inner) => {
                    // We can only move minor cards to the foundation if it is unblocked
                    // (free cell blocks foundations)
                    if self.free_cell.is_none() {
                        for foundation in self.minor_foundations.iter() {
                            let foundation_card = foundation
                                .last()
                                .expect("Minor foundation should be non-empty");
                            if inner.suit == foundation_card.suit
                                && inner.rank as usize == foundation_card.rank as usize + 1
                            {
                                return vec![Move::Build {
                                    card: *top_card,
                                    from: from_idx,
                                }];
                            }
                        }
                    }
                }
            }

            // Add moves related to moving cards between columns
            // Flag for only adding 1 move to an empty column, since they're all identical
            let mut added_empty_column_move = false;
            for (to_idx, to_col) in self.tableau.iter().enumerate() {
                // Skip current column
                if from_idx == to_idx {
                    continue;
                }

                // We don't want moves that just move a stack from one empty column to another,
                // so if this stack is the entire column (and therefore would leave it empty),
                // and the dest is empty, we skip
                // if to_col.is_empty() && stack_is_entire_col {
                //     continue;
                // }

                if to_col.is_empty() {
                    // We don't want moves that just move the last card from one stack to another empty stack,
                    // so we only add the move if there is more than this card in the stack
                    if !added_empty_column_move && from_col.len() > 1 {
                        moves.push(Move::Stack {
                            card: *top_card,
                            from: from_idx,
                            to: to_idx,
                        });
                        added_empty_column_move = true;
                    }
                    continue;
                }
                let dest_card = to_col.last().expect("Vec checked to be non-empty");

                if top_card.can_stack_on(dest_card) {
                    moves.push(Move::Stack {
                        card: *top_card,
                        from: from_idx,
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
                .take(11)
                .collect::<Vec<_>>()
                .contains(&m)
        });

        if !self.history.is_empty() {
            let last_move = self
                .history
                .last()
                .expect("History was checked to be non-empty");

            // Remove moves that are just moving the same cards as the last move
            let last_card = match last_move {
                Move::Free { card, .. }
                | Move::Unfree { card, .. }
                | Move::Stack { card, .. }
                | Move::Build { card, .. } => card,
            };
            moves.retain(|m| {
                let card = match m {
                    Move::Free { card, .. }
                    | Move::Unfree { card, .. }
                    | Move::Stack { card, .. }
                    | Move::Build { card, .. } => card,
                };
                card != last_card
            });

            // If we just freed a card, we can't unfree it immediately
            if last_move.is_free() {
                moves.retain(|m| !m.is_unfree());
            }
        }

        // TODO: similarly we can remove moves that are reversions of the previous move
        // TODO: Should we remove _all_ moves that could just be undone next turn?

        // Sort descending so the "best" moves are at the front
        moves.sort_unstable();
        moves.reverse();

        moves
    }

    fn act(&self, action: &Self::Action) -> Self {
        let mut history = self.history.clone();
        history.push(action.clone());

        let mut tableau = self.tableau.clone();
        let mut new_state = match *action {
            Move::Free { card, from } => {
                tableau[from]
                    .pop()
                    .expect("Tableau column to be non-empty when freeing card");
                Self {
                    free_cell: Some(card),
                    tableau,
                    minor_foundations: self.minor_foundations.clone(),
                    major_foundations: self.major_foundations.clone(),
                    history,
                }
            }

            Move::Unfree { card, to } => {
                tableau[to].push(card);
                Self {
                    free_cell: None,
                    tableau,
                    minor_foundations: self.minor_foundations.clone(),
                    major_foundations: self.major_foundations.clone(),
                    history,
                }
            }

            Move::Build { card, from } => {
                tableau[from].pop().unwrap();
                let mut major_foundations = self.major_foundations.clone();
                let mut minor_foundations = self.minor_foundations.clone();
                match card {
                    Major(card) => {
                        // Decide which foundation to stack on (left/right)
                        if card.rank == 0
                            || major_foundations[0]
                                .last()
                                .is_some_and(|fc| card.can_stack_on(fc))
                        {
                            major_foundations[0].push(card);
                        } else if card.rank == 21
                            || major_foundations[1]
                                .last()
                                .is_some_and(|fc| card.can_stack_on(fc))
                        {
                            major_foundations[1].push(card);
                        }
                    }

                    Minor(inner) => {
                        // Find the foundation that this card can stack on
                        for foundation in minor_foundations.iter_mut() {
                            let top_card = foundation
                                .last()
                                .expect("Minor foundation should be non-empty");
                            if inner.suit == top_card.suit
                                && inner.rank as usize == top_card.rank as usize + 1
                            {
                                foundation.push(inner);
                            }
                        }
                    }
                }

                Self {
                    major_foundations,
                    minor_foundations,
                    tableau,
                    free_cell: self.free_cell,
                    history,
                }
            }

            Move::Stack { card, from, to } => {
                tableau[from]
                    .pop()
                    .expect("Tableau column to be non-empty when moving card");
                tableau[to].push(card);
                Self {
                    tableau,
                    minor_foundations: self.minor_foundations.clone(),
                    major_foundations: self.major_foundations.clone(),
                    free_cell: self.free_cell,
                    history,
                }
            }
        };

        // TODO: Check if any cards can be moved to the foundation
        // We'll need to restart the check every time one completes, since moving a card could
        // make another move valid
        // for column in new_state.tableau.iter_mut() {
        //     for card in column {
        //
        //     }
        // }

        new_state
    }

    fn evaluate(&self, print_components: bool) -> f32 {
        if self.is_win() {
            return (10_000 - self.history.len()) as f32;
        }

        let mut score = 0.;
        for foundation in self.minor_foundations.iter() {
            score += foundation.len() as f32 * 7.5;
        }
        score += self
            .major_foundations
            .iter()
            .flatten()
            .collect::<Vec<_>>()
            .len() as f32
            * 7.5;

        let mut stack_size = 0;
        let mut depth_error = 0;
        for column in &self.tableau {
            for cards in column.windows(2).rev() {
                if cards[1].can_stack_on(&cards[0]) {
                    stack_size += 1;
                    // slightly higher score for desc stack (10, 9, 8..)
                    match (cards[1], cards[0]) {
                        (Major(_), Major(_)) => (),
                        (Minor(card_1), Minor(card_0)) => {
                            if card_1.rank as usize == card_0.rank as usize - 1 {
                                stack_size += 1;
                            }
                        }
                        (_, _) => unreachable!("Cards must be matching type to stack"),
                    }
                } else {
                    break;
                }
            }

            for (depth, card) in column.iter().rev().enumerate() {
                match card {
                    Major(..) => (),
                    Minor(inner) => depth_error += depth.abs_diff(inner.rank as usize - 2),
                }
            }
        }
        score += stack_size as f32 * 0.5;
        if print_components {
            println!("{depth_error}");
        }
        score -= depth_error as f32 * 0.0;

        let empty_columns = self
            .tableau
            .iter()
            .filter(|column| column.is_empty())
            .count();
        score += empty_columns as f32 * 3.;

        if self.free_cell.is_none() {
            score += 5.;
        }

        score += self.possible_actions().len() as f32 * 0.25;

        // Subtract score for each remaining card
        score -= self.tableau.iter().map(|c| c.iter().count()).sum::<usize>() as f32;

        score
    }
}

impl Display for FortunesFoundation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_col_len = self
            .tableau
            .iter()
            .map(|col| col.len())
            .max()
            .expect("Tableau should not be fully empty");

        println!(
            "{}  \u{21CB}  {}          {} {} {} {}\n",
            MaybeTarotCard(self.major_foundations[0].last().map(|a| Major(*a))),
            MaybeTarotCard(self.major_foundations[1].last().map(|a| Major(*a))),
            Minor(
                *self.minor_foundations[0]
                    .last()
                    .expect("Minor foundation is never empty")
            ),
            Minor(
                *self.minor_foundations[1]
                    .last()
                    .expect("Minor foundation is never empty")
            ),
            Minor(
                *self.minor_foundations[2]
                    .last()
                    .expect("Minor foundation is never empty")
            ),
            Minor(
                *self.minor_foundations[3]
                    .last()
                    .expect("Minor foundation is never empty")
            ),
        );

        for i in 0..max_col_len {
            let row = format!(
                "{} {} {} {} {} {} {} {} {} {} {}",
                MaybeTarotCard(self.tableau[0].get(i).copied()),
                MaybeTarotCard(self.tableau[1].get(i).copied()),
                MaybeTarotCard(self.tableau[2].get(i).copied()),
                MaybeTarotCard(self.tableau[3].get(i).copied()),
                MaybeTarotCard(self.tableau[4].get(i).copied()),
                MaybeTarotCard(self.tableau[5].get(i).copied()),
                MaybeTarotCard(self.tableau[6].get(i).copied()),
                MaybeTarotCard(self.tableau[7].get(i).copied()),
                MaybeTarotCard(self.tableau[8].get(i).copied()),
                MaybeTarotCard(self.tableau[9].get(i).copied()),
                MaybeTarotCard(self.tableau[10].get(i).copied()),
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

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        // Building foundations > moving cards > unfree > free
        match self {
            Move::Free { .. } => match other {
                Move::Free { .. } => Ordering::Equal,
                _ => Ordering::Less,
            },
            Move::Unfree { .. } => match other {
                Move::Unfree { .. } => Ordering::Equal,
                Move::Free { .. } => Ordering::Greater,
                _ => Ordering::Less,
            },
            Move::Stack { .. } => match other {
                Move::Stack { .. } => Ordering::Equal,
                Move::Build { .. } => Ordering::Less,
                _ => Ordering::Greater,
            },
            Move::Build { .. } => match other {
                Move::Build { .. } => Ordering::Equal,
                _ => Ordering::Greater,
            },
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // We add 1 to convert from 0-based index to 1-based index
        match self {
            Move::Free { card, from } => write!(f, "{}->\u{1F0A0}\t{card}", from + 1),
            Move::Unfree { card, to } => write!(f, "\u{1F0A0}->{}\t{card}", to + 1),
            Move::Build { card, from } => write!(f, "{}->Foundation\t{card}", from + 1),
            Move::Stack { card, from, to } => write!(f, "{}->{}\t{card}", from + 1, to + 1),
        }
    }
}

pub fn display_moves(moves: &Vec<Move>) {
    if moves.len() <= 5 {
        for (i, mv) in moves.iter().enumerate() {
            println!("{i}: {mv}");
        }
    } else {
        let left_col_width = moves
            .chunks(2)
            .map(|moves| moves[0].to_string().len())
            .max()
            .expect("Vec should have at least one chunk")
            - 3;
        for moves in moves.iter().enumerate().collect::<Vec<_>>().chunks(2) {
            match moves.len() {
                1 => println!("{}: {}", moves[0].0, moves[0].1),
                2 => println!(
                    "{}: {:<left_col_width$} | {}: {}",
                    moves[0].0,
                    format!("{}", moves[0].1),
                    moves[1].0,
                    moves[1].1,
                ),
                _ => unreachable!("Chunk size is 2"),
            };
        }
    }
}
