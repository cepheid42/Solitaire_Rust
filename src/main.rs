use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;

use Suit::*;

#[derive(Clone)]
struct Card {
    value: isize,
    suit: Suit
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
enum Suit {
    Spade = 0,
    Club,
    Heart,
    Diamond,
}

const DECK: [Card; 52] = [Card {value: 1, suit: Spade},
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

struct PlayField {
    first: Vec<&'static Card>,
    second: Vec<&'static Card>,
    sec_hid: Vec<&'static Card>,
    third: Vec<&'static Card>,
    thd_hid: Vec<&'static Card>,
    fourth: Vec<&'static Card>,
    for_hid: Vec<&'static Card>,
    fifth: Vec<&'static Card>,
    fif_hid: Vec<&'static Card>,
    sixth: Vec<&'static Card>,
    six_hid: Vec<&'static Card>,
    seventh: Vec<&'static Card>,
    sev_hid: Vec<&'static Card>,
    drawpile: Vec<&'static Card>,
    stack1: Vec<&'static Card>,
    stack2: Vec<&'static Card>,
    stack3: Vec<&'static Card>,
    stack4: Vec<&'static Card>,
}

trait Field {
    fn new(deck: &mut Vec<&'static Card>) -> PlayField;
    fn pop_card(&mut self, col: usize) -> Option<&'static Card>;
    fn push_card(&mut self, col: usize, card: &'static Card);
    fn move_card(&mut self, from_col: usize, to_col: usize);
    fn get_col(&self, col: usize) -> &Vec<&'static Card>;
    fn valid_moves(&mut self) -> Vec<(usize, usize, usize)>;
    fn game_over(&self) -> bool;

}

fn stackable(cur: &Card, other: &Card, fin: bool) -> bool {
    let mut stack: bool = false;
    if (other.value - cur.value) == 1 {
        if fin && cur.suit == other.suit {
            stack = true;
        }
        if !fin && (cur.suit == Spade || cur.suit == Club) && (other.suit == Heart || other.suit == Diamond) {
            stack = true;
        }
        if !fin && (cur.suit == Heart || cur.suit == Diamond) && (other.suit == Spade || other.suit == Club) {
            stack = true;
        }
    }
    stack
}

impl Field for PlayField {
    fn new(deck: &mut Vec<&'static Card>) -> PlayField {
        PlayField {
            first: vec![deck[0]],

            second: vec![deck[1]],
            sec_hid: vec![deck[2]],

            third: vec![deck[3]],
            thd_hid: vec![deck[4],
                          deck[5]],

            fourth: vec![deck[6]],
            for_hid: vec![deck[7],
                          deck[8],
                          deck[9]],

            fifth: vec![deck[10]],
            fif_hid: vec![deck[11],
                          deck[12],
                          deck[13],
                          deck[14]],

            sixth: vec![deck[15]],
            six_hid: vec![deck[16],
                          deck[17],
                          deck[18],
                          deck[19],
                          deck[20]],

            seventh: vec![deck[21]],
            sev_hid: vec![deck[22],
                          deck[23],
                          deck[24],
                          deck[25],
                          deck[26],
                          deck[27]],

            drawpile: vec![deck[28],
                           deck[29],
                           deck[30],
                           deck[31],
                           deck[32],
                           deck[33],
                           deck[34],
                           deck[35],
                           deck[36],
                           deck[37],
                           deck[38],
                           deck[39],
                           deck[40],
                           deck[41],
                           deck[42],
                           deck[43],
                           deck[44],
                           deck[45],
                           deck[46],
                           deck[47],
                           deck[48],
                           deck[40],
                           deck[50],
                           deck[51]],
            stack1: vec![],
            stack2: vec![],
            stack3: vec![],
            stack4: vec![],
        }
    }

    fn pop_card(&mut self, col: usize) -> Option<&'static Card> {
        match col {
            1 => self.first.pop(),
            2 => self.second.pop(),
            3 => self.third.pop(),
            4 => self.fourth.pop(),
            5 => self.fifth.pop(),
            6 => self.sixth.pop(),
            7 => self.seventh.pop(),
            8 => self.stack1.pop(),
            9 => self.stack2.pop(),
            10 => self.stack3.pop(),
            11 => self.stack4.pop(),
            12 => self.drawpile.pop(),
            13 => self.sec_hid.pop(),
            14 => self.thd_hid.pop(),
            15 => self.for_hid.pop(),
            16 => self.fif_hid.pop(),
            17 => self.six_hid.pop(),
            18 => self.sev_hid.pop(),
            _ => unreachable!(),
        }
    }

    fn push_card(&mut self, col: usize, card: &'static Card) {
        match col {
            1 => self.first.push(card),
            2 => self.second.push(card),
            3 => self.third.push(card),
            4 => self.fourth.push(card),
            5 => self.fifth.push(card),
            6 => self.sixth.push(card),
            7 => self.seventh.push(card),
            8 => self.stack1.push(card),
            9 => self.stack2.push(card),
            10 => self.stack3.push(card),
            11 => self.stack4.push(card),
            12 => self.drawpile.push(card),
            13 => self.sec_hid.push(card),
            14 => self.thd_hid.push(card),
            15 => self.for_hid.push(card),
            16 => self.fif_hid.push(card),
            17 => self.six_hid.push(card),
            18 => self.sev_hid.push(card),
            _ => unreachable!(),
        }
    }

    fn move_card(&mut self, from_col: usize, to_col: usize) {
        if from_col == to_col {
            println!("Invalid move to self");
            return
        }
        else {
            let card: &Card = self.pop_card(from_col).unwrap();
            self.push_card(to_col, card);
        }
    }

    fn get_col(&self, col: usize) -> &Vec<&'static Card> {
        match col {
            1 => &self.first,
            2 => &self.second,
            3 => &self.third,
            4 => &self.fourth,
            5 => &self.fifth,
            6 => &self.sixth,
            7 => &self.seventh,
            8 => &self.stack1,
            9 => &self.stack2,
            10 => &self.stack3,
            11 => &self.stack4,
            12 => &self.drawpile,
            _ => unreachable!(),
        }
    }

    fn valid_moves(&mut self) -> Vec<(usize, usize, usize)> {
        let mut move_list: Vec<(usize, usize, usize)> = Vec::new();

        // Iterate over columns
        for i in 1..=12 {
            let col: &Vec<&'static Card> = self.get_col(i);
            if col.is_empty() {
                continue;
            }

            // Iterate over column in reverse
            let col_start: usize;
            let col_end = col.len();

            // Set draw pile limit
            if i == 12 {
                col_start = col.len() - 1;
            } else {
                col_start = 0;
            }

            for j in (col_start..col_end).rev() {
                let cur_card: &Card = col[j];

                // Iterate over columns again, excluding drawpile
                for ii in 1..=11 {
//                    println!("{}",ii);
                    if i == ii {
//                        println!("Continue because i == ii");
                        continue
                    }
                    let target_col = self.get_col(ii);
                    if ii == 8 || ii == 9 || ii == 10 || ii == 11 {
                        // Check for Aces on empty stacks
                        if cur_card.value == 1 && target_col.is_empty() {
                            move_list.push((i, j, ii));
                            continue;
                        }
                    }
                    // Only check last card in target column
                    let target_card: &Card;
                    match target_col.last() {
                        Some(x) => target_card = x,
                        _ => continue,
                    }
                    // Check final stacks
                    if ii == 8 || ii == 9 || ii == 10 || ii == 11 {
                        if stackable(cur_card, target_card, true) {
                            move_list.push((i, j, ii));
                        }
                    } else {
                        // Check stackable on all other columns
                        if stackable(cur_card, target_card, false) {
                            move_list.push((i, j, ii));
                        }
                    }
                }
            }
        }
        move_list
    }

    fn game_over(&self) -> bool{
        unimplemented!();

    }
}

fn main() {
    let mut rng = thread_rng();
    let mut deck: Vec<&'static Card> = DECK.iter().collect();
    deck.shuffle(&mut rng);

    let mut field = PlayField::new(&mut deck);
//    for card in deck.iter() {
//        println!("{}", card)
//    }


    for i in 1..=12 {
        let col = field.get_col(i);
        println!("{}: {:?}", i, col);
    }
//    field.move_card(1, 2);
//    println!("{:?}", field.first);
//    println!("{:?}", field.second);

    let moves = field.valid_moves();
    println!("{:?}", moves);
}
