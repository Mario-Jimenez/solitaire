use crate::card::Card;
use crate::deck;
use crate::logger;
use crate::moves;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use log::info;
use rand::Rng;
use std::env;
use std::process::exit;
use undo::{Command, Record};

#[derive(Debug)]
struct Add(Vec<Vec<Card>>);

impl Command<Vec<Vec<Vec<Card>>>> for Add {
    fn apply(&mut self, s: &mut Vec<Vec<Vec<Card>>>) -> undo::Result {
        s.push(self.0.clone());
        Ok(())
    }

    fn undo(&mut self, s: &mut Vec<Vec<Vec<Card>>>) -> undo::Result {
        self.0 = s.pop().ok_or("s is empty")?;
        Ok(())
    }
}

pub fn start_game() {
    let args: Vec<String> = env::args().collect();
    let mut game: u64 = 0;
    if args.len() > 1 {
        match args[1].parse::<u64>() {
            Ok(n) => game = n,
            Err(_) => {}
        }
    }
    new_game(game);
}

fn new_game(mut game: u64) {
    if game == 0 {
        let mut rng = rand::thread_rng();
        game = rng.gen_range(1, 1000000);
    }
    let mut piles = deck::set_up(game);
    print!("\x1B[2J\x1B[1;1H");
    deck::print_piles(&piles);
    let mut record = Record::default();
    match record.apply(Add(piles.clone())) {
        Ok(_) => {}
        Err(_) => exit(1),
    }

    info!("New Game!\n{}", logger::piles_to_log(&piles));

    actions(&mut piles, &mut record);
}

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

fn user_move(piles: &mut Vec<Vec<Card>>, piles_index: usize) -> bool {
    let mut pile_index = 1;
    loop {
        let user_card = moves::get_user_card(&piles, piles_index, pile_index);
        match user_card {
            Some(c) => {
                let target_pile = moves::valid_moves(&piles, &c, piles_index);
                if target_pile == 0 {
                    if piles_index == 1 || piles_index > 8 {
                        info!(
                            "No moves for {}\n{}",
                            piles_index_to_str(piles_index),
                            logger::piles_to_log(&piles)
                        );
                        println!("No moves");
                        return false;
                    }
                    pile_index += 1;
                    continue;
                }
                if moves::move_card(piles, piles_index, target_pile, pile_index) {
                    info!(
                        "From {} to {}\n{}",
                        piles_index_to_str(piles_index),
                        piles_index_to_str(target_pile),
                        logger::piles_to_log(&piles)
                    );
                    return true;
                }
                pile_index += 1;
            }
            None => {
                info!(
                    "No moves for {}\n{}",
                    piles_index_to_str(piles_index),
                    logger::piles_to_log(&piles)
                );
                println!("No moves");
                return false;
            }
        }
    }
}

fn piles_index_to_str(index: usize) -> String {
    match index {
        0 => String::from("Hand"),
        1 => String::from("Waste"),
        2 => String::from("1"),
        3 => String::from("2"),
        4 => String::from("3"),
        5 => String::from("4"),
        6 => String::from("5"),
        7 => String::from("6"),
        8 => String::from("7"),
        9 => String::from("q"),
        10 => String::from("w"),
        11 => String::from("e"),
        12 => String::from("r"),
        _ => exit(1),
    }
}

pub fn actions(piles: &mut Vec<Vec<Card>>, record: &mut Record<Vec<Vec<Vec<Card>>>>) {
    loop {
        if game_over(piles) {
            println!("Game Over!");
            info!("Game Over!\n{}", logger::piles_to_log(&piles));
            return;
        }
        let key = read_key();
        print!("\x1B[2J\x1B[1;1H");
        match key {
            KeyCode::Char('h') => {
                add_record(piles, 1, record);
            }
            KeyCode::Char('1') => {
                add_record(piles, 2, record);
            }
            KeyCode::Char('2') => {
                add_record(piles, 3, record);
            }
            KeyCode::Char('3') => {
                add_record(piles, 4, record);
            }
            KeyCode::Char('4') => {
                add_record(piles, 5, record);
            }
            KeyCode::Char('5') => {
                add_record(piles, 6, record);
            }
            KeyCode::Char('6') => {
                add_record(piles, 7, record);
            }
            KeyCode::Char('7') => {
                add_record(piles, 8, record);
            }
            KeyCode::Char('q') => {
                add_record(piles, 9, record);
            }
            KeyCode::Char('w') => {
                add_record(piles, 10, record);
            }
            KeyCode::Char('e') => {
                add_record(piles, 11, record);
            }
            KeyCode::Char('r') => {
                add_record(piles, 12, record);
            }
            KeyCode::Enter => {
                moves::get_hand(piles);
                match record.apply(Add(piles.clone())) {
                    Ok(_) => {}
                    Err(_) => exit(1),
                }
                info!(
                    "From {} to {}\n{}",
                    piles_index_to_str(0),
                    piles_index_to_str(1),
                    logger::piles_to_log(&piles)
                );
                deck::print_piles(piles);
            }
            KeyCode::Esc => break,
            KeyCode::Char('n') => {
                new_game(0);
                break;
            }
            KeyCode::Char('N') => {
                new_game(0);
                break;
            }
            KeyCode::Char('u') => {
                if record.current() < 2 {
                    continue;
                }
                match record.undo() {
                    Ok(_) => {}
                    Err(_) => exit(1),
                }
                match record.target().get(record.current() - 1) {
                    Some(r) => {
                        let mut record_piles = r.clone();
                        info!("Undo\n{}", logger::piles_to_log(&record_piles));
                        deck::print_piles(&record_piles);
                        actions(&mut record_piles, record)
                    }
                    None => exit(1),
                }
                break;
            }
            KeyCode::Char('U') => {
                if record.current() < 2 {
                    continue;
                }
                match record.undo() {
                    Ok(_) => {}
                    Err(_) => exit(1),
                }
                match record.target().get(record.current() - 1) {
                    Some(r) => {
                        let mut record_piles = r.clone();
                        info!("Undo\n{}", logger::piles_to_log(&record_piles));
                        deck::print_piles(&record_piles);
                        actions(&mut record_piles, record)
                    }
                    None => exit(1),
                }
                break;
            }
            KeyCode::Char('i') => {
                if record.current() == record.len() {
                    continue;
                }
                match record.redo() {
                    Ok(_) => {}
                    Err(_) => exit(1),
                }
                match record.target().get(record.current() - 1) {
                    Some(r) => {
                        let mut record_piles = r.clone();
                        info!("Redo\n{}", logger::piles_to_log(&record_piles));
                        deck::print_piles(&record_piles);
                        actions(&mut record_piles, record)
                    }
                    None => exit(1),
                }
                break;
            }
            KeyCode::Char('I') => {
                match record.redo() {
                    Ok(_) => {}
                    Err(_) => exit(1),
                }
                match record.target().get(record.current() - 1) {
                    Some(r) => {
                        let mut record_piles = r.clone();
                        info!("Redo\n{}", logger::piles_to_log(&record_piles));
                        deck::print_piles(&record_piles);
                        actions(&mut record_piles, record)
                    }
                    None => exit(1),
                }
                break;
            }
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

fn add_record(
    piles: &mut Vec<Vec<Card>>,
    piles_index: usize,
    record: &mut Record<Vec<Vec<Vec<Card>>>>,
) {
    if user_move(piles, piles_index) {
        match record.apply(Add(piles.clone())) {
            Ok(_) => {}
            Err(_) => exit(1),
        }
    }
    deck::print_piles(piles);
}
