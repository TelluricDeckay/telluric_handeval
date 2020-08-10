use ionic_deckhandler::{card_type, suit, Card, Deck};

enum PokerRank {
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

impl PokerRank {
    fn rank_value(&self) -> i32 {
        match *self {
            PokerRank::Pair => 1,
            PokerRank::TwoPair => 2,
            PokerRank::ThreeOfAKind => 3,
            PokerRank::Straight => 4,
            PokerRank::Flush => 5,
            PokerRank::FullHouse => 6,
            PokerRank::FourOfAKind => 7,
            PokerRank::StraightFlush => 8,
            PokerRank::RoyalFlush => 9,
        }
    }
}

/* Compiles. prevents repetitive code, but isn't needed because hand is already
 * an array. Maybe this will give you ideas though. */
pub struct Hand([Card; 5]);

impl Hand {
    pub fn new(hand: &[Card; 5]) -> Self {
        Self { 0: *hand }
    }
}

pub fn evaluate(hand: &[Card; 5]) -> i32 {
    //
    // existing algorithm in C, but probably will be done differently
    // https://github.com/theimpossibleastronaut/aa-pokerhands/blob/master/src/functions.c#L155-L187
    5
}

#[test]
fn test_evaluate() {
    let hand_arr: [Card; 5] = [
        Card::new(&suit::CLUB, &card_type::ACE),
        Card::new(&suit::HEART, &card_type::THREE),
        Card::new(&suit::DIAMOND, &card_type::THREE),
        Card::new(&suit::CLUB, &card_type::KING),
        Card::new(&suit::CLUB, &card_type::QUEEN),
    ];

    let st_hand = Hand::new(&hand_arr);
    for card in 0..5 {
        println!("{}", st_hand.0[card].value);
    }

    println!();
    println!("{}", hand_arr[2].value);
    assert!(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
