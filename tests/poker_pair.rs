use telluric_handeval::poker::{HandRank, PokerRankedHand};
use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;

#[test]
fn test_evaluate_pair() {
    let hand_arr: [Card; 5] = [
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];
    assert_eq!(
        hand_arr.evaluate_hand(),
        HandRank::Pair {
            pair_rank: Rank::Three,
            other_ranks: [Rank::King, Rank::Queen, Rank::Ace]
        }
    );
}
