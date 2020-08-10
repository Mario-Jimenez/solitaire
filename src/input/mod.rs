use crate::card::Card;
use crate::deck;
use crate::moves;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::process::exit;

fn game_over(piles: &Vec<Vec<Card>>) -> bool {
    for i in 9..13 {
        match piles.get(i) {
            Some(p) => {
                if p.len() < 13 {
                    return false;
                }
            }
            None => exit(1),
        }
    }
    true
}

fn user_move(piles: &mut Vec<Vec<Card>>, piles_index: usize) {
    let mut pile_index = 1;
    loop {
        let user_card = moves::get_user_card(&piles, piles_index, pile_index);
        match user_card {
            Some(c) => {
                let target_pile = moves::valid_moves(&piles, &c, piles_index);
                if target_pile == 0 {
                    if piles_index == 1 || piles_index > 8 {
                        println!("No moves");
                        return;
                    }
                    pile_index += 1;
                    continue;
                }
                if moves::move_card(piles, piles_index, target_pile, pile_index) {
                    return;
                }
                pile_index += 1;
            }
            None => {
                println!("No moves");
                return;
            }
        }
    }
}

pub fn actions(piles: &mut Vec<Vec<Card>>) {
    loop {
        if game_over(piles) {
            println!("Game Over!");
            return;
        }
        let key = read_key();
        print!("\x1B[2J\x1B[1;1H");
        match key {
            KeyCode::Char('h') => {
                user_move(piles, 1);
                deck::print_piles(piles);
            }
            KeyCode::Char('1') => {
                user_move(piles, 2);
                deck::print_piles(piles);
            }
            KeyCode::Char('2') => {
                user_move(piles, 3);
                deck::print_piles(piles);
            }
            KeyCode::Char('3') => {
                user_move(piles, 4);
                deck::print_piles(piles);
            }
            KeyCode::Char('4') => {
                user_move(piles, 5);
                deck::print_piles(piles);
            }
            KeyCode::Char('5') => {
                user_move(piles, 6);
                deck::print_piles(piles);
            }
            KeyCode::Char('6') => {
                user_move(piles, 7);
                deck::print_piles(piles);
            }
            KeyCode::Char('7') => {
                user_move(piles, 8);
                deck::print_piles(piles);
            }
            KeyCode::Char('q') => {
                user_move(piles, 9);
                deck::print_piles(piles);
            }
            KeyCode::Char('w') => {
                user_move(piles, 10);
                deck::print_piles(piles);
            }
            KeyCode::Char('e') => {
                user_move(piles, 11);
                deck::print_piles(piles);
            }
            KeyCode::Char('r') => {
                user_move(piles, 12);
                deck::print_piles(piles);
            }
            KeyCode::Enter => {
                moves::get_hand(piles);
                deck::print_piles(piles);
            }
            KeyCode::Esc => break,
            KeyCode::Char('n') => println!("n"),
            KeyCode::Char('N') => println!("N"),
            KeyCode::Char('u') => println!("u"),
            KeyCode::Char('U') => println!("U"),
            _ => {
                println!("Invalid command");
                deck::print_piles(piles);
            }
        }
    }
}

fn read_key() -> KeyCode {
    let mut user_input = KeyCode::Null;
    //going into raw mode
    enable_raw_mode().unwrap();
    //matching the key
    match read().unwrap() {
        Event::Key(KeyEvent { code, .. }) => user_input = code,
        _ => {}
    }

    //disabling raw mode
    disable_raw_mode().unwrap();

    user_input
}
