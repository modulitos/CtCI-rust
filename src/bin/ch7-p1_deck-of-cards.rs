// Deck of Cards: Design the data structures for a generic deck of
// cards. Explain how you would subclass the data structures to
// implement blackjack.

// (Doubling down is not supported)
use rand::prelude::*;
use std::cmp::Ordering;
use std::fmt::Display;
use std::io;

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
    value: Value,
    suit: Suit,
}

impl Card {
    fn new(value: Value, suit: Suit) -> Self {
        Card { suit, value }
    }

    fn get_min_value(&self) -> u8 {
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
            Value::Ace => 1, // min value for Ace
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
}

impl Deck {
    // Returns a deck sorted from highest card to lowest card:
    fn new() -> Self {
        Deck {
            cards: Deck::create_cards(),
        }
    }

    fn create_cards() -> Vec<Card> {
        (0..52)
            .rev()
            .map(|n| Card::new(Value::new(n % 13), Suit::new(n % 4)))
            .collect()
    }

    fn draw_card(&mut self) -> Card {
        if let Some(card) = self.cards.pop() {
            card
        } else {
            // reset the deck:
            self.cards = Deck::create_cards();
            self.cards.pop().unwrap()
        }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

struct Player {
    hand: Vec<Card>,
}

impl Player {
    fn new() -> Self {
        Player { hand: Vec::new() }
    }

    fn hit(&mut self, card: Card) {
        self.hand.push(card);
    }
    // NOTE: if there are Aces, returns the best score possible
    fn get_score(&self) -> u8 {
        let num_aces = self
            .hand
            .iter()
            .filter(|c| c.value == Value::Ace)
            .collect::<Vec<&Card>>()
            .len();
        let mut curr = self
            .hand
            .iter()
            .fold(0, |sum, card| sum + card.get_min_value());
        for _ in 0..num_aces {
            if curr > 11 {
                return curr;
            } else {
                curr += 10;
            }
        }
        curr
    }

    fn print_hand(&self) -> String {
        self.hand
            .iter()
            .map(|card| format!("{}", card))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

fn run_game() {
    let mut player = Player::new();
    let mut dealer = Player::new();
    let mut deck = Deck::new();
    deck.shuffle();
    loop {
        println!("Your cards: {}", player.print_hand());
        if player.get_score() > 21 {
            println!("You busted!");
            break;
        } else if player.get_score() == 21 {
            println!("You won!");
            break;
        }
        println!("enter your next move! (h = hit, s = stay, q = quit)");
        let mut next_move = String::new();
        io::stdin()
            .read_line(&mut next_move)
            .expect("Failed to read line");
        match next_move.trim() {
            "h" => player.hit(deck.draw_card()),
            "s" => {
                println!("alright! Now let's see how the dealer did...");
                while dealer.get_score() < 17 {
                    dealer.hit(deck.draw_card());
                    println!("dealers cards: {}", dealer.print_hand());
                }
                match player.get_score().cmp(&dealer.get_score()) {
                    Ordering::Less => println!("You lost!"),
                    Ordering::Greater => println!("You won!"),
                    Ordering::Equal => println!("Tied!"),
                }
                break;
            }
            "q" => break,
            _ => println!("invalid input! Choose again, or press 'q' to quit"),
        }
    }
}

fn main() {
    println!("welcome to black jack!");
    loop {
        run_game();
        println!("Thanks for playing");
        break;
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
    assert_eq!(deck.draw_card(), Card::new(Value::Two, Suit::Club));
    deck.shuffle();
    println!("{:?}", deck.draw_card());
    println!("{:?}", deck.draw_card());
    println!("{:?}", deck.draw_card());
}

#[test]
fn test_player_score() {
    let mut player = Player::new();
    player.hit(Card::new(Value::Six, Suit::Spade));
    assert_eq!(player.get_score(), 6);
    player.hit(Card::new(Value::King, Suit::Spade));
    assert_eq!(player.get_score(), 16);
}

#[test]
fn test_player_score_aces() {
    let mut player = Player::new();
    player.hit(Card::new(Value::Ace, Suit::Spade));
    assert_eq!(player.get_score(), 11);
    player.hit(Card::new(Value::Ace, Suit::Spade));
    assert_eq!(player.get_score(), 12);
    player.hit(Card::new(Value::Ace, Suit::Spade));
    assert_eq!(player.get_score(), 13);
    player.hit(Card::new(Value::Ace, Suit::Spade));
    assert_eq!(player.get_score(), 14);
    player.hit(Card::new(Value::King, Suit::Spade));
    assert_eq!(player.get_score(), 14);
    player.hit(Card::new(Value::King, Suit::Spade));
    assert_eq!(player.get_score(), 24);

    player = Player::new();
    player.hit(Card::new(Value::Six, Suit::Spade));
    assert_eq!(player.get_score(), 6);
    player.hit(Card::new(Value::Ace, Suit::Spade));
    assert_eq!(player.get_score(), 17);
    player.hit(Card::new(Value::Nine, Suit::Spade));
    assert_eq!(player.get_score(), 16);
    player.hit(Card::new(Value::Five, Suit::Spade));
    assert_eq!(player.get_score(), 21);
}
