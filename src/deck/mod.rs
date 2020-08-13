use crate::card::{Card, CardType, CardValue};
use rand::seq::SliceRandom;
use rand::{rngs::StdRng, SeedableRng};
use std::process::exit;

pub fn new_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::with_capacity(52);
    load_suit(&mut deck, CardType::Corazones);
    load_suit(&mut deck, CardType::Espadas);
    load_suit(&mut deck, CardType::Diamantes);
    load_suit(&mut deck, CardType::Treboles);
    return deck;
}

fn load_suit(deck: &mut Vec<Card>, suit: CardType) {
    let a_card = Card::new(suit, CardValue::A);
    deck.push(a_card);
    for number in 2..10 {
        let number_card = Card::new(suit, CardValue::N(number));
        deck.push(number_card);
    }
    let z_card = Card::new(suit, CardValue::Z);
    deck.push(z_card);
    let j_card = Card::new(suit, CardValue::J);
    deck.push(j_card);
    let q_card = Card::new(suit, CardValue::Q);
    deck.push(q_card);
    let k_card = Card::new(suit, CardValue::K);
    deck.push(k_card);
}

pub fn shuffle_deck(deck: &mut Vec<Card>, game: u64) {
    let mut rng = StdRng::seed_from_u64(game);
    deck.shuffle(&mut rng);
}

pub fn set_up(game: u64) -> Vec<Vec<Card>> {
    let mut piles: Vec<Vec<Card>> = Vec::with_capacity(13);
    let mut initial_deck = new_deck();
    shuffle_deck(&mut initial_deck, game);

    create_tableaus(&mut piles, &mut initial_deck);

    let empty_deck: Vec<Card> = Vec::new();

    piles.insert(0, initial_deck);
    piles.insert(1, empty_deck.clone());

    let empty_foundation: Vec<Card> = Vec::with_capacity(13);
    for _ in 0..4 {
        piles.push(empty_foundation.clone());
    }

    piles
}

fn create_tableaus(piles: &mut Vec<Vec<Card>>, deck: &mut Vec<Card>) {
    for i in 1..8 {
        piles.push(create_tableau(i, deck));
    }
}

fn create_tableau(size: u8, deck: &mut Vec<Card>) -> Vec<Card> {
    let mut tableau: Vec<Card> = Vec::with_capacity(13);
    for i in 0..size {
        let card = deck.pop();
        match card {
            Some(mut x) => {
                if i == size - 1 {
                    x.show();
                }
                tableau.push(x)
            }
            None => exit(1),
        }
    }
    tableau
}

pub fn print_piles(piles: &Vec<Vec<Card>>) {
    print_top(piles);
    let largest = largest_tableau(piles);
    print_tableaus(piles, largest);
    println!();
}

fn print_top(piles: &Vec<Vec<Card>>) {
    print_last(piles, 0);
    print_last(piles, 1);
    print!("    ");
    print_last(piles, 9);
    print_last(piles, 10);
    print_last(piles, 11);
    print_last(piles, 12);
}

fn print_last(piles: &Vec<Vec<Card>>, index: usize) {
    match piles.get(index) {
        Some(pile) => {
            let size = pile.len();
            if size == 0 {
                print!("___ ");
                return;
            }
            match pile.get(size - 1) {
                Some(card) => print!("{} ", card),
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

fn print_tableaus(piles: &Vec<Vec<Card>>, largest: usize) {
    for n in 0..largest {
        println!();
        for i in 2..9 {
            match piles.get(i) {
                Some(pile) => match pile.get(n) {
                    Some(card) => print!("{} ", card),
                    None => print!("    "),
                },
                None => exit(1),
            }
        }
    }
}
