use std::fmt;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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
    pub fn get_card_value(&self) -> CardValue {
        self.number
    }
    pub fn get_card_color(&self) -> CardColor {
        self.color
    }
    pub fn get_card_suit(&self) -> CardType {
        self.suit
    }
    pub fn get_value(&self) -> u8 {
        self.value
    }
    pub fn is_faceup(&self) -> bool {
        self.faceup
    }
    pub fn show(&mut self) {
        self.faceup = true;
    }
    pub fn hide(&mut self) {
        self.faceup = false;
    }
    pub fn log(&self, log_string: &mut String) {
        let log = format!("{}{}{} ", self.number, self.suit, self.color);
        log_string.push_str(&log);
    }
}
