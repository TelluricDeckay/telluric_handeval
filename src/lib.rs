use ionic_deckhandler::{card_type, suit, Card, Deck};

pub struct PokerRank(String);

const PAIR: i32 = 0;
const TWO_PAIR: i32 = 1;
const THREE_OF_A_KIND: i32 = 2;
const STRAIGHT: i32 = 3;
const FLUSH: i32 = 4;
const FULL_HOUSE: i32 = 5;
const FOUR_OF_A_KIND: i32 = 6;
const STRAIGHT_FLUSH: i32 = 7;
const ROYAL_FLUSH: i32 = 8;

const NUM_OF_RANKS: i32 = 9;

impl PokerRank {
    pub fn new(lit: String) -> Self {
        Self { 0: lit }
    }

    pub fn init_ranks() -> [PokerRank; NUM_OF_RANKS as usize] {
        [
            PokerRank::new(String::from("Pair")),
            PokerRank::new(String::from("Two of a kind")),
            PokerRank::new(String::from("Three of a kind")),
            PokerRank::new(String::from("Straight")),
            PokerRank::new(String::from("Flush")),
            PokerRank::new(String::from("Full House")),
            PokerRank::new(String::from("Four of a kind")),
            PokerRank::new(String::from("Straight Flush")),
            PokerRank::new(String::from("Royal Flush")),
        ]
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
