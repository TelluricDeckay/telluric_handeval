use ionic_deckhandler::{Card, Rank, Suit};
use telluric_handeval::poker::{compare, evaluate, Comparison, HandRank};

#[test]
fn test_evaluate_two_pair() {
    let mut hand_arr_0: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];
    assert_eq!(evaluate(&mut hand_arr_0).0, HandRank::TwoPair);

    let mut hand_arr_1: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

    let mut hand_arr_2: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Hearts),
        Card::new(Rank::Four, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Spades),
    ];

    let mut hand_arr_3: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Hearts),
        Card::new(Rank::Four, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Spades),
    ];

    let mut hand_arr_4: [Card; 5] = [
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Two, Suit::Spades),
        Card::new(Rank::Two, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::King, Suit::Spades),
    ];

    assert_eq!(
        compare(
            HandRank::TwoPair,
            evaluate(&mut hand_arr_0).1,
            evaluate(&mut hand_arr_1).1
        ),
        Comparison::LessThan
    );

    assert_eq!(
        compare(
            HandRank::TwoPair,
            evaluate(&mut hand_arr_1).1,
            evaluate(&mut hand_arr_2).1
        ),
        Comparison::Equal
    );

    assert_eq!(
        compare(
            HandRank::TwoPair,
            evaluate(&mut hand_arr_2).1,
            evaluate(&mut hand_arr_3).1
        ),
        Comparison::GreaterThan
    );

    assert_eq!(
        compare(
            HandRank::TwoPair,
            evaluate(&mut hand_arr_3).1,
            evaluate(&mut hand_arr_4).1
        ),
        Comparison::LessThan
    );
}
