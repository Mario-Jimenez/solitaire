use crate::card::Card;
use chrono::prelude::*;
use simplelog::*;
use std::fs;
use std::fs::File;
use std::process::exit;

pub fn init() {
    fs::create_dir_all("logs").unwrap();

    let dt = Utc::now();
    let file_name = format!(
        "logs/solitaire_{}.log",
        dt.format("%Y-%m-%d_%H:%M:%S").to_string()
    );

    let mut config = simplelog::ConfigBuilder::new();

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        config.build(),
        File::create(file_name).unwrap(),
    )])
    .unwrap();
}

pub fn piles_to_log(piles: &Vec<Vec<Card>>) -> String {
    let mut log_string = String::from("");
    top_to_log(piles, &mut log_string);
    let largest = largest_tableau(piles);
    tableaus_to_log(piles, largest, &mut log_string);
    return log_string;
}

fn top_to_log(piles: &Vec<Vec<Card>>, log_string: &mut String) {
    last_to_log(piles, 0, log_string);
    last_to_log(piles, 1, log_string);
    log_string.push_str("    ");
    last_to_log(piles, 9, log_string);
    last_to_log(piles, 10, log_string);
    last_to_log(piles, 11, log_string);
    last_to_log(piles, 12, log_string);
}

fn last_to_log(piles: &Vec<Vec<Card>>, index: usize, log_string: &mut String) {
    match piles.get(index) {
        Some(pile) => {
            let size = pile.len();
            if size == 0 {
                log_string.push_str("___ ");
                return;
            }
            match pile.get(size - 1) {
                Some(card) => card.log(log_string),
                None => exit(1),
            }
        }
        None => exit(1),
    }
}

fn largest_tableau(piles: &Vec<Vec<Card>>) -> usize {
    let mut max = 0;
    for i in 2..9 {
        match piles.get(i) {
            Some(pile) => {
                let temp = pile.len();
                if max < temp {
                    max = temp;
                }
            }
            None => exit(1),
        }
    }
    max
}

fn tableaus_to_log(piles: &Vec<Vec<Card>>, largest: usize, log_string: &mut String) {
    for n in 0..largest {
        log_string.push_str("\n");
        for i in 2..9 {
            match piles.get(i) {
                Some(pile) => match pile.get(n) {
                    Some(card) => card.log(log_string),
                    None => log_string.push_str("    "),
                },
                None => exit(1),
            }
        }
    }
}
