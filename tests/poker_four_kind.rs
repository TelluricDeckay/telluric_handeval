use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;
use telluric_handeval::poker::{HandRank, PokerRankedHand};

#[test]
fn test_evaluate_four_of_a_kind() {
    let hand_arr: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Four, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
    ];
    assert_eq!(
        hand_arr.evaluate_hand(),
        HandRank::FourOfAKind {
            kind_rank: Rank::Four,
            kicker_rank: Rank::Queen
        }
    );
}
