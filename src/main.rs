use rand::seq::SliceRandom;
use rand::thread_rng;

mod cards;
mod play_field;

use play_field::*;
use cards::*;


fn read_move() -> usize {
    println!("Input number");
    let cin = std::io::stdin();
    let mut input = String::new();
    cin.read_line(&mut input).unwrap();

    let value: usize = input.trim().parse().unwrap();
    value - 1
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
            println!("{}: {:?}", i, move_list[i-1]);
        }
        println!("{}: Draw next 3", move_list.len() + 1);
        let input = read_move();

        if input == move_list.len() {
            // Drawpile
            field.draw_cards();
            continue;
        } else if input < move_list.len() {
            field.move_card(move_list[input].0, move_list[input].1, move_list[input].2);
        } else {
            println!("Invalid move selected. Try Again!");
            continue;
        }


    }
    // Make game over smarter (so it doesn't keep moving same cards back and forth).
    // This may require BFS in order to determine if making a move leads to more moves
    // or just leads to back and forth.

    // Look into generics for simplifying drawpile/column code (Generic for Vec and VecDeque)

    // Need to beef up detection of looping of states, otherwise automation will loop infinite
    // Example, if moving
}
