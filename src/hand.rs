use ionic_deckhandler::{Rank, Card};
use std::convert::TryInto;
use std::cmp::Ordering;


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
        ranks: [Rank; 5]
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
        ranks: [Rank; 5]
    }
}

impl HandRank {
    fn get_rank_u8(&self) -> u8 {
        match *self {
            HandRank::RoyalFlush => 10,
            HandRank::StraightFlush {..} => 9,
            HandRank::FourOfAKind {..} => 8,
            HandRank::FullHouse {..} => 7,
            HandRank::Flush {..} => 6,
            HandRank::Straight {..} => 5,
            HandRank::ThreeOfAkind {..} => 4,
            HandRank::TwoPair {..} => 3,
            HandRank::Pair {..} => 2,
            HandRank::Highest {..} => 1,
        }
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match *self {
            HandRank::RoyalFlush => {
                if *other == HandRank::RoyalFlush {
                    Some(Ordering::Equal)
                }
                else {
                    Some(Ordering::Greater)
                }
            }
            HandRank::StraightFlush { highest_rank } => {
                if let HandRank::StraightFlush { highest_rank: other_highest_rank } = *other {
                    Some(highest_rank.cmp(&other_highest_rank))
                }
                else {
                    Some(self.get_rank_u8().cmp(&other.get_rank_u8()))
                }
            }
            HandRank::FourOfAKind { kind_rank, kicker_rank } => {
                if let HandRank::FourOfAKind { kind_rank: other_kind_rank, kicker_rank: other_kicker_rank } = *other {
                    let kind_cmp = kind_rank.cmp(&other_kind_rank);
                    if kind_cmp == Ordering::Equal {
                        Some(kicker_rank.cmp(&other_kicker_rank))
                    }
                    else {
                        Some(kind_cmp)
                    }
                }
                else {
                    Some(self.get_rank_u8().cmp(&other.get_rank_u8()))
                }
            }
            _ => unimplemented!()
        }
    }
}

// This is a helper struct for counting the number of cards of a particular rank.
struct RankCount {
    rank: Rank,
    count: u8
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
                    rank_histogram.push(RankCount { rank: card.get_rank(), count: 1 });
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
                Ordering::Equal => {
                    r2.rank.cmp(&r1.rank)
                },
                _ => count_cmp
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
                }
                else {
                    HandRank::FullHouse {
                        three_kind_rank: rank_histogram[0].rank,
                        pair_rank: rank_histogram[1].rank,
                    }
                }
            },
            3 => {
                // Flush, ThreeOfAKind or TwoPair.
                if is_flush {
                    HandRank::Flush {
                        ranks: cards.into_iter().map(|c| c.get_rank()).collect::<Vec<_>>()[0..5].try_into().unwrap()
                    }
                }
                else {
                    if rank_histogram[0].count == 3 {
                        HandRank::ThreeOfAkind {
                            kind_rank: rank_histogram[0].rank,
                            other_ranks: [rank_histogram[1].rank, rank_histogram[2].rank]
                        }
                    }
                    else {
                        HandRank::TwoPair {
                            higher_pair_rank: rank_histogram[0].rank,
                            lower_pair_rank: rank_histogram[1].rank,
                            kicker_rank: rank_histogram[2].rank
                        }
                    }
                }
            }
            4 => {
                // Pair or Flush.
                if is_flush {
                    HandRank::Flush {
                        ranks: cards.into_iter().map(|c| c.get_rank()).collect::<Vec<_>>()[0..5].try_into().unwrap()
                    }
                }
                else {
                    HandRank::Pair {
                        pair_rank: rank_histogram[0].rank,
                        other_ranks: [rank_histogram[1].rank, rank_histogram[2].rank, rank_histogram[3].rank],
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
                        if cards[i].get_rank() as u8 - cards[i-1].get_rank() as u8 != 1 {
                            is_four_straight = false;
                            break;
                        }
                    }
                    is_four_straight
                } {
                    if cards[4].get_rank() == Rank::Ace && cards[0].get_rank() == Rank::King {
                        HandRank::RoyalFlush
                    }
                    else if cards[4].get_rank() as u8 - cards[3].get_rank() as u8 != 1 {
                        if is_flush {
                            HandRank::StraightFlush {
                                // Use the Ace as the highest rank, because then we know it's a Royal straight.
                                highest_rank: cards[4].get_rank(),
                            }
                        }
                        else {
                            HandRank::Straight {
                                highest_rank: cards[0].get_rank(),
                            } 
                        }
                    }
                    else {
                        HandRank::Highest {
                            ranks: cards.into_iter().map(|c| c.get_rank()).collect::<Vec<_>>()[0..5].try_into().unwrap()
                        }
                    }
                }
                else {
                    if is_flush {
                        HandRank::Flush {
                            ranks: cards.into_iter().map(|c| c.get_rank()).collect::<Vec<_>>()[0..5].try_into().unwrap()
                        }
                    }
                    else {
                        HandRank::Highest {
                            ranks: cards.into_iter().map(|c| c.get_rank()).collect::<Vec<_>>()[0..5].try_into().unwrap()
                        }
                    }
                }

            }
            _ => panic!("Invalid hand.")
        }
    }
}