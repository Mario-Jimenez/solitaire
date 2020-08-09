use std::fmt;

#[derive(Copy, Clone)]
pub enum CardValue {
    A,
    N(u8),
    Z,
    J,
    Q,
    K,
}

impl fmt::Display for CardValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CardValue::A => f.write_str("A"),
            CardValue::N(i) => write!(f, "{}", i),
            CardValue::Z => f.write_str("Z"),
            CardValue::J => f.write_str("J"),
            CardValue::Q => f.write_str("Q"),
            CardValue::K => f.write_str("K"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum CardType {
    Corazones,
    Espadas,
    Treboles,
    Diamantes,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CardType::Corazones => f.write_str("♥"),
            CardType::Espadas => f.write_str("♠"),
            CardType::Treboles => f.write_str("♣"),
            CardType::Diamantes => f.write_str("♦"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum CardColor {
    Red,
    Black,
}

impl fmt::Display for CardColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CardColor::Red => f.write_str("r"),
            CardColor::Black => f.write_str("n"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Card {
    suit: CardType,
    number: CardValue,
    value: u8,
    color: CardColor,
    faceup: bool,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.faceup {
            return write!(f, "{}{}{}", self.number, self.suit, self.color);
        }
        write!(f, "XXX")
    }
}

impl Card {
    pub fn new(suit: CardType, number: CardValue) -> Self {
        let value: u8;
        let color: CardColor;
        match number {
            CardValue::A => value = 1,
            CardValue::N(i) => value = i,
            CardValue::Z => value = 10,
            CardValue::J => value = 11,
            CardValue::Q => value = 12,
            CardValue::K => value = 13,
        }
        match suit {
            CardType::Corazones | CardType::Diamantes => color = CardColor::Red,
            _ => color = CardColor::Black,
        }
        Self {
            suit,
            number,
            value,
            color,
            faceup: false,
        }
    }
    pub fn show(&mut self) {
        self.faceup = true;
    }
    pub fn hide(&mut self) {
        self.faceup = false;
    }
}

pub trait Move {
    fn valid_move(&self, card: Card) -> bool;
}

impl Move for Card {
    fn valid_move(&self, card: Card) -> bool {
        match (&self.color, card.color) {
            (CardColor::Black, CardColor::Black) => return false,
            (CardColor::Red, CardColor::Red) => return false,
            _ => (),
        }

        let next_value = self.value + 1;

        if next_value != card.value {
            return false;
        }

        return true;
    }
}
