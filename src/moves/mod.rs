use crate::card::{Card, CardColor, CardType, CardValue};
use std::process::exit;

pub fn get_hand(piles: &mut Vec<Vec<Card>>) {
    let hand: &mut Vec<Card>;
    let mut waste: &mut Vec<Card>;

    match piles.get_mut(0) {
        Some(p) => hand = p,
        None => exit(1),
    }

    if hand.is_empty() {
        match piles.get_mut(1) {
            Some(p) => waste = p,
            None => exit(1),
        }
        waste.reverse();
        hide_pile(&mut waste);
        piles.swap(0, 1);
        return;
    }

    let mut card: Card;
    match hand.pop() {
        Some(c) => card = c,
        None => exit(1),
    }

    card.show();

    match piles.get_mut(1) {
        Some(p) => p.push(card),
        None => exit(1),
    }
}

fn hide_pile(pile: &mut Vec<Card>) {
    if !pile.is_empty() {
        for i in 0..pile.len() {
            match pile.get_mut(i) {
                Some(c) => c.hide(),
                None => exit(1),
            }
        }
    }
}

pub fn move_card(
    piles: &mut Vec<Vec<Card>>,
    source: usize,
    target: usize,
    quantity: usize,
) -> bool {
    if target > 8 && quantity > 1 {
        return false;
    }

    let source_pile: &mut Vec<Card>;
    let mut cards: Vec<Card> = Vec::new();

    match piles.get_mut(source) {
        Some(p) => source_pile = p,
        None => return false,
    }

    for _ in 0..quantity {
        match source_pile.pop() {
            Some(c) => cards.push(c),
            None => exit(1),
        }
    }

    let source_pile_size = source_pile.len();
    if source_pile_size > 0 {
        match source_pile.get_mut(source_pile_size - 1) {
            Some(c) => c.show(),
            None => exit(1),
        }
    }

    match piles.get_mut(target) {
        Some(p) => {
            for _ in 0..quantity {
                match cards.pop() {
                    Some(c) => p.push(c),
                    None => exit(1),
                }
            }
        }
        None => return false,
    }

    return true;
}

pub fn get_user_card(
    piles: &Vec<Vec<Card>>,
    piles_index: usize,
    pile_index: usize,
) -> Option<Card> {
    let pile = piles.get(piles_index);
    match pile {
        Some(p) => {
            let size = p.len();
            if size < pile_index {
                return None;
            }
            let card = p.get(size - pile_index);
            match card {
                Some(c) => {
                    if c.is_faceup() {
                        return Some(c.clone());
                    }
                    return None;
                }
                None => return None,
            }
        }
        None => exit(1),
    }
}

pub fn valid_moves(piles: &Vec<Vec<Card>>, user_card: &Card, piles_index: usize) -> usize {
    if piles_index < 9 {
        for i in 9..13 {
            if i == piles_index {
                continue;
            }
            let pile = piles.get(i);
            match pile {
                Some(p) => {
                    if valid_move_foundations(user_card, p) {
                        return i;
                    }
                }
                None => exit(1),
            }
        }
    }

    for i in 2..9 {
        if i == piles_index {
            continue;
        }
        let pile = piles.get(i);
        match pile {
            Some(p) => {
                if valid_move_tableau(user_card, p) {
                    return i;
                }
            }
            None => exit(1),
        }
    }

    0
}

fn valid_move_tableau(card: &Card, pile: &Vec<Card>) -> bool {
    let size = pile.len();
    if size == 0 {
        match card.get_card_value() {
            CardValue::K => return true,
            _ => return false,
        }
    }

    let last_card: &Card;
    match pile.get(size - 1) {
        Some(x) => last_card = x,
        None => exit(1),
    }

    if last_card.get_value() == card.get_value() + 1 {
        match (last_card.get_card_color(), card.get_card_color()) {
            (CardColor::Black, CardColor::Black) => return false,
            (CardColor::Red, CardColor::Red) => return false,
            _ => return true,
        }
    }

    false
}

fn valid_move_foundations(card: &Card, pile: &Vec<Card>) -> bool {
    let size = pile.len();
    if size == 0 {
        match card.get_card_value() {
            CardValue::A => return true,
            _ => return false,
        }
    }

    let last_card: &Card;
    match pile.get(size - 1) {
        Some(x) => last_card = x,
        None => exit(1),
    }

    if last_card.get_value() == card.get_value() - 1 {
        match (last_card.get_card_suit(), card.get_card_suit()) {
            (CardType::Corazones, CardType::Corazones) => return true,
            (CardType::Diamantes, CardType::Diamantes) => return true,
            (CardType::Espadas, CardType::Espadas) => return true,
            (CardType::Treboles, CardType::Treboles) => return true,
            _ => return false,
        }
    }

    false
}
