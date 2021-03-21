use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;
use telluric_handeval::poker::{HandRank, PokerRankedHand};

#[test]
fn test_evaluate_four_of_a_kind() {
    let hand_arr1: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Four, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
    ];

    let hand_arr2: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::Ace, Suit::Spades),
        Card::new(Rank::Ace, Suit::Clubs),
    ];

    let hand_arr3: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Hearts),
        Card::new(Rank::Queen, Suit::Diamonds),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
    ];

    assert_eq!(
        hand_arr1.evaluate_hand(),
        HandRank::FourOfAKind {
            kind_rank: Rank::Four,
            kicker_rank: Rank::Queen
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
        hand_arr3.evaluate_hand().cmp(&hand_arr1.evaluate_hand()),
        Ordering::Greater
    );
}
