use std::fmt;

#[derive(Clone)]
pub struct Card {
    pub value: isize,
    pub suit: Suit
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val: &str;
        match self.value {
            1 => val = "A",
            2 => val = "2",
            3 => val = "3",
            4 => val = "4",
            5 => val = "5",
            6 => val = "6",
            7 => val = "7",
            8 => val = "8",
            9 => val = "9",
            10 => val = "10",
            11 => val = "J",
            12 => val = "Q",
            13 => val = "K",
            _ => unreachable!(),
        }
        let suit: &str;
        match self.suit {
            Spade => suit = "\u{2660}",
            Club => suit = "\u{2663}",
            Heart => suit = "\u{2665}",
            Diamond => suit = "\u{2666}",
        }
        write!(f, "{}{}", val, suit)
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Suit {
    Spade = 0,
    Club,
    Heart,
    Diamond,
}

use Suit::*;
pub const DECK: [Card; 52] = [Card {value: 1, suit: Spade},
                                Card {value: 2, suit: Spade},
                                Card {value: 3, suit: Spade},
                                Card {value: 4, suit: Spade},
                                Card {value: 5, suit: Spade},
                                Card {value: 6, suit: Spade},
                                Card {value: 7, suit: Spade},
                                Card {value: 8, suit: Spade},
                                Card {value: 9, suit: Spade},
                                Card {value: 10, suit: Spade},
                                Card {value: 11, suit: Spade},
                                Card {value: 12, suit: Spade},
                                Card {value: 13, suit: Spade},
                                Card {value: 1, suit: Club},
                                Card {value: 2, suit: Club},
                                Card {value: 3, suit: Club},
                                Card {value: 4, suit: Club},
                                Card {value: 5, suit: Club},
                                Card {value: 6, suit: Club},
                                Card {value: 7, suit: Club},
                                Card {value: 8, suit: Club},
                                Card {value: 9, suit: Club},
                                Card {value: 10, suit: Club},
                                Card {value: 11, suit: Club},
                                Card {value: 12, suit: Club},
                                Card {value: 13, suit: Club},
                                Card {value: 1, suit: Heart},
                                Card {value: 2, suit: Heart},
                                Card {value: 3, suit: Heart},
                                Card {value: 4, suit: Heart},
                                Card {value: 5, suit: Heart},
                                Card {value: 6, suit: Heart},
                                Card {value: 7, suit: Heart},
                                Card {value: 8, suit: Heart},
                                Card {value: 9, suit: Heart},
                                Card {value: 10, suit: Heart},
                                Card {value: 11, suit: Heart},
                                Card {value: 12, suit: Heart},
                                Card {value: 13, suit: Heart},
                                Card {value: 1, suit: Diamond},
                                Card {value: 2, suit: Diamond},
                                Card {value: 3, suit: Diamond},
                                Card {value: 4, suit: Diamond},
                                Card {value: 5, suit: Diamond},
                                Card {value: 6, suit: Diamond},
                                Card {value: 7, suit: Diamond},
                                Card {value: 8, suit: Diamond},
                                Card {value: 9, suit: Diamond},
                                Card {value: 10, suit: Diamond},
                                Card {value: 11, suit: Diamond},
                                Card {value: 12, suit: Diamond},
                                Card {value: 13, suit: Diamond}];