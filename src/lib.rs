pub mod poker {
    use ionic_deckhandler::{Card, Rank};
    use std::cmp::Ordering;
    use std::convert::TryInto;

    #[derive(Debug, PartialEq, Eq)]
    pub enum HandRank {
        RoyalFlush,
        StraightFlush {
            highest_rank: Rank,
        },
        FourOfAKind {
            kind_rank: Rank,
            kicker_rank: Rank,
        },
        FullHouse {
            three_kind_rank: Rank,
            pair_rank: Rank,
        },
        Flush {
            ranks: [Rank; 5],
        },
        Straight {
            highest_rank: Rank,
        },
        ThreeOfAKind {
            kind_rank: Rank,
            other_ranks: [Rank; 2],
        },
        TwoPair {
            higher_pair_rank: Rank,
            lower_pair_rank: Rank,
            kicker_rank: Rank,
        },
        Pair {
            pair_rank: Rank,
            other_ranks: [Rank; 3],
        },
        Highest {
            ranks: [Rank; 5],
        },
        Invalid,
    }

    impl HandRank {
        fn get_rank_u8(&self) -> u8 {
            match *self {
                HandRank::RoyalFlush => 10,
                HandRank::StraightFlush { .. } => 9,
                HandRank::FourOfAKind { .. } => 8,
                HandRank::FullHouse { .. } => 7,
                HandRank::Flush { .. } => 6,
                HandRank::Straight { .. } => 5,
                HandRank::ThreeOfAKind { .. } => 4,
                HandRank::TwoPair { .. } => 3,
                HandRank::Pair { .. } => 2,
                HandRank::Highest { .. } => 1,
                HandRank::Invalid { .. } => 0,
            }
        }

        pub fn stringify(&self) -> &'static str {
            match *self {
                Self::RoyalFlush => "Royal Flush",
                Self::StraightFlush { .. } => "Straight Flush",
                Self::FourOfAKind { .. } => "Four of a kind",
                Self::FullHouse { .. } => "Full House",
                Self::Flush { .. } => "Flush",
                Self::Straight { .. } => "Straight",
                Self::ThreeOfAKind { .. } => "Three of a kind",
                Self::TwoPair { .. } => "Two Pair",
                Self::Pair { .. } => "Pair",
                Self::Highest { .. } => "Nothing",
                Self::Invalid { .. } => "InvalidHand",
            }
        }
    }

    #[macro_export]
    macro_rules! stringify {
        (HandRank::RoyalFlush) => ("Royal Flush");
        (HandRank::StraightFlush) => ("Straight Flush");
        (HandRank::FourOfAKind) => ("Four of a kind");
        (HandRank::FullHouse) => ("Full House");
        (HandRank::Flush) => ("Flush");
        (HandRank::Straight) => ("Straight");
        (HandRank::ThreeOfAKind) => ("Three of a kind");
        (HandRank::TwoPair) => ("Two Pair");
        (HandRank::Pair) => ("Pair");
        (HandRank::Highest) => ("Nothing");
        (HandRank::Invalid) => ("InvalidHand");
    }

    trait AceHigh {
        fn rank_card_ace_high_u8(&self) -> u8;

        fn cmp_ace_high(&self, other: &Self) -> Ordering;
    }

    impl AceHigh for Rank {
        fn rank_card_ace_high_u8(&self) -> u8 {
            match *self {
                Rank::Two => 2,
                Rank::Three => 3,
                Rank::Four => 4,
                Rank::Five => 5,
                Rank::Six => 6,
                Rank::Seven => 7,
                Rank::Eight => 8,
                Rank::Nine => 9,
                Rank::Ten => 10,
                Rank::Jack => 11,
                Rank::Queen => 12,
                Rank::King => 13,
                Rank::Ace => 14,
            }
        }

        fn cmp_ace_high(&self, other: &Self) -> Ordering {
            self.rank_card_ace_high_u8().cmp(&other.rank_card_ace_high_u8())
        }
    }
    
    impl Ord for HandRank {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(&other)
                .expect("Error getting hand ordering.")
        }
    }

    impl PartialOrd for HandRank {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match *self {
                HandRank::RoyalFlush => {
                    if *other == HandRank::RoyalFlush {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Greater)
                    }
                }
                HandRank::StraightFlush { highest_rank } => {
                    if let HandRank::StraightFlush {
                        highest_rank: other_highest_rank,
                    } = *other
                    {
                        if highest_rank == Rank::Ace {
                            if other_highest_rank == Rank::Ace {
                                Some(Ordering::Equal)
                            } else {
                                Some(Ordering::Greater)
                            }
                        } else if other_highest_rank == Rank::Ace {
                            Some(Ordering::Less)
                        } else {
                            highest_rank.partial_cmp(&other_highest_rank)
                        }
                    } else {
                        self.get_rank_u8().partial_cmp(&other.get_rank_u8())
                    }
                }
                HandRank::FourOfAKind {
                    kind_rank,
                    kicker_rank,
                } => {
                    if let HandRank::FourOfAKind {
                        kind_rank: other_kind_rank,
                        kicker_rank: other_kicker_rank,
                    } = *other
                    {
                        let kind_cmp = kind_rank.cmp_ace_high(&other_kind_rank);
                        if kind_cmp == Ordering::Equal {
                            Some(kicker_rank.cmp_ace_high(&other_kicker_rank))
                        } else {
                            Some(kind_cmp)
                        }
                    } else {
                        self.get_rank_u8().partial_cmp(&other.get_rank_u8())
                    }
                }
                HandRank::FullHouse {
                    three_kind_rank,
                    pair_rank,
                } => {
                    if let HandRank::FullHouse {
                        three_kind_rank: other_three_kind_rank,
                        pair_rank: other_pair_rank,
                    } = *other
                    {
                        let three_cmp = three_kind_rank.cmp_ace_high(&other_three_kind_rank);
                        if three_cmp == Ordering::Equal {
                            Some(pair_rank.cmp_ace_high(&other_pair_rank))
                        } else {
                            Some(three_cmp)
                        }
                    } else {
                        self.get_rank_u8().partial_cmp(&other.get_rank_u8())
                    }
                }
                HandRank::Flush { ranks } => {
                    if let HandRank::Flush { ranks: other_ranks } = *other {
                        let mut hand_comp = Ordering::Equal;
                        for (rank, rank_other) in ranks.iter().zip(other_ranks.iter()) {
                            hand_comp = rank.cmp_ace_high(rank_other);
                            if hand_comp != Ordering::Equal {
                                break;
                            }
                        }
                        Some(hand_comp)
                    } else {
                        self.get_rank_u8().partial_cmp(&other.get_rank_u8())
                    }
                }
                HandRank::Straight { highest_rank } => {
                    if let HandRank::Straight {
                        highest_rank: other_highest_rank,
                    } = *other
                    {
                        if highest_rank == Rank::Ace {
                            if other_highest_rank == Rank::Ace {
                                Some(Ordering::Equal)
                            } else {
                                Some(Ordering::Greater)
                            }
                        } else if other_highest_rank == Rank::Ace {
                            Some(Ordering::Less)
                        } else {
                            Some(highest_rank.cmp_ace_high(&other_highest_rank))
                        }
                    } else {
                        self.get_rank_u8().partial_cmp(&other.get_rank_u8())
                    }
                }
                HandRank::ThreeOfAKind {
                    kind_rank,
                    other_ranks,
                } => {
                    if let HandRank::ThreeOfAKind {
                        kind_rank: other_kind_rank,
                        other_ranks: other_other_ranks,
                    } = *other
                    {
                        let cmp_three = kind_rank.cmp_ace_high(&other_kind_rank);
                        if cmp_three == Ordering::Equal {
                            let mut others_comp = Ordering::Equal;
                            for (rank, rank_other) in
                                other_ranks.iter().zip(other_other_ranks.iter())
                            {
                                others_comp = rank.cmp_ace_high(rank_other);
                                if others_comp != Ordering::Equal {
                                    break;
                                }
                            }
                            Some(others_comp)
                        } else {
                            Some(cmp_three)
                        }
                    } else {
                        self.get_rank_u8().partial_cmp(&other.get_rank_u8())
                    }
                }
                HandRank::TwoPair {
                    higher_pair_rank,
                    lower_pair_rank,
                    kicker_rank,
                } => {
                    if let HandRank::TwoPair {
                        higher_pair_rank: other_higher_pair_rank,
                        lower_pair_rank: other_lower_pair_rank,
                        kicker_rank: other_kicker_rank,
                    } = *other
                    {
                        let high_pair_cmp = higher_pair_rank.cmp_ace_high(&other_higher_pair_rank);
                        if high_pair_cmp == Ordering::Equal {
                            let low_pair_cmp = lower_pair_rank.cmp_ace_high(&other_lower_pair_rank);
                            if low_pair_cmp == Ordering::Equal {
                                Some(kicker_rank.cmp_ace_high(&other_kicker_rank))
                            } else {
                                Some(low_pair_cmp)
                            }
                        } else {
                            Some(high_pair_cmp)
                        }
                    } else {
                        self.get_rank_u8().partial_cmp(&other.get_rank_u8())
                    }
                }
                HandRank::Pair {
                    pair_rank,
                    other_ranks,
                } => {
                    if let HandRank::Pair {
                        pair_rank: other_pair_rank,
                        other_ranks: other_other_ranks,
                    } = *other
                    {
                        let pairs_cmp = pair_rank.cmp_ace_high(&other_pair_rank);
                        if pairs_cmp == Ordering::Equal {
                            let mut others_cmp = Ordering::Equal;
                            for (rank, rank_other) in
                                other_ranks.iter().zip(other_other_ranks.iter())
                            {
                                others_cmp = rank.cmp_ace_high(rank_other);
                                if others_cmp != Ordering::Equal {
                                    break;
                                }
                            }
                            Some(others_cmp)
                        } else {
                            Some(pairs_cmp)
                        }
                    } else {
                        self.get_rank_u8().partial_cmp(&other.get_rank_u8())
                    }
                }
                HandRank::Highest { ranks } => {
                    if let HandRank::Highest { ranks: other_ranks } = *other {
                        let mut ranks_cmp = Ordering::Equal;
                        for (rank, rank_other) in ranks.iter().zip(other_ranks.iter()) {
                            ranks_cmp = rank.cmp_ace_high(rank_other);
                            if ranks_cmp != Ordering::Equal {
                                break;
                            }
                        }
                        Some(ranks_cmp)
                    } else {
                        self.get_rank_u8().partial_cmp(&other.get_rank_u8())
                    }
                }
                HandRank::Invalid => Some(Ordering::Less),
            }
        }
    }

    // This is a helper struct for counting the number of cards of a particular rank.
    struct RankCount {
        rank: Rank,
        count: u8,
    }

    pub trait PokerRankedHand {
        fn evaluate_hand(&self) -> HandRank;
    }

    impl PokerRankedHand for [Card; 5] {
        fn evaluate_hand(&self) -> HandRank {
            // Algorithm source: https://nsayer.blogspot.com/2007/07/algorithm-for-evaluating-poker-hands.html
            let mut cards = self.clone();
            // Sort cards by rank descending. Useful for straights and highest.
            cards.sort_by(|c1, c2| c2.get_rank().cmp(&c1.get_rank()));

            let mut rank_histogram: Vec<RankCount> = Vec::new();

            // Get a histogram of ranks and check if flush.
            let is_flush = {
                let mut is_flush = true;
                let suit = cards[0].get_suit();

                for card in cards.iter() {
                    let mut not_found = true;
                    for rank_count in rank_histogram.iter_mut() {
                        if card.get_rank() == (*rank_count).rank {
                            rank_count.count += 1;
                            not_found = false;
                            break;
                        }
                    }
                    if not_found {
                        rank_histogram.push(RankCount {
                            rank: card.get_rank(),
                            count: 1,
                        });
                    }

                    if is_flush && card.get_suit() != suit {
                        is_flush = false;
                    }
                }
                is_flush
            };

            // Order by count descending first, then by rank descending.
            rank_histogram.sort_by(|r1, r2| {
                let count_cmp = r2.count.cmp(&r1.count);
                match count_cmp {
                    Ordering::Equal => r2.rank.cmp_ace_high(&r1.rank),
                    _ => count_cmp,
                }
            });

            // Now we work out the hand kind using the histogram
            // Length of the histogram indicates how many different ranks there are. This must be between 2 and 5.
            // E.g. length 2 means there's only 2 ranks. This only happens with 4 kind or full house.
            match rank_histogram.len() {
                2 => {
                    // FourOfAKind or FullHouse.
                    if rank_histogram[0].count == 4 {
                        HandRank::FourOfAKind {
                            kind_rank: rank_histogram[0].rank,
                            kicker_rank: rank_histogram[1].rank,
                        }
                    } else {
                        HandRank::FullHouse {
                            three_kind_rank: rank_histogram[0].rank,
                            pair_rank: rank_histogram[1].rank,
                        }
                    }
                }
                3 => {
                    // Flush, ThreeOfAKind or TwoPair.
                    if is_flush {
                        HandRank::Flush {
                            ranks: cards.iter().map(|c| c.get_rank()).collect::<Vec<_>>()[0..5]
                                .try_into()
                                .unwrap(),
                        }
                    } else {
                        if rank_histogram[0].count == 3 {
                            HandRank::ThreeOfAKind {
                                kind_rank: rank_histogram[0].rank,
                                other_ranks: [rank_histogram[1].rank, rank_histogram[2].rank],
                            }
                        } else {
                            HandRank::TwoPair {
                                higher_pair_rank: rank_histogram[0].rank,
                                lower_pair_rank: rank_histogram[1].rank,
                                kicker_rank: rank_histogram[2].rank,
                            }
                        }
                    }
                }
                4 => {
                    // Pair or Flush.
                    if is_flush {
                        HandRank::Flush {
                            ranks: cards.iter().map(|c| c.get_rank()).collect::<Vec<_>>()[0..5]
                                .try_into()
                                .unwrap(),
                        }
                    } else {
                        HandRank::Pair {
                            pair_rank: rank_histogram[0].rank,
                            other_ranks: [
                                rank_histogram[1].rank,
                                rank_histogram[2].rank,
                                rank_histogram[3].rank,
                            ],
                        }
                    }
                }
                5 => {
                    // Highest, Straight, Flush, StraightFlush or RoyalFlush.
                    if {
                        // Check if the first four cards are straight. We only check the first four because if it's
                        // a royal flush, the last card will be an Ace, so will fail the standard "straight" test.
                        let mut is_four_straight = true;
                        for i in 1..4 {
                            if cards[i - 1].get_rank() as u8 - cards[i].get_rank() as u8 != 1 {
                                is_four_straight = false;
                                break;
                            }
                        }
                        is_four_straight
                    } {
                        if cards[4].get_rank() == Rank::Ace && cards[0].get_rank() == Rank::King {
                            if is_flush {
                                HandRank::RoyalFlush
                            } else {
                                // Use the Ace as the highest rank, because then we know it's a Royal straight.
                                HandRank::Straight {
                                    highest_rank: cards[4].get_rank(),
                                }
                            }
                        } else if cards[3].get_rank() as u8 - cards[4].get_rank() as u8 == 1 {
                            if is_flush {
                                HandRank::StraightFlush {
                                    highest_rank: cards[0].get_rank(),
                                }
                            } else {
                                HandRank::Straight {
                                    highest_rank: cards[0].get_rank(),
                                }
                            }
                        } else {
                            HandRank::Highest {
                                ranks: cards.iter().map(|c| c.get_rank()).collect::<Vec<_>>()[0..5]
                                    .try_into()
                                    .unwrap(),
                            }
                        }
                    } else if is_flush {
                        HandRank::Flush {
                            ranks: cards.iter().map(|c| c.get_rank()).collect::<Vec<_>>()[0..5]
                                .try_into()
                                .unwrap(),
                        }
                    } else {
                        HandRank::Highest {
                            ranks: cards.iter().map(|c| c.get_rank()).collect::<Vec<_>>()[0..5]
                                .try_into()
                                .unwrap(),
                        }
                    }
                }
                _ => HandRank::Invalid,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::poker::{HandRank, PokerRankedHand};
        use ionic_deckhandler::{Card, Deck, Rank, Suit};
        use std::cmp::Ordering;

        #[test]
        fn test_evaluate_nothing() {
            let hand_arr: [Card; 5] = [
                Card::new(Rank::Queen, Suit::Clubs),
                Card::new(Rank::Five, Suit::Hearts),
                Card::new(Rank::Eight, Suit::Diamonds),
                Card::new(Rank::King, Suit::Spades),
                Card::new(Rank::Ten, Suit::Clubs),
            ];
            assert_eq!(
                hand_arr.evaluate_hand(),
                HandRank::Highest {
                    ranks: [Rank::King, Rank::Queen, Rank::Ten, Rank::Eight, Rank::Five]
                }
            );

            // Almost a straight, but still nothing
            let hand_arr = [
                Card::new(Rank::Ten, Suit::Clubs),
                Card::new(Rank::Five, Suit::Hearts),
                Card::new(Rank::Eight, Suit::Diamonds),
                Card::new(Rank::Seven, Suit::Clubs),
                Card::new(Rank::Nine, Suit::Clubs),
            ];
            assert_eq!(
                hand_arr.evaluate_hand(),
                HandRank::Highest {
                    ranks: [Rank::Ten, Rank::Nine, Rank::Eight, Rank::Seven, Rank::Five]
                }
            );
        }

        #[test]
        fn test_evaluate_invalid_hand() {
            let hand_arr: [Card; 5] = [
                Card::new(Rank::Queen, Suit::Hearts),
                Card::new(Rank::Queen, Suit::Diamonds),
                Card::new(Rank::Queen, Suit::Clubs),
                Card::new(Rank::Queen, Suit::Spades),
                Card::new(Rank::Queen, Suit::Clubs),
            ];
            assert_eq!(hand_arr.evaluate_hand(), HandRank::Invalid);
        }
        
        #[test]
        fn test_multiple_ranks() {
            let mut deck = Card::get_deck();
            let mut pairs = 0;
            let mut two_pairs = 0;
            let mut three_of_a_kinds = 0;
            let mut straights = 0;
            let mut flush = 0;
            let mut full_house = 0;
            let mut four_of_a_kind = 0;
            let mut straight_flushes = 0;
            let mut royal_flushes = 0;

            let hands_dealt = 10000;

            for _i in 0..hands_dealt {
                deck.shuffle_deck();
                let mut hand_arr: [Card; 5] = [deck[0], deck[1], deck[2], deck[3], deck[4]];

                match hand_arr.evaluate_hand() {
                    HandRank::Pair { .. } => pairs += 1,
                    HandRank::TwoPair { .. } => two_pairs += 1,
                    HandRank::ThreeOfAKind { .. } => three_of_a_kinds += 1,
                    HandRank::Straight { .. } => straights += 1,
                    HandRank::Flush { .. } => flush += 1,
                    HandRank::FullHouse { .. } => full_house += 1,
                    HandRank::FourOfAKind { .. } => four_of_a_kind += 1,
                    HandRank::StraightFlush { .. } => straight_flushes += 1,
                    HandRank::RoyalFlush => royal_flushes += 1,
                    _ => (),
                }
            }
            println!("Out of {} hands dealt...\n", hands_dealt);
            println!("{} = {}", stringify!(HandRank::Pair), pairs);
            println!("{} = {}", stringify!(HandRank::TwoPair), two_pairs);
            println!("{} = {}", stringify!(HandRank::ThreeOfAKind), three_of_a_kinds);
            println!("{} = {}", stringify!(HandRank::Straight), straights);
            println!("{} = {}", stringify!(HandRank::Flush), flush);
            println!("{} = {}", stringify!(HandRank::FullHouse), full_house);
            println!("{} = {}", stringify!(HandRank::FourOfAKind), four_of_a_kind);
            println!("{} = {}", stringify!(HandRank::StraightFlush), straight_flushes);
            println!("{} = {}", stringify!(HandRank::RoyalFlush), royal_flushes);
        } 
    }
}
