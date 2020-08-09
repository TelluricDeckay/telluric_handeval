use ionic_deckhandler::{Card, Deck};

enum PokerRank {
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl PokerRank {
    fn value(&self) -> i32 {
        match *self {
            PokerRank::Pair => 1,
            PokerRank::TwoPair => 2,
            PokerRank::ThreeOfAKind => 3,
            PokerRank::Straight => 4,
            PokerRank::Flush => 5,
            PokerRank::FullHouse => 6,
            PokerRank::FourOfAKind => 7,
            PokerRank::StraightFlush => 8,
            PokerRank::RoyalFlush => 9,
        }
    }
}

pub struct Hand(Card, Card, Card, Card, Card);

impl Hand {
    pub fn new(deck: &mut Vec<Card>) -> Self {
        Self {
            0: deck.take_from_top().expect("Error, deck is empty!"),
            1: deck.take_from_top().expect("Error, deck is empty!"),
            2: deck.take_from_top().expect("Error, deck is empty!"),
            3: deck.take_from_top().expect("Error, deck is empty!"),
            4: deck.take_from_top().expect("Error, deck is empty!"),
        }
    }
}

pub fn evaluate(hand: Hand) -> i32 {
    //
    // existing algorithm in C, but probably will be done differently
    // https://github.com/theimpossibleastronaut/aa-pokerhands/blob/master/src/functions.c#L155-L187
    5
}

#[test]
fn test_evaluate() {
    let mut deck = ionic_deckhandler::Card::get_deck();
    {
        let first_card = &deck[0];
        assert_eq!(format!("{}", first_card.get_suit()), String::from("♣"));
    }
    deck.shuffle_deck();

    // create an instance of hand
    let hand = crate::Hand::new(&mut deck);
    assert_eq!(evaluate(hand), 5);
}

#[test]
fn test_create_deck() {
    let mut deck = ionic_deckhandler::Card::get_deck();
    {
        let first_card = &deck[0];
        assert_eq!(format!("{}", first_card.get_suit()), String::from("♣"));
    }
    deck.shuffle_deck();

    for card in 0..5 {
        println!("{}", &deck[card]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
