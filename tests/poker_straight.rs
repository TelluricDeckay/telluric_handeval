use telluric_handeval::poker::{HandRank, PokerRankedHand};
use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;

#[test]
fn test_evaluate_straight() {
    // Test for Ace-high straight
    let hand_arr: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Diamonds),
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Clubs),
    ];
    assert_eq!(
        hand_arr.evaluate_hand(),
        HandRank::Straight {
            highest_rank: Rank::Ace
        }
    );

    // Test for Ace-low straight
    let hand_arr = [
        Card::new(Rank::Three, Suit::Clubs),
        Card::new(Rank::Five, Suit::Hearts),
        Card::new(Rank::Two, Suit::Diamonds),
        Card::new(Rank::Four, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Clubs),
    ];
    assert_eq!(
        hand_arr.evaluate_hand(),
        HandRank::Straight {
            highest_rank: Rank::Five
        }
    );

    // Test for Ten-high straight
    let hand_arr = [
        Card::new(Rank::Ten, Suit::Clubs),
        Card::new(Rank::Six, Suit::Hearts),
        Card::new(Rank::Eight, Suit::Diamonds),
        Card::new(Rank::Seven, Suit::Clubs),
        Card::new(Rank::Nine, Suit::Clubs),
    ];
    assert_eq!(
        hand_arr.evaluate_hand(),
        HandRank::Straight {
            highest_rank: Rank::Ten
        }
    );
}

// Comparator tests.
#[test]
fn test_compare_straights() {
    let hand_arr1 = [
        Card::new(Rank::Queen, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Spades),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
    ];

    let hand_arr2 = [
        Card::new(Rank::Queen, Suit::Diamonds),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Nine, Suit::Hearts),
    ];

    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr2.evaluate_hand()),
        Ordering::Greater
    );

    let hand_arr1 = [
        Card::new(Rank::Queen, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Spades),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Eight, Suit::Hearts),
        Card::new(Rank::Nine, Suit::Hearts),
    ];

    let hand_arr2 = [
        Card::new(Rank::Queen, Suit::Diamonds),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Nine, Suit::Hearts),
    ];

    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr2.evaluate_hand()),
        Ordering::Less
    );

    let hand_arr3 = [
        Card::new(Rank::Eight, Suit::Diamonds),
        Card::new(Rank::Jack, Suit::Spades),
        Card::new(Rank::Ten, Suit::Diamonds),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Nine, Suit::Hearts),
    ];

    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr3.evaluate_hand()),
        Ordering::Equal
    );
}
