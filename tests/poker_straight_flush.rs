use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;
use telluric_handeval::poker::{HandRank, PokerRankedHand};

#[test]
fn test_evaluate_straight_flush() {
    let hand_arr1: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Nine, Suit::Clubs),
        Card::new(Rank::Ten, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Clubs),
    ];
    assert_eq!(
        hand_arr1.evaluate_hand(),
        HandRank::StraightFlush {
            highest_rank: Rank::King
        }
    );

    // Test for Ace-low Straight Flush
    let hand_arr2 = [
        Card::new(Rank::Five, Suit::Hearts),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Two, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Four, Suit::Hearts),
    ];
    assert_eq!(
        hand_arr2.evaluate_hand(),
        HandRank::StraightFlush {
            highest_rank: Rank::Five
        }
    );

    // A straight flush should never be considered a straight
    let hand_arr3 = [
        Card::new(Rank::Five, Suit::Clubs),
        Card::new(Rank::Six, Suit::Clubs),
        Card::new(Rank::Seven, Suit::Clubs),
        Card::new(Rank::Eight, Suit::Clubs),
        Card::new(Rank::Nine, Suit::Clubs),
    ];
    assert!(
        hand_arr3.evaluate_hand()
            != HandRank::Straight {
                highest_rank: Rank::Nine
            }
    );

    // Test for Royal Flush (Ace-high straight flush)
    let hand_arr4 = [
        Card::new(Rank::Queen, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::King, Suit::Hearts),
    ];
    assert_eq!(hand_arr4.evaluate_hand(), HandRank::RoyalFlush);

    // Comparator tests.
    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr4.evaluate_hand()),
        Ordering::Less
    );

    let hand_arr5: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr5.evaluate_hand()),
        Ordering::Greater
    );
}
