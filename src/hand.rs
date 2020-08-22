use ionic_deckhandler::{Card, Rank};
use std::cmp::Ordering;
use std::convert::TryInto;

#[derive(PartialEq, Eq)]
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
    ThreeOfAkind {
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
            HandRank::ThreeOfAkind { .. } => 4,
            HandRank::TwoPair { .. } => 3,
            HandRank::Pair { .. } => 2,
            HandRank::Highest { .. } => 1,
        }
    }
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).expect("Error getting hand ordering.")
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
                    let kind_cmp = kind_rank.cmp(&other_kind_rank);
                    if kind_cmp == Ordering::Equal {
                        Some(kicker_rank.cmp(&other_kicker_rank))
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
                    let three_cmp = three_kind_rank.cmp(&other_three_kind_rank);
                    if three_cmp == Ordering::Equal {
                        pair_rank.partial_cmp(&other_pair_rank)
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
                        hand_comp = rank.cmp(rank_other);
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
                        highest_rank.partial_cmp(&other_highest_rank)
                    }
                } else {
                    self.get_rank_u8().partial_cmp(&other.get_rank_u8())
                }
            }
            HandRank::ThreeOfAkind {
                kind_rank,
                other_ranks,
            } => {
                if let HandRank::ThreeOfAkind {
                    kind_rank: other_kind_rank,
                    other_ranks: other_other_ranks,
                } = *other
                {
                    let cmp_three = kind_rank.cmp(&other_kind_rank);
                    if cmp_three == Ordering::Equal {
                        let mut others_comp = Ordering::Equal;
                        for (rank, rank_other) in other_ranks.iter().zip(other_other_ranks.iter()) {
                            others_comp = rank.cmp(rank_other);
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
                    let high_pair_cmp = higher_pair_rank.cmp(&other_higher_pair_rank);
                    if high_pair_cmp == Ordering::Equal {
                        let low_pair_cmp = lower_pair_rank.cmp(&other_lower_pair_rank);
                        if low_pair_cmp == Ordering::Equal {
                            kicker_rank.partial_cmp(&other_kicker_rank)
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
                    let pairs_cmp = pair_rank.cmp(&other_pair_rank);
                    if pairs_cmp == Ordering::Equal {
                        let mut others_cmp = Ordering::Equal;
                        for (rank, rank_other) in other_ranks.iter().zip(other_other_ranks.iter()) {
                            others_cmp = rank.cmp(rank_other);
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
                        ranks_cmp = rank.cmp(rank_other);
                        if ranks_cmp != Ordering::Equal {
                            break;
                        }
                    }
                    Some(ranks_cmp)
                } else {
                    self.get_rank_u8().partial_cmp(&other.get_rank_u8())
                }
            }
        }
    }
}

// This is a helper struct for counting the number of cards of a particular rank.
struct RankCount {
    rank: Rank,
    count: u8,
}

pub trait PokerRankedHand {
    fn get_hand_kind(&self) -> HandRank;
}

impl PokerRankedHand for [Card; 5] {
    fn get_hand_kind(&self) -> HandRank {
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
                Ordering::Equal => r2.rank.cmp(&r1.rank),
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
                        HandRank::ThreeOfAkind {
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
                        if cards[i].get_rank() as u8 - cards[i - 1].get_rank() as u8 != 1 {
                            is_four_straight = false;
                            break;
                        }
                    }
                    is_four_straight
                } {
                    if cards[4].get_rank() == Rank::Ace && cards[0].get_rank() == Rank::King {
                        HandRank::RoyalFlush
                    } else if cards[4].get_rank() as u8 - cards[3].get_rank() as u8 != 1 {
                        if is_flush {
                            HandRank::StraightFlush {
                                // Use the Ace as the highest rank, because then we know it's a Royal straight.
                                highest_rank: cards[4].get_rank(),
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
            _ => panic!("Invalid hand."),
        }
    }
}
