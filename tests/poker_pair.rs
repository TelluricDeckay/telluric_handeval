use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;
use telluric_handeval::poker::{HandRank, PokerRankedHand};

#[test]
fn test_evaluate_pair() {
    let hand_arr1: [Card; 5] = [
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    let hand_arr2: [Card; 5] = [
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    let hand_arr3: [Card; 5] = [
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    let hand_arr4: [Card; 5] = [
        Card::new(Rank::Two, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    assert_eq!(
        hand_arr1.evaluate_hand(),
        HandRank::Pair {
            pair_rank: Rank::Three,
            other_ranks: [Rank::Ace, Rank::King, Rank::Queen]
        }
    );

    // Comparator tests
    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr2.evaluate_hand()),
        Ordering::Less
    );

    assert_eq!(
        hand_arr2.evaluate_hand().cmp(&hand_arr3.evaluate_hand()),
        Ordering::Equal
    );

    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr3.evaluate_hand()),
        Ordering::Less
    );

    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr4.evaluate_hand()),
        Ordering::Greater
    );
}
