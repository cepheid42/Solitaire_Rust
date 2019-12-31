use std::fmt;
use crate::cards::*;
use Suit::*;

pub struct PlayField {
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

pub trait Field {
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
            // Iterate over cards in column
            for j in (col_start..col_end).rev() {
                let cur_card: &Card = col[j];

                // Iterate over target columns, excluding draw pile
                for ii in 1..=11 {
                    if i == ii {
                        continue
                    }
                    let target_col = self.get_col(ii);

                    if target_col.is_empty() {
                        if ii == 8 || ii == 9 || ii == 10 || ii == 11 {
                            if cur_card.value == 13 {
                                // not valid
                                continue;
                            } else if cur_card.value == 1 {
                                // valid
                                move_list.push((i, j, ii));
                            }
                        } else {
                            if cur_card.value == 13 {
                                // kings on empty columns
                                move_list.push((i, j, ii));
                            }
                        }
                    } else {
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
        }
        move_list
    }

    fn game_over(&self) -> bool {
        if self.stack1.len() == 12 && self.stack2.len() == 12 && self.stack3.len() == 12 && self.stack4.len() == 12 {
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for PlayField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 1..=12 {
            let col: &Vec<&Card> = self.get_col(i);
            if i == 12 {
                print!("{}: [", i);
                for (x, card) in col.iter().rev().enumerate() {
                    if x == 2 {
                        print!("{:?}", card);
                        break;
                    } else {
                        print!("{:?}", card);
                    }
                    print!(", ")
                }
                print!("]");
            } else {
                println!("{}: {:?}", i, col);
            }
        }
        write!(f, "{}", "")
    }
}