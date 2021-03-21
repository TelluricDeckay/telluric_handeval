use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;
use telluric_handeval::poker::{HandRank, PokerRankedHand};

#[test]
fn test_evaluate_flush() {
    let hand_arr1: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Five, Suit::Clubs),
        Card::new(Rank::Four, Suit::Clubs),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Three, Suit::Clubs),
    ];
    assert_eq!(
        hand_arr1.evaluate_hand(),
        HandRank::Flush {
            ranks: [Rank::King, Rank::Queen, Rank::Five, Rank::Four, Rank::Three]
        }
    );

    let hand_arr2 = [
        Card::new(Rank::Queen, Suit::Hearts),
        Card::new(Rank::Five, Suit::Hearts),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
    ];
    assert_eq!(
        hand_arr2.evaluate_hand(),
        HandRank::Flush {
            ranks: [Rank::King, Rank::Queen, Rank::Five, Rank::Four, Rank::Ace]
        }
    );

    let hand_arr3: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Ten, Suit::Clubs),
        Card::new(Rank::Four, Suit::Clubs),
        Card::new(Rank::Six, Suit::Clubs),
    ];

    let hand_arr4: [Card; 5] = [
        Card::new(Rank::Three, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Four, Suit::Clubs),
        Card::new(Rank::Seven, Suit::Clubs),
        Card::new(Rank::Six, Suit::Clubs),
    ];

    let hand_arr5: [Card; 5] = [
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Seven, Suit::Hearts),
        Card::new(Rank::Six, Suit::Hearts),
    ];

    // Comparator tests
    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr2.evaluate_hand()),
        Ordering::Less
    );

    assert_eq!(
        hand_arr3.evaluate_hand().cmp(&hand_arr4.evaluate_hand()),
        Ordering::Greater
    );

    assert_eq!(
        hand_arr4.evaluate_hand().cmp(&hand_arr5.evaluate_hand()),
        Ordering::Equal
    );
}
