// Deck of Cards: Design the data structures for a generic deck of
// cards. Explain how you would subclass the data structures to
// implement blackjack.

use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Debug, Eq, PartialEq)]
enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Suit {
    fn short_str(&self) -> &str {
        match *self {
            Suit::Spade => "s",
            Suit::Heart => "h",
            Suit::Diamond => "d",
            Suit::Club => "c",
        }
    }

    fn new(n: u8) -> Self {
        match n {
            0 => Suit::Club,
            1 => Suit::Heart,
            2 => Suit::Spade,
            3 => Suit::Diamond,
            _ => panic!("Suit.new(): invalid value: {}", n),
        }
    }
}
impl Value {
    fn short_str(&self) -> &str {
        match *self {
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "T",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
            Value::Ace => "A",
        }
    }

    fn new(n: u8) -> Self {
        match n {
            0 => Value::Two,
            1 => Value::Three,
            2 => Value::Four,
            3 => Value::Five,
            4 => Value::Six,
            5 => Value::Seven,
            6 => Value::Eight,
            7 => Value::Nine,
            8 => Value::Ten,
            9 => Value::Jack,
            10 => Value::Queen,
            11 => Value::King,
            12 => Value::Ace,
            _ => panic!("Value.new(): invalid value: {}", n),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    fn new(value: Value, suit: Suit) -> Self {
        Card { suit, value }
    }
    fn get_value(&self) -> u8 {
        match self.value {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
            Value::Ace => 11,
        }
    }
}

impl Display for Card {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "{}{}", self.value.short_str(), self.suit.short_str())
    }
}

struct Deck {
    cards: Vec<Card>,
    // draw_card: fn(&mut self)
}

impl Deck {
    // Returns a deck sorted from highest card to lowest card:
    fn new() -> Self {
        Deck {
            cards: (0..52)
                .rev()
                .map(|n| Card::new(Value::new(n % 13), Suit::new(n % 4)))
                .collect(),
        }
    }

    fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[test]
fn test_card() {
    assert_eq!(format!("{}", Card::new(Value::Ace, Suit::Spade)), "As");
    assert_eq!(format!("{}", Card::new(Value::Queen, Suit::Heart)), "Qh");
}

#[test]
fn test_deck() {
    let mut deck = Deck::new();
    assert_eq!(deck.draw_card(), Some(Card::new(Value::Two, Suit::Club)));
}
