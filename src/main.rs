use rand::seq::SliceRandom;
use rand::thread_rng;

#[macro_use]
extern crate text_io;

mod cards;
mod play_field;

use play_field::*;
use cards::*;

fn read_move() -> usize {
    let mut char_ind: usize = 100;
    while char_ind > 50 {
        char_ind = read!();
    }
    char_ind
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
    let mut field = PlayField::new(&mut deck);


    while !field.game_over() {
        let move_list: Vec<(usize, usize, usize)> = field.valid_moves();
        println!("{:?}", field);
        println!("{:?}", move_list);
        println!("Choose From column.");
        let from: usize = read_move();
        println!("Choose card index");
        let ind: usize = read_move();
        println!("Choose To column");
        let to: usize = read_move();
        if move_list.is_empty() {
            println!("No valid moves left\nGame Over.");
            break;
        }
        if is_valid_move((from, ind, to), move_list) {
            field.move_card(from, to);
        }

    }
    // Make game over smarter (so it doesn't keep moving same cards back and forth).
    // Add auto flip on empty columns
    // Make move function, takes all cards below (above?) index with it (move multiple cards)
    //
}
