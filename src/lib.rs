use ionic_deckhandler::{Card, Deck, Rank, Suit};

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

// Evalute for a poker hand.
//
//
pub fn evaluate(hand: &mut [Card; 5]) -> i32 {
    hand.sort();
    let last_index = hand.len() - 1;
    let mut card_matches: [i32; 2] = [0, 0];
    let mut break_in_card_matches: bool = false;
    for card in 0..last_index {
        println!("card {}", card);
        match hand[card] == hand[card + 1] {
            true => match card_matches[0] {
                0 => card_matches[0] += 1,
                _ => {
                    if break_in_card_matches == false {
                        card_matches[0] += 1;
                    } else {
                        card_matches[1] += 1;
                    }
                }
            },
            false => break_in_card_matches = true,
        }
    }

    if card_matches[0] == 0 {
        return -1;
    }

    if card_matches[0] >= 1 && (card_matches[1] == 0) {
        println!("matches 0 {}", card_matches[0]);
        match card_matches[0] {
            1 => return PokerRankOrder::Pair as i32,
            2 => return PokerRankOrder::ThreeOfAKind as i32,
            3 => return PokerRankOrder::FourOfAKind as i32,
            _ => return -100, // More than 3 matches in a 5-card hand would be impossible (unless playing with a wild card)
        }
    }

    // Check for two pair or full house
    if card_matches[0] == card_matches[1] {
        return PokerRankOrder::TwoPair as i32;
    }

    PokerRankOrder::FullHouse as i32
}

#[test]
fn test_evaluate_pair() {
    let mut hand_arr: [Card; 5] = [
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    assert_eq!(evaluate(&mut hand_arr), PokerRankOrder::Pair as i32);
}

#[test]
fn test_evaluate_two_pair() {
    let mut hand_arr: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    assert_eq!(evaluate(&mut hand_arr), PokerRankOrder::TwoPair as i32);
}

#[test]
fn test_evaluate_three_of_a_kind() {
    let mut hand_arr: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Four, Suit::Clubs),
    ];

    assert_eq!(evaluate(&mut hand_arr), PokerRankOrder::ThreeOfAKind as i32);
}

#[test]
fn test_evaluate_full_house() {
    let mut hand_arr: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
    ];

    assert_eq!(evaluate(&mut hand_arr), PokerRankOrder::FullHouse as i32);
}

#[test]
fn test_evaluate_four_of_a_kind() {
    let mut hand_arr: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Four, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
    ];

    assert_eq!(evaluate(&mut hand_arr), PokerRankOrder::FourOfAKind as i32);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
