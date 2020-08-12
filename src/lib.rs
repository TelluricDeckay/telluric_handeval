mod poker {

    use ionic_deckhandler::{Card, Deck, Rank, Suit, CARD_RANK_COUNT};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandRank {
        Nothing,
        Pair,
        TwoPair,
        ThreeOfAKind,
        Straight,
        Flush,
        FullHouse,
        FourOfAKind,
        StraightFlush,
        RoyalFlush,
        InvalidHand,
    }

    impl HandRank {
        pub fn name(&self) -> &'static str {
            match self {
                Self::Nothing => "Nothing",
                Self::Pair => "Pair",
                Self::TwoPair => "Two of a kind",
                Self::ThreeOfAKind => "Three of a kind",
                Self::Straight => "Straight",
                Self::Flush => "Flush",
                Self::FullHouse => "Full House",
                Self::FourOfAKind => "Four of a kind",
                Self::StraightFlush => "Straight Flush",
                Self::RoyalFlush => "Royal Flush",
                Self::InvalidHand => "InvalidHand",
            }
        }
    }
    /*
    fn main() {
        let my_rank = HandRank::Pair;
        println!("{}", my_rank.name());
    }
    */

    // Evalute for a poker hand.
    //
    // At some point this function may get split into smaller ones.
    pub fn evaluate(hand: &mut [Card; 5]) -> HandRank {
        // Used to check for a straight, but in the future may also be used to replace
        // the method used to check for pairs, two pair, etc.
        let mut hand_by_card_rank_sequence: [i32; CARD_RANK_COUNT + 1] =
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        // Used later to check for a flush
        let mut hand_by_suit: [i32; 4] = [0, 0, 0, 0];
        for card in hand.iter() {
            let ucard_rank = Card::get_rank(&card) as usize;
            let ucard_suit = Card::get_suit(&card) as usize;
            hand_by_suit[ucard_suit] += 1;
            hand_by_card_rank_sequence[ucard_rank] += 1;
            // If the card is an Ace, also add '1' to the end of the array (after the King)
            if ucard_rank == Rank::Ace as usize {
                hand_by_card_rank_sequence[ucard_rank + CARD_RANK_COUNT] += 1;
            }
        }

        hand.sort();
        let last_index = hand.len() - 1;
        let mut card_matches: [i32; 2] = [0, 0];
        let mut break_in_card_matches: bool = false;
        // The [Card; 5] will probably be changed to an Option<T> later to account
        // for 7-card variations, using last_index here instead of a literal value.
        for card in 0..last_index {
            println!("card {},{}", card, Card::get_rank(&hand[card]) as i32);
            match hand[card] == hand[card + 1] {
                true => match card_matches[0] {
                    0 => card_matches[0] += 1,
                    _ => {
                        if break_in_card_matches == false {
                            card_matches[0] += 1;
                        } else {
                            card_matches[1] += 1;
                        }
                    }
                },
                false => break_in_card_matches = true,
            }
        }

        if card_matches[0] == 0 {
            // Check for a straight
            let mut is_straight: bool = false;
            let mut consecutive_1_counter: i32 = 0;
            for i in &hand_by_card_rank_sequence {
                if *i == 1 {
                    consecutive_1_counter += 1;
                    if consecutive_1_counter == 5 {
                        is_straight = true;
                        break;
                    }
                } else {
                    consecutive_1_counter = 0;
                }
            }

            // Check for flush
            let mut is_flush: bool = false;
            for i in &hand_by_suit {
                if *i == 5 {
                    is_flush = true;
                    break;
                }
            }

            let is_straight_flush: bool = is_straight && is_flush;

            if is_straight_flush {
                // Check for High Ace - King Sequence
                if hand_by_card_rank_sequence[CARD_RANK_COUNT] == 1
                    && hand_by_card_rank_sequence[CARD_RANK_COUNT - 1] == 1
                {
                    return HandRank::RoyalFlush;
                }
                return HandRank::StraightFlush;
            }

            if is_straight {
                return HandRank::Straight;
            }

            if is_flush {
                return HandRank::Flush;
            }
            return HandRank::Nothing;
        }

        if card_matches[0] >= 1 && (card_matches[1] == 0) {
            println!("matches 0 {}", card_matches[0]);
            match card_matches[0] {
                1 => return HandRank::Pair,
                2 => return HandRank::ThreeOfAKind,
                3 => return HandRank::FourOfAKind,
                // Error handling: see https://github.com/theimpossibleastronaut/rmwrs/blob/fabcf801a65a7d86a380573cf60ef7dff6d85511/src/lib.rs#L139
                _ => return HandRank::InvalidHand,
            }
        }

        // Check for two pair or full house
        if card_matches[0] == card_matches[1] {
            return HandRank::TwoPair;
        }

        HandRank::FullHouse
    }

    #[test]
    fn test_evaluate_pair() {
        let mut hand_arr: [Card; 5] = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::Pair);
    }

    #[test]
    fn test_evaluate_two_pair() {
        let mut hand_arr: [Card; 5] = [
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::TwoPair);
    }

    #[test]
    fn test_evaluate_three_of_a_kind() {
        let mut hand_arr: [Card; 5] = [
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::ThreeOfAKind);
    }

    #[test]
    fn test_evaluate_straight() {
        // Test for Ace-high straight
        let mut hand_arr: [Card; 5] = [
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::Straight);

        // Test for Ace-low straight
        hand_arr = [
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::Straight);

        // Test for Ten-high straight
        hand_arr = [
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::Straight);
    }

    #[test]
    fn test_evaluate_flush() {
        let mut hand_arr: [Card; 5] = [
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::Flush);

        hand_arr = [
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::Flush);
    }

    #[test]
    fn test_evaluate_full_house() {
        let mut hand_arr: [Card; 5] = [
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Four, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::FullHouse);
    }

    #[test]
    fn test_evaluate_four_of_a_kind() {
        let mut hand_arr: [Card; 5] = [
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Four, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::FourOfAKind);
    }

    #[test]
    fn test_evaluate_nothing() {
        let mut hand_arr: [Card; 5] = [
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Ten, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::Nothing);

        // Almost a straight, but still nothing
        hand_arr = [
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
        ];
        assert!(evaluate(&mut hand_arr) == HandRank::Nothing);
    }

    #[test]
    fn test_evaluate_straight_flush() {
        let mut hand_arr: [Card; 5] = [
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::StraightFlush);

        // Test for Ace-low Straight Flush
        hand_arr = [
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::StraightFlush);

        // Test for Royal Flush (Ace-high straight flush)
        hand_arr = [
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::RoyalFlush);
    }

    #[test]
    fn test_evaluate_invalid_hand() {
        let mut hand_arr: [Card; 5] = [
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Queen, Suit::Clubs),
        ];
        assert_eq!(evaluate(&mut hand_arr), HandRank::InvalidHand);
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    }
}
