use std::fmt;
use crate::cards::*;
use crate::cards::Suit::*;
use std::collections::VecDeque;
use std::iter::FromIterator;


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
    drawpile: VecDeque<&'static Card>,
    stack1: Vec<&'static Card>,
    stack2: Vec<&'static Card>,
    stack3: Vec<&'static Card>,
    stack4: Vec<&'static Card>,
    draw_size: usize,
}

pub trait Field {
    fn new(deck: &mut Vec<&'static Card>, draw_size: usize) -> PlayField;
    fn pop_card(&mut self, col: usize) -> Option<&'static Card>;
    fn push_card(&mut self, col: usize, card: &'static Card);
    fn move_card(&mut self, fc: usize, ind: usize, tc: usize);
    fn get_col(&self, col: usize) -> &Vec<&'static Card>;
    fn get_drawpile(&self) -> &VecDeque<&'static Card>;
    fn valid_moves(&mut self) -> Vec<(usize, usize, usize)>;
    fn draw_cards(&mut self);
    fn game_over(&self) -> bool;
    fn check_empty(&self, tc: usize) -> bool;
}

fn stackable(cur: &Card, other: &Card, tc: usize) -> bool {
    let mut stackable: bool = false;
    // Target is a stack
    if (other.value - cur.value) == 1 {
        // Stacks
        if tc >= 8 && cur.suit == other.suit {
            stackable = true;
        }
        // Columns
        else if tc < 8 && (cur.suit == Spade || cur.suit == Club) && (other.suit == Heart || other.suit == Diamond) {
            stackable = true;
        }
        else if tc < 8 && (cur.suit == Heart || cur.suit == Diamond) && (other.suit == Spade || other.suit == Club) {
            stackable = true;
        }
        // Catch all
        else {
            stackable = false;
        }
    }
    stackable
}

impl Field for PlayField {
    fn new(deck: &mut Vec<&'static Card>, draw_size: usize) -> PlayField {
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

            drawpile: VecDeque::from_iter(vec![deck[28],
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
                                                deck[51]]),
            stack1: vec![],
            stack2: vec![],
            stack3: vec![],
            stack4: vec![],
            draw_size,
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
            12 => self.sec_hid.pop(),
            13 => self.thd_hid.pop(),
            14 => self.for_hid.pop(),
            15 => self.fif_hid.pop(),
            16 => self.six_hid.pop(),
            17 => self.sev_hid.pop(),
            18 => self.drawpile.pop_front(),
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
            12 => self.sec_hid.push(card),
            13 => self.thd_hid.push(card),
            14 => self.for_hid.push(card),
            15 => self.fif_hid.push(card),
            16 => self.six_hid.push(card),
            17 => self.sev_hid.push(card),
            _ => unreachable!(),
        }
    }

    fn move_card(&mut self, fc: usize, ci: usize, tc: usize) {
        // Move from drawpile
        if fc == 18 {
            let card: &Card = self.pop_card(fc).unwrap();
            self.push_card(tc, card);
        } else {
            let fcol: &Vec<&Card> = self.get_col(fc);
            let col_len: usize = fcol.len();

            // More than one card on the column
            if col_len > 1 {
                let mut temp: Vec<&Card> = Vec::new();

                for _i in (ci..=(col_len - 1)).rev() {
                    let card: &Card = self.pop_card(fc).unwrap();
                    temp.push(card);
                }
                while !temp.is_empty() {
                    let card: &Card = temp.pop().unwrap();
                    self.push_card(tc, card);
                }

            } else {
                let card: &Card = self.pop_card(fc).unwrap();
                self.push_card(tc, card);
            }

            // Pop from hidden col to empty fc
            if (fc != 1 && fc < 8) && self.check_empty(fc) {
                let hid_col: &Vec<&Card> = self.get_col(fc + 10);
                if hid_col.is_empty() {
                    return
                } else {
                    let c: &Card = self.pop_card(fc + 10).unwrap();
                    self.push_card(fc, c);
                }
            }
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
            12 => &self.sec_hid,
            13 => &self.thd_hid,
            14 => &self.for_hid,
            15 => &self.fif_hid,
            16 => &self.six_hid,
            17 => &self.sev_hid,
            _ => unreachable!(),
        }
    }

    fn get_drawpile(&self) -> &VecDeque<&'static Card> {
        &self.drawpile
    }

    fn valid_moves(&mut self) -> Vec<(usize, usize, usize)> {
        let mut move_list: Vec<(usize, usize, usize)> = Vec::new();

        // Iterate over columns
        for fc in 1..=7 {
            let from_col: &Vec<&Card> = self.get_col(fc);
            if from_col.is_empty() {
                continue;
            }
            // Iterate over columns and stacks
            for tc in 1..=11 {
                // Skip moves to self
                if fc == tc {
                    continue;
                }
                let target_col: &Vec<&Card> = self.get_col(tc);
                // Iterate over cards in column
                for ci in (0..from_col.len()).rev() {
                    let cur_card: &Card = from_col[ci];

                    if target_col.is_empty() {
                        // Only Kings are valid on empty columns
                        if tc <= 7 && cur_card.value == 13 {
                            if !move_list.contains(&(fc, ci, tc)) {
                                move_list.push((fc, ci, tc));
                            }
                        }
                        // Only Aces are valid on empty stacks
                        else if tc >= 8 && tc <= 11 && cur_card.value == 1 {
                            if !move_list.contains(&(fc, ci, tc)) {
                                move_list.push((fc, ci, tc));
                            }
                        }
                        else {
                            continue;
                        }
                    // Non-empty column/stack
                    } else {
                        let target_card: &Card = target_col.last().unwrap();
                        if stackable(cur_card, target_card, tc) {
                            if !move_list.contains(&(fc, ci, tc)) {
                                move_list.push((fc, ci, tc));
                            }
                        }
                    }
                }
            }

            // Iterate over Stacks
            for fc in 8..=11 {
                let from_col: &Vec<&Card> = self.get_col(fc);
                if from_col.is_empty() {
                    continue;
                }
                for tc in 1..=7 {
                    let ci = from_col.len() - 1;
                    let cur_card: &Card = from_col[ci];
                    // Ignores Aces and Twos as valid moves from stacks
                    // This should help limit infinite move loops
                    // because these moves have no strategic value in the game
                    if cur_card.value == 1 || cur_card.value == 2 {
                        break;
                    }
                    let target_col: &Vec<&Card> = self.get_col(tc);
                    // Only Kings are valid on empty Columns
                    if target_col.is_empty() && cur_card.value == 13 {
                        if !move_list.contains(&(fc, ci, tc)) {
                            move_list.push((fc, ci, tc));
                        }
                    }
                    else {
                        let target_card: &Card = target_col.last().unwrap();
                        if stackable(cur_card, target_card, tc) {
                            if !move_list.contains(&(fc, ci, tc)) {
                                move_list.push((fc, ci, tc));
                            }
                        }
                    }
                }
            }
        }

        let dp: &VecDeque<&Card> = self.get_drawpile();
        if !dp.is_empty() {
            let cur_card: &Card = dp[0];
            // Iterate over target columns/stacks
            for tc in 1..=11 {
                let target_col = self.get_col(tc);
                // Check if target column is empty
                if target_col.is_empty() {
                    // Only Aces are valid on empty stacks
                    if tc >= 8 && cur_card.value == 1 {
                        if !move_list.contains(&(18, 0, tc)) {
                            move_list.push((18, 0, tc));
                        }
                    }
                    else if tc < 8 && cur_card.value == 13 {
                        // Only Kings are valid in the case
                        if !move_list.contains(&(18, 0, tc)) {
                            move_list.push((18, 0, tc));
                        }
                    }
                    else {
                        continue;
                    }
                // Non-empty target column
                } else {
                    let target_card: &Card = target_col.last().unwrap();
                    if stackable(cur_card, target_card, tc) {
                        if !move_list.contains(&(18, 0, tc)) {
                            move_list.push((18, 0, tc));
                        }
                    }
                }
            }
        }
        move_list
    }

    fn draw_cards(&mut self) {
        self.drawpile.rotate_left(self.draw_size);

    }

    fn game_over(&self) -> bool {
        if self.stack1.len() == 12 && self.stack2.len() == 12 && self.stack3.len() == 12 && self.stack4.len() == 12 {
            true
        } else {
            false
        }
    }

    fn check_empty(&self, tc: usize) -> bool {
        let from_col: &Vec<&Card> = self.get_col(tc);
        from_col.is_empty()
    }
}

impl fmt::Debug for PlayField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 1..=12 {
            // Prints first three cards in draw pile
            if i == 12 {
                let dp: &VecDeque<&Card> = self.get_drawpile();
                println!("D: {:?}, {:?}, {:?}", &dp[0], &dp[1], &dp[2]);
            // Prints Stacks normally
            } else if i >= 8 && i <= 11 {
                let col: &Vec<&Card> = self.get_col(i);
                println!("S{}: {:?}", i-7 , col);
            // Print Column and Hidden stacks
            } else {
                let col: &Vec<&Card> = self.get_col(i);
                println!("\nC{}: {:?}", i, col);
                if i < 8 && i > 1 {
                    println!("H{}: {:?}", i, self.get_col(i+10));
                }
            }
        }
        write!(f, "{}", "")
    }
}