use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;
use telluric_handeval::poker::{HandRank, PokerRankedHand};

#[test]
fn test_evaluate_full_house() {
    let hand_arr1: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
    ];

    let hand_arr2: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    let hand_arr3: [Card; 5] = [
        Card::new(Rank::Two, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::Two, Suit::Clubs),
        Card::new(Rank::Two, Suit::Clubs),
    ];

    assert_eq!(
        hand_arr1.evaluate_hand(),
        HandRank::FullHouse {
            three_kind_rank: Rank::Four,
            pair_rank: Rank::Queen
        }
    );

    // Comparator tests
    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr2.evaluate_hand()),
        Ordering::Less
    );

    assert_eq!(
        hand_arr2.evaluate_hand().cmp(&hand_arr3.evaluate_hand()),
        Ordering::Greater
    );

    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr3.evaluate_hand()),
        Ordering::Greater
    );
}
