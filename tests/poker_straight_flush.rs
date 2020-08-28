use telluric_handeval::poker::{HandRank, PokerRankedHand};
use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;

#[test]
fn test_evaluate_straight_flush() {
    let hand_arr: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Nine, Suit::Clubs),
        Card::new(Rank::Ten, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Clubs),
    ];
    assert_eq!(
        hand_arr.evaluate_hand(),
        HandRank::StraightFlush {
            highest_rank: Rank::King
        }
    );

    // Test for Ace-low Straight Flush
    let hand_arr = [
        Card::new(Rank::Five, Suit::Hearts),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Two, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Four, Suit::Hearts),
    ];
    assert_eq!(
        hand_arr.evaluate_hand(),
        HandRank::StraightFlush {
            highest_rank: Rank::Five
        }
    );

    // Test for Royal Flush (Ace-high straight flush)
    let hand_arr = [
        Card::new(Rank::Queen, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::King, Suit::Hearts),
    ];
    assert_eq!(hand_arr.evaluate_hand(), HandRank::RoyalFlush);
}

