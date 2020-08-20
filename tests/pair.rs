use ionic_deckhandler::{Card, Rank, Suit};
use telluric_handeval::poker::{compare, evaluate, Comparison, HandRank};

#[test]
fn test_evaluate_pair() {
    let mut hand_arr: [Card; 5] = [
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];
    assert_eq!(evaluate(&mut hand_arr).0, HandRank::Pair);
}

#[test]
fn test_compare_pair() {
    let mut hand_arr_0: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::King, Suit::Diamonds),
        Card::new(Rank::Nine, Suit::Spades),
        Card::new(Rank::Nine, Suit::Clubs),
    ];

    let mut hand_arr_1: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Ten, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
    ];

    let mut hand_arr_2: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Spades),
        Card::new(Rank::Four, Suit::Spades),
    ];

    let mut hand_arr_3: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Nine, Suit::Spades),
        Card::new(Rank::Four, Suit::Spades),
    ];

    assert_eq!(
        compare(
            HandRank::Pair,
            evaluate(&mut hand_arr_0).1,
            evaluate(&mut hand_arr_1).1
        ),
        Comparison::GreaterThan
    );

    assert_eq!(
        compare(
            HandRank::Pair,
            evaluate(&mut hand_arr_1).1,
            evaluate(&mut hand_arr_2).1
        ),
        Comparison::Equal
    );

    assert_eq!(
        compare(
            HandRank::Pair,
            evaluate(&mut hand_arr_2).1,
            evaluate(&mut hand_arr_3).1
        ),
        Comparison::GreaterThan
    );
}
