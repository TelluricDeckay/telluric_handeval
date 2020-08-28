use telluric_handeval::poker::{HandRank, PokerRankedHand};
use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;

#[test]
fn test_evaluate_full_house() {
    let hand_arr: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
    ];
    assert_eq!(
        hand_arr.evaluate_hand(),
        HandRank::FullHouse {
            three_kind_rank: Rank::Four,
            pair_rank: Rank::Queen
        }
    );
}
