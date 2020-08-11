use ionic_deckhandler::{card_type, suit, Card, Deck};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerRankOrder {
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

/*
impl PokerRankOrder {
    fn rank_value(&self) -> i32 {
        match *self {
            PokerRankOrder::Pair => 1,
            PokerRankOrder::TwoPair => 2,
            PokerRankOrder::ThreeOfAKind => 3,
            PokerRankOrder::Straight => 4,
            PokerRankOrder::Flush => 5,
            PokerRankOrder::FullHouse => 6,
            PokerRankOrder::FourOfAKind => 7,
            PokerRankOrder::StraightFlush => 8,
            PokerRankOrder::RoyalFlush => 9,
        }
    }
}
*/

// This implementation was suggested by #![feature(Brainlet)] on Discord
impl PokerRankOrder {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Pair => "Pair",
            Self::TwoPair => "Two of a kind",
            Self::ThreeOfAKind => "Three of a kind",
            Self::Straight => "Straight",
            Self::Flush => "Flush",
            Self::FullHouse => "Full House",
            Self::FourOfAKind => "Four of a kind",
            Self::StraightFlush => "Straight Flush",
            Self::RoyalFlush => "Royal Flush",
        }
    }
}
/*
fn main() {
    let my_rank = PokerRankOrder::Pair;
    println!("{}", my_rank.name());
}
*/

/*
struct PokerRank(&'static str);

const NUM_OF_RANKS: i32 = 9;

impl PokerRank {
    pub fn new(lit: &'static str) -> Self {
        Self { 0: lit }
    }

    pub fn init_ranks() -> [PokerRank; NUM_OF_RANKS as usize] {
        [
            PokerRank::new("Pair"),
            PokerRank::new("Two of a kind"),
            PokerRank::new("Three of a kind"),
            PokerRank::new("Straight"),
            PokerRank::new("Flush"),
            PokerRank::new("Full House"),
            PokerRank::new("Four of a kind"),
            PokerRank::new("Straight Flush"),
            PokerRank::new("Royal Flush"),
        ]
    }
}
*/

pub fn evaluate(hand: &[Card; 5]) -> i32 {
    //
    // existing algorithm in C, but probably will be done differently
    // https://github.com/theimpossibleastronaut/aa-pokerhands/blob/master/src/functions.c#L155-L187
    5
}

#[test]
fn test_evaluate() {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    let mut hand_arr: [Card; 5] = [
        Card::new(&suit::CLUB, &card_type::ACE),
        Card::new(&suit::HEART, &card_type::THREE),
        Card::new(&suit::DIAMOND, &card_type::THREE),
        Card::new(&suit::CLUB, &card_type::KING),
        Card::new(&suit::CLUB, &card_type::QUEEN),
    ];

    //
    // Quicksort functions
    // https://turreta.com/2019/10/22/quicksort-algorithm-example-in-rust/
    // let last_index = hand_array.len() - 1;

    println!();
    println!("{}", hand_arr[2].value);
    println!();
    assert!(PokerRankOrder::Flush == PokerRankOrder::Flush)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
