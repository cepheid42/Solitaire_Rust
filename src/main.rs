use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

mod cards;
mod play_field;

use play_field::*;
use cards::*;


fn read_move() -> usize {
    println!("Input number");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);

    let value: usize = input.trim().parse().unwrap();
    value - 1
}

fn is_valid_move(m: (usize, usize, usize), move_list: Vec<(usize, usize, usize)>) -> bool {
    if move_list.contains(&m){
        true
    } else {
        false
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut deck: Vec<&'static Card> = DECK.iter().collect();
    deck.shuffle(&mut rng);

    let draw_size = 3;
    let mut field = PlayField::new(&mut deck, draw_size);

    while !field.game_over() {
        let move_list: Vec<(usize, usize, usize)> = field.valid_moves();
        if move_list.is_empty() {
            println!("{:?}", field);
            println!("No valid moves left\nGame Over.");
            break;
        }

        println!("{:?}", field);
        println!("Choose move:");
        for i in 1..=move_list.len() {
            println!("{}: {:?}", i, move_list);
        }
        println!("{}: Draw next 3", move_list.len() + 1);
        let input = read_move();

        if input == move_list.len() {
            // Drawpile
        } else {
            field.move_card(move_list[input].0, move_list[input].1, move_list[input].2);
        }


    }
    // Make game over smarter (so it doesn't keep moving same cards back and forth).
    // This may require BFS in order to determine if making a move leads to more moves
    // or just leads to back and forth.
    // -- Move from one stack to another are not put in valid move list

    // Moved from drawpile to C1 automatically, think I fixed it.

    // Move from index other than 0 doesn't work properly, moves card(s) below target card.
    // -- Should be fixed.

    // Redo inputs so that drawpile can be scrolled through.
    // -- Used VecDeque, make sure refactored everything correctly
    // -- Update debug print to print slice
    // -- rotate and pop from front, card moves and stuff need checking
}
