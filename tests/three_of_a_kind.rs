use ionic_deckhandler::{Card, Rank, Suit};
use telluric_handeval::poker::{compare, evaluate, Comparison, HandRank};

#[test]
fn test_evaluate_three_of_a_kind() {
    let mut hand_arr: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Four, Suit::Clubs),
    ];
    assert_eq!(evaluate(&mut hand_arr).0, HandRank::ThreeOfAKind);
}

#[test]
fn test_compare_three_of_a_kind() {
    let mut hand_arr_0: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Ten, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
    ];

    let mut hand_arr_1: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::King, Suit::Diamonds),
        Card::new(Rank::Nine, Suit::Spades),
        Card::new(Rank::King, Suit::Clubs),
    ];

    let mut hand_arr_2: [Card; 5] = [
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::Six, Suit::Hearts),
        Card::new(Rank::Six, Suit::Diamonds),
        Card::new(Rank::Five, Suit::Spades),
        Card::new(Rank::Six, Suit::Clubs),
    ];

    assert_eq!(
        compare(
            HandRank::ThreeOfAKind,
            evaluate(&mut hand_arr_0).1,
            evaluate(&mut hand_arr_1).1
        ),
        Comparison::LessThan
    );
}
