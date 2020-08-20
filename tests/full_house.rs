use ionic_deckhandler::{Card, Rank, Suit};
use telluric_handeval::poker::{compare, evaluate, Comparison, HandRank};

#[test]
fn test_evaluate_full_house() {
    let mut hand_arr: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
    ];
    assert_eq!(evaluate(&mut hand_arr).0, HandRank::FullHouse);
}

#[test]
fn test_compare_full_house() {
    let mut hand_arr_0: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Four, Suit::Clubs),
    ];

    let mut hand_arr_1: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Ace, Suit::Clubs),
    ];

    let mut hand_arr_2: [Card; 5] = [
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Jack, Suit::Diamonds),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Jack, Suit::Clubs),
    ];

    assert_eq!(
        compare(
            HandRank::FullHouse,
            evaluate(&mut hand_arr_0).1,
            evaluate(&mut hand_arr_1).1
        ),
        Comparison::LessThan
    );
}
