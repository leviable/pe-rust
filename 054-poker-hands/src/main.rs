// In the card game poker, a hand consists of five cards and are ranked, from
// lowest to highest, in the following way:
//
// High Card: Highest value card.
// One Pair: Two cards of the same value.
// Two Pairs: Two different pairs.
// Three of a Kind: Three cards of the same value.
// Straight: All cards are consecutive values.
// Flush: All cards of the same suit.
// Full House: Three of a kind and a pair.
// Four of a Kind: Four cards of the same value.
// Straight Flush: All cards are consecutive values of same suit.
// Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
// The cards are valued in the order:
// 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
//
// If two players have the same ranked hands then the rank made up of the
// highest value wins; for example, a pair of eights beats a pair of fives
// (see example 1 below). But if two ranks tie, for example, both players
// have a pair of queens, then highest cards in each hand are compared
// (see example 4 below); if the highest cards tie then the next highest
// cards are compared, and so on.
//
// Consider the following five hands dealt to two players:
//
// Hand	 	Player 1	 	  Player 2	 	     Winner
// 1	 	5H 5C 6S 7S KD    2C 3S 8S 8D TD  	Player 2
//          Pair of Fives     Pair of Eights
//
// 2	 	5D 8C 9S JS AC    2C 5C 7D 8S QH 	Player 1
//          Highest card Ace  Highest card Queen
//
// 3	 	2D 9C AS AH AC	  3D 6D 7D TD QD 	Player 2
//          Three Aces        Flush with Diamonds
//
// 4	 	4D 6S 9H QH QC 	  3D 6D 7H QD QS 	Player 1
//          Pair of queens    Pair of Queens
//          Highest card Nine Highest card Seven
//
// 5	 	2H 2D 4C 4D 4S 	  3C 3D 3S 9S 9D 	Player 1
//          Full House        Full House
//          With Three Fours  with Three Threes
//
// The file, poker.txt, contains one-thousand random hands dealt to two players.
// Each line of the file contains ten cards (separated by a single space): the
// first five are Player 1's cards and the last five are Player 2's cards. You
// can assume that all hands are valid (no invalid characters or repeated cards),
// each player's hand is in no specific order, and in each hand there is a clear
// winner.
//
// How many hands does Player 1 win?

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, Hash, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl Suit {
    fn derive(c: char) -> Self {
        match c {
            'H' => Suit::Heart,
            'D' => Suit::Diamond,
            'C' => Suit::Club,
            'S' => Suit::Spade,
            _ => panic!("{}", format!("Got a suit I know nothing about: {c}")),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum CardValue {
    None = 0,
    Two = 2,
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

impl CardValue {
    fn derive(v: char) -> Self {
        match v {
            '2' => CardValue::Two,
            '3' => CardValue::Three,
            '4' => CardValue::Four,
            '5' => CardValue::Five,
            '6' => CardValue::Six,
            '7' => CardValue::Seven,
            '8' => CardValue::Eight,
            '9' => CardValue::Nine,
            'T' => CardValue::Ten,
            'J' => CardValue::Jack,
            'Q' => CardValue::Queen,
            'K' => CardValue::King,
            'A' => CardValue::Ace,
            _ => panic!("{}", format!("Got a card value I know nothing about: {v}")),
        }
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard{rank: i32, high_card: CardValue},
    OnePair{rank: i32, pair_value: CardValue, high_card: CardValue},
    TwoPairs{rank: i32, pair_1_value: CardValue, pair_2_value: CardValue, high_card: CardValue},
    ThreeOfAKind{rank: i32, trip_value:CardValue, high_card: CardValue},
    Straight{rank: i32, high_card: CardValue},
    Flush{rank: i32, high_card: CardValue},
    FullHouse{rank: i32, trip_value: CardValue, pair_value: CardValue},
    FourOfAKind{rank: i32, quad_value: CardValue, high_card: CardValue},
    StraightFlush{rank: i32, high_card: CardValue},
    RoyalFlush{rank: i32 },
}

impl HandType {
    fn new(cards: Vec<Card>) -> HandType {
        if royal_flush(&cards) {
            Self::RoyalFlush
        } else if straight(&cards) && flush(&cards) {
            Self::StraightFlush
        } else if four_of_a_kind(&cards) {
            Self::FourOfAKind
        } else if full_house(&cards) {
            Self::FullHouse
        } else if flush(&cards) {
            Self::Flush
        } else if straight(&cards) {
            Self::Straight
        } else if three_of_a_kind(&cards) {
            Self::ThreeOfAKind
        } else if two_pairs(&cards) {
            Self::TwoPairs
        } else if let Some(one_pair) = one_pair(&cards) {
            Self::OnePair{rank: one_pair.0, pair_value.1, high_card.2}
        } else {
            Self::HighCard
        }
    }
}

// Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
fn royal_flush(cards: &[Card]) -> bool {
    straight(cards) && flush(cards) && cards.iter().any(|x| x.value == CardValue::Ace)
}

#[test]
fn test_royal_flush() {
    assert_eq!(
        royal_flush(&Hand::new(&vec!["TD", "JD", "QD", "KD", "AD"]).cards),
        true
    );
    assert_eq!(
        royal_flush(&Hand::new(&vec!["9D", "JD", "QD", "KD", "AD"]).cards),
        false
    );
    assert_eq!(
        royal_flush(&Hand::new(&vec!["TC", "JD", "QD", "KD", "AD"]).cards),
        false
    );
}

// Straight: All cards are consecutive values.
fn straight(cards: &[Card]) -> bool {
    let mut state = cards[0].value as i32;
    for card in &cards[1..] {
        if card.value as i32 != state + 1 {
            return false;
        }
        state = card.value as i32;
    }

    true
}

#[test]
fn test_straight() {
    assert_eq!(
        straight(&Hand::new(&vec!["TC", "JD", "QD", "KD", "AD"]).cards),
        true
    );
    assert_eq!(
        straight(&Hand::new(&vec!["9D", "JD", "QD", "KD", "AD"]).cards),
        false
    );
}

// Flush: All cards of the same suit.
fn flush(cards: &[Card]) -> Option<(i32, CardValue)>{
    let initial_state = &cards[0].suit;
    let is_flush = cards.iter().all(|x| x.suit == *initial_state);
    let high_card = CardValue::None;
    for card in cards.iter() {
        if card.value > high_card {
            high_card = card.value;
        }
    }

    if high_card == CardValue::None {
        panic!("Expected this to not be none: {:?}", high_card);
    } else if is_flush {
        Some((8, high_card))
    } else {
        None
    }

}

#[test]
fn test_flush() {
    assert_eq!(
        flush(&Hand::new(&vec!["2D", "4D", "QD", "8D", "AD"]).cards),
        Some((8, CardValue::Ace))
    );
    assert_eq!(
        flush(&Hand::new(&vec!["TC", "JD", "QD", "KD", "AD"]).cards),
        None
    );
}

// Four of a Kind: Four cards of the same value.
fn four_of_a_kind(cards: &Vec<Card>) -> Option<(i32, CardValue, CardValue)> {
    let mut hm: HashMap<CardValue, i32> = HashMap::new();
    for card in cards {
        let count = hm.entry(card.value).or_insert(0);
        *count += 1
    }

    let quad_value = CardValue::None;
    let high_card = CardValue::None;
    for (k, v) in hm {
        if v == 4 {
            quad_value = k;
        } else if k > high_card {
            high_card = k;
        }
    }

    if quad_value == CardValue::None  || high_card == CardValue::None{
        panic!("Expected these to not be none: {:?} {:?}", quad_value, high_card);
    } else if quad_value != CardValue::None {
        Some((7, quad_value, high_card))
    } else {
        None
    }
}

#[test]
fn test_four_of_a_kind() {
    assert_eq!(
        four_of_a_kind(&Hand::new(&vec!["2D", "2C", "2H", "2H", "AD"]).cards),
        Some((7, CardValue::Three, CardValue::Two))
    );
    assert_eq!(
        four_of_a_kind(&Hand::new(&vec!["2C", "2D", "2C", "KD", "AD"]).cards),
        None
    );
}

// Full House: Three of a kind and a pair.
fn full_house(cards: &Vec<Card>) -> Option<(i32, CardValue, CardValue)> {
    let mut hm: HashMap<CardValue, i32> = HashMap::new();
    for card in cards {
        let count = hm.entry(card.value).or_insert(0);
        *count += 1
    }
    let mut found_three = false;
    let mut found_two = false;
    let mut trip_value = CardValue::None;
    let mut pair_value = CardValue::None;
    for (k, v) in hm {
        match v {
            2 => pair_value = k,
            3 => trip_value = k,
            _ => (),
        }
    }

    if trip_value == CardValue::None  || pair_value == CardValue::None{
        panic!("Expected these to not be none: {:?} {:?}", trip_value, pair_value);
    } else if trip_value != CardValue::None && pair_value != CardValue::None {
        Some((6, trip_value, pair_value))
    } else {
        None
    }
}

#[test]
fn test_full_house() {
    assert_eq!(
        full_house(&Hand::new(&vec!["2D", "2C", "3D", "3C", "3H"]).cards),
        Some((6, CardValue::Three, CardValue::Two))
    );
    assert_eq!(
        full_house(&Hand::new(&vec!["2D", "4C", "3D", "3C", "3H"]).cards),
        None
    );
    assert_eq!(
        full_house(&Hand::new(&vec!["2D", "2C", "4D", "3C", "3H"]).cards),
        None
    );
}

// Three of a Kind: Three cards of the same value.
fn three_of_a_kind(cards: &Vec<Card>) -> Option<(i32, CardValue, CardValue)> {
    let mut hm: HashMap<CardValue, i32> = HashMap::new();
    for card in cards {
        let count = hm.entry(card.value).or_insert(0);
        *count += 1
    }
    let mut found_three = false;
    let mut high_card = CardValue::None;
    let mut trip_value = CardValue::None;
    for (k, v) in hm {
        if v == 3 {
            found_three = true;
            trip_value = k;
        } else if k > high_card {
            high_card = k;
        }
    }

    if high_card == CardValue::None || trip_value == CardValue::None {
        panic!("Expected these to not be none: {:?} {:?}", trip_value, high_card);
    } else if found_three {
        Some((5, trip_value, high_card))
    } else {
        None
    }
}

#[test]
fn test_three_of_a_kind() {
    assert_eq!(
        three_of_a_kind(&Hand::new(&vec!["2D", "3C", "2H", "8D", "2C"]).cards),
        Some((5, CardValue::Two, CardValue::Eight))
    );
    assert_eq!(
        three_of_a_kind(&Hand::new(&vec!["2D", "4C", "3D", "3C", "9H"]).cards),
        None
    );
}

// Two Pairs: Two different pairs.
fn two_pairs(cards: &Vec<Card>) -> Option<(i32, CardValue, CardValue, CardValue)> {
    let mut hm: HashMap<CardValue, i32> = HashMap::new();
    for card in cards {
        let count = hm.entry(card.value).or_insert(0);
        *count += 1
    }
    let mut found_two = 0;
    let mut high_card = CardValue::None;
    let mut pair_1_value = CardValue::None;
    let mut pair_2_value = CardValue::None;
    for (k, v) in hm {
        if v == 2 {
            found_two += 1;
            if pair_1_value == CardValue::None {
                pair_1_value = k;
            } else {
                pair_2_value = k;
            }
        } else {
            high_card = k;

        }
    }

    if high_card == CardValue::None || pair_1_value == CardValue::None || pair_2_value == CardValue::None {
        panic!("Expected these to not be none: {:?} {:?} {:?}", pair_1_value, pair_2_value, high_card);

    }
    else if found_two == 2 {
        Some((3, pair_1_value, pair_2_value, high_card))
    } else {
        None
    }
}

#[test]
fn test_two_pair() {
    assert_eq!(
        two_pairs(&Hand::new(&vec!["2D", "2C", "3H", "3D", "8C"]).cards),
        Some((3, CardValue::Three, CardValue::Two, CardValue::Eight))
    );
    assert_eq!(
        two_pairs(&Hand::new(&vec!["2D", "2C", "3H", "3D", "3C"]).cards),
        None
    );
}

// One Pair: Two cards of the same value.
fn one_pair(cards: &Vec<Card>) -> Option<(i32, CardValue, CardValue)> {
    let mut hm: HashMap<CardValue, i32> = HashMap::new();
    for card in cards {
        let count = hm.entry(card.value).or_insert(0);
        *count += 1
    }
    let mut found_one = 0;
    let mut one_pair_value: CardValue = CardValue::None;
    let mut high_card: CardValue = CardValue::None;
    for (k, v) in hm {
        if v == 2 {
            found_one += 1;
            one_pair_value = k;
        }
    }

    if one_pair_value == CardValue::None || high_card == CardValue::None {
        panic!("Expected these to not be none: {:?} {:?}", one_pair_value, high_card);
    }
    else if found_one == 1 {
        Some((2, one_pair_value, high_card))
    } else {
        None
    }
}

#[test]
fn test_one_pair() {
    assert_eq!(
        one_pair(&Hand::new(&vec!["2D", "2C", "4H", "3D", "8C"]).cards),
        Some((2, CardValue::Two, CardValue::Eight))
    );
    assert_eq!(
        one_pair(&Hand::new(&vec!["2D", "2C", "2H", "3C", "9H"]).cards),
        None
    );
}

#[derive(Debug, Hash, Clone, Ord, Eq, PartialEq, PartialOrd)]
struct Card {
    suit: Suit,
    value: CardValue,
}

impl Card {
    fn new(card: &str) -> Self {
        Self {
            value: CardValue::derive(card.chars().collect::<Vec<char>>()[0]),
            suit: Suit::derive(card.chars().collect::<Vec<char>>()[1]),
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    high_value: Card,
    high_card: Card,
}

impl Hand {
    pub fn new(raw_cards: &[&str]) -> Self {
        let mut cards = raw_cards
            .iter()
            .map(|x| Card::new(x))
            .collect::<Vec<Card>>();
        cards.sort_by(|c1, c2| (c1).value.cmp(&c2.value));
        let (hand_type, high_value, high_card) = HandType::new(cards)
        Self {
            cards: cards.clone(),
            hand_type: hand_type,
            high_value: high_value,
            high_card: high_card,
        }
    }
}

fn compare(p1: Hand, p2: Hand) -> Option<Winner> {
    println!("0000000000000000000000000000000000000000");
    println!("Compairing {:?} to {:?}", p1.hand_type, p2.hand_type);

    None
}

#[derive(PartialEq, Eq)]
enum Winner {
    P1,
    P2,
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn pe054() -> u64 {
    let mut lines: Vec<String> = vec![];
    for line in read_lines("./src/pe054-poker.txt".to_string()) {
        lines.push(line.unwrap());
    }
    let mut p1_wins = 0;
    let mut _p2_wins = 0;
    for (idx, line) in lines.iter().enumerate() {
        let raw_cards = line.split(' ').collect::<Vec<&str>>();
        let (p1, p2) = (Hand::new(&raw_cards[..5]), Hand::new(&raw_cards[5..]));
        println!("11111111111111111111111111111111111111");
        dbg!(&p1);
        dbg!(&p2);
        match compare(p1, p2) {
            Some(Winner::P1) => p1_wins += 1,
            Some(Winner::P2) => _p2_wins += 1,
            None => unreachable!("Couldn't deduce winner"),
        }

        if idx == 5 {
            break;
        }
    }
    p1_wins
}

fn main() {
    let start = Instant::now();
    let solution = pe054();
    println!("Solution: {solution}");
    println!("Time Elapsed: {}", start.elapsed().as_secs_f64());
}
