use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;
use telluric_handeval::poker::{HandRank, PokerRankedHand};

#[test]
fn test_evaluate_flush() {
    let hand_arr: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Five, Suit::Clubs),
        Card::new(Rank::Four, Suit::Clubs),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Three, Suit::Clubs),
    ];
    assert_eq!(
        hand_arr.evaluate_hand(),
        HandRank::Flush {
            ranks: [Rank::King, Rank::Queen, Rank::Five, Rank::Four, Rank::Three]
        }
    );

    let hand_arr = [
        Card::new(Rank::Queen, Suit::Hearts),
        Card::new(Rank::Five, Suit::Hearts),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
    ];
    assert_eq!(
        hand_arr.evaluate_hand(),
        HandRank::Flush {
            ranks: [Rank::King, Rank::Queen, Rank::Five, Rank::Four, Rank::Ace]
        }
    );
}
