use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;
use telluric_handeval::poker::{HandRank, PokerRankedHand};

#[test]
fn test_evaluate_two_pair() {
    let hand_arr1: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    let hand_arr2: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    let hand_arr3: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    let hand_arr4: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    let hand_arr5: [Card; 5] = [
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Diamonds),
        Card::new(Rank::Five, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Clubs),
    ];

    let hand_arr6: [Card; 5] = [
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Clubs),
    ];

    let hand_arr7: [Card; 5] = [
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Diamonds),
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Clubs),
    ];

    assert_eq!(
        hand_arr1.evaluate_hand(),
        HandRank::TwoPair {
            higher_pair_rank: Rank::Queen,
            lower_pair_rank: Rank::Three,
            kicker_rank: Rank::King,
        }
    );

    // Comparator tests.

    // These two tests fail, probably due to the Ace not being counted as high

    assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr2.evaluate_hand()),
        Ordering::Less
    );

    assert_eq!(
        hand_arr6.evaluate_hand().cmp(&hand_arr7.evaluate_hand()),
        Ordering::Less
    );

    assert_eq!(
        hand_arr2.evaluate_hand().cmp(&hand_arr3.evaluate_hand()),
        Ordering::Equal
    );

    assert_eq!(
        hand_arr4.evaluate_hand().cmp(&hand_arr1.evaluate_hand()),
        Ordering::Greater
    );

    assert_eq!(
        hand_arr4.evaluate_hand().cmp(&hand_arr5.evaluate_hand()),
        Ordering::Greater
    );

    assert_eq!(
        hand_arr5.evaluate_hand().cmp(&hand_arr6.evaluate_hand()),
        Ordering::Less
    );
}
