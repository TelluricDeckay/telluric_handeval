[![Rust](https://github.com/TelluricDeckay/telluric_handeval/workflows/Rust/badge.svg?branch=trunk)](https://github.com/TelluricDeckay/telluric_handeval/actions?query=workflow%3ARust)

# telluric_handeval

Hand Evaluation Library

## Examples

### Evaluate a hand to determine its poker rank

```rust
use telluric_handeval::poker::{HandRank, PokerRankedHand};
use ionic_deckhandler::{Card, Deck, Rank, Suit};

// This hand will be ranked as a flush

let hand_arr1: [Card; 5] = [
    Card::new(Rank::Queen, Suit::Clubs),
    Card::new(Rank::Five, Suit::Clubs),
    Card::new(Rank::Four, Suit::Clubs),
    Card::new(Rank::King, Suit::Clubs),
    Card::new(Rank::Three, Suit::Clubs),
];
assert_eq!(
    hand_arr1.evaluate_hand(),
    HandRank::Flush {
        ranks: [Rank::King, Rank::Queen, Rank::Five, Rank::Four, Rank::Three]
    }
);

// This hand will be ranked by the highest card

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
```

### Compare two hands to determine which ranks higher

```rust
use ionic_deckhandler::{Card, Rank, Suit};
use std::cmp::Ordering;
use telluric_handeval::poker::{HandRank, PokerRankedHand};

// A pair of threes, Ace high
let hand_arr1: [Card; 5] = [
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

// A Pair of threes, King high
let hand_arr2: [Card; 5] = [
        Card::new(Rank::Two, Suit::Clubs),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
    ];

// Flush, Ace high
let hand_arr3 = [
    Card::new(Rank::Queen, Suit::Hearts),
    Card::new(Rank::Five, Suit::Hearts),
    Card::new(Rank::Four, Suit::Hearts),
    Card::new(Rank::King, Suit::Hearts),
    Card::new(Rank::Ace, Suit::Hearts),
];

// Flush, King high (Clubs)
let hand_arr4: [Card; 5] = [
    Card::new(Rank::Queen, Suit::Clubs),
    Card::new(Rank::Five, Suit::Clubs),
    Card::new(Rank::Four, Suit::Clubs),
    Card::new(Rank::King, Suit::Clubs),
    Card::new(Rank::Three, Suit::Clubs),
];

// Flush, King high (Hearts)
let hand_arr5: [Card; 5] = [
    Card::new(Rank::Queen, Suit::Hearts),
    Card::new(Rank::Five, Suit::Hearts),
    Card::new(Rank::Four, Suit::Hearts),
    Card::new(Rank::King, Suit::Hearts),
    Card::new(Rank::Three, Suit::Hearts),
];

assert_eq!(
        hand_arr1.evaluate_hand().cmp(&hand_arr2.evaluate_hand()),
        Ordering::Greater
    );

assert_eq!(
    hand_arr4.evaluate_hand().cmp(&hand_arr3.evaluate_hand()),
    Ordering::Less
);

assert_eq!(
    hand_arr4.evaluate_hand().cmp(&hand_arr2.evaluate_hand()),
    Ordering::Greater
);

assert_eq!(
    hand_arr4.evaluate_hand().cmp(&hand_arr5.evaluate_hand()),
    Ordering::Equal
);
```

## Contributing

See
[CONTRIBUTING.md](https://github.com/TelluricDeckay/telluric_handeval/blob/trunk/CONTRIBUTING.md)

## Help and Support

* [Discussions](https://github.com/TelluricDeckay/telluric_handeval/discussions)
* [Issues](https://github.com/TelluricDeckay/telluric_handeval/issues)
* [Chat room](https://telluric-deckay.zulipchat.com/)
* Emails listed in [Cargo.toml](https://github.com/TelluricDeckay/telluric_handeval/blob/trunk/Cargo.toml)
