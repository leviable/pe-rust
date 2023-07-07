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
    HighCard {
        rank: i32,
        high_card: CardValue,
    },
    OnePair {
        rank: i32,
        pair_value: CardValue,
        high_card: CardValue,
    },
    TwoPairs {
        rank: i32,
        pair_1_value: CardValue,
        pair_2_value: CardValue,
        high_card: CardValue,
    },
    ThreeOfAKind {
        rank: i32,
        trip_value: CardValue,
        high_card: CardValue,
    },
    Straight {
        rank: i32,
        high_card: CardValue,
    },
    Flush {
        rank: i32,
        high_card: CardValue,
    },
    FullHouse {
        rank: i32,
        trip_value: CardValue,
        pair_value: CardValue,
    },
    FourOfAKind {
        rank: i32,
        quad_value: CardValue,
        high_card: CardValue,
    },
    StraightFlush {
        rank: i32,
        high_card: CardValue,
    },
    RoyalFlush {
        rank: i32,
    },
}

impl HandType {
    fn new(cards: Vec<Card>) -> HandType {
        if let Some(hand) = royal_flush(&cards) {
            Self::RoyalFlush { rank: hand.0 }
        } else if let Some(hand) = straight_flush(&cards) {
            Self::StraightFlush {
                rank: hand.0,
                high_card: hand.1,
            }
        } else if let Some(hand) = four_of_a_kind(&cards) {
            Self::FourOfAKind {
                rank: hand.0,
                quad_value: hand.1,
                high_card: hand.2,
            }
        } else if let Some(hand) = full_house(&cards) {
            Self::FullHouse {
                rank: hand.0,
                trip_value: hand.1,
                pair_value: hand.2,
            }
        } else if let Some(hand) = flush(&cards) {
            Self::Flush {
                rank: hand.0,
                high_card: hand.1,
            }
        } else if let Some(hand) = straight(&cards) {
            Self::Straight {
                rank: hand.0,
                high_card: hand.1,
            }
        } else if let Some(hand) = three_of_a_kind(&cards) {
            Self::ThreeOfAKind {
                rank: hand.0,
                trip_value: hand.1,
                high_card: hand.2,
            }
        } else if let Some(hand) = two_pairs(&cards) {
            Self::TwoPairs {
                rank: hand.0,
                pair_1_value: hand.1,
                pair_2_value: hand.2,
                high_card: hand.3,
            }
        } else if let Some(hand) = one_pair(&cards) {
            Self::OnePair {
                rank: hand.0,
                pair_value: hand.1,
                high_card: hand.2,
            }
        } else if let Some(hand) = high_card(&cards) {
            Self::HighCard {
                rank: hand.0,
                high_card: hand.1,
            }
        } else {
            panic!("Failed to destructure cards into hand");
        }
    }
}

// Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
fn royal_flush(cards: &[Card]) -> Option<(i32,)> {
    // straight(cards) && flush(cards) && cards.iter().any(|x| x.value == CardValue::Ace)
    // Is a straight, is a flush, and high card is an ace
    match straight(cards) {
        None => None,
        Some(_) => match flush(cards) {
            None => None,
            Some(_) => {
                if cards.iter().any(|x| x.value == CardValue::Ace) {
                    Some((10,))
                } else {
                    None
                }
            }
        },
    }
}

#[test]
fn test_royal_flush() {
    assert_eq!(
        royal_flush(&Hand::new(&vec!["TD", "JD", "QD", "KD", "AD"]).cards),
        Some((10,))
    );
    assert_eq!(
        royal_flush(&Hand::new(&vec!["9D", "JD", "QD", "KD", "AD"]).cards),
        None
    );
    assert_eq!(
        royal_flush(&Hand::new(&vec!["TC", "JD", "QD", "KD", "AD"]).cards),
        None
    );
}

// Straight Flush: All cards are consecutive values of same suit.
fn straight_flush(cards: &[Card]) -> Option<(i32, CardValue)> {
    let mut high_card = CardValue::None;
    match straight(cards) {
        None => None,
        Some(_) => match flush(cards) {
            None => None,
            Some(_) => {
                for card in cards {
                    if card.value == CardValue::Ace {
                        return None;
                    } else if card.value > high_card {
                        high_card = card.value;
                    }
                }
                Some((10, high_card))
            }
        },
    }
}

#[test]
fn test_straight_flush() {
    assert_eq!(
        straight_flush(&Hand::new(&vec!["9D", "TD", "JD", "QD", "KD"]).cards),
        Some((10, CardValue::King))
    );
    assert_eq!(
        straight_flush(&Hand::new(&vec!["9D", "JD", "QD", "KD", "AD"]).cards),
        None
    );
    assert_eq!(
        straight_flush(&Hand::new(&vec!["TC", "JD", "QD", "KD", "AD"]).cards),
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

    let mut quad_value = CardValue::None;
    let mut high_card = CardValue::None;
    for (k, v) in hm {
        if v == 4 {
            quad_value = k;
        } else if k > high_card {
            high_card = k;
        }
    }

    if quad_value == CardValue::None || high_card == CardValue::None {
        None
    } else if quad_value != CardValue::None {
        Some((8, quad_value, high_card))
    } else {
        None
    }
}

#[test]
fn test_four_of_a_kind() {
    assert_eq!(
        four_of_a_kind(&Hand::new(&vec!["2D", "2C", "2H", "2H", "AD"]).cards),
        Some((8, CardValue::Two, CardValue::Ace))
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
    let mut trip_value = CardValue::None;
    let mut pair_value = CardValue::None;
    for (k, v) in hm {
        match v {
            2 => pair_value = k,
            3 => trip_value = k,
            _ => (),
        }
    }

    if trip_value == CardValue::None || pair_value == CardValue::None {
        None
    } else if trip_value != CardValue::None && pair_value != CardValue::None {
        Some((7, trip_value, pair_value))
    } else {
        None
    }
}

#[test]
fn test_full_house() {
    assert_eq!(
        full_house(&Hand::new(&vec!["2D", "2C", "3D", "3C", "3H"]).cards),
        Some((7, CardValue::Three, CardValue::Two))
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

// Flush: All cards of the same suit.
fn flush(cards: &[Card]) -> Option<(i32, CardValue)> {
    let initial_state = &cards[0].suit;
    let is_flush = cards.iter().all(|x| x.suit == *initial_state);
    let mut high_card = CardValue::None;
    for card in cards.iter() {
        if card.value > high_card {
            high_card = card.value;
        }
    }

    if high_card == CardValue::None {
        panic!("Expected this to not be none: {:?}", high_card);
    } else if is_flush {
        Some((6, high_card))
    } else {
        None
    }
}

#[test]
fn test_flush() {
    assert_eq!(
        flush(&Hand::new(&vec!["2D", "4D", "QD", "8D", "AD"]).cards),
        Some((6, CardValue::Ace))
    );
    assert_eq!(
        flush(&Hand::new(&vec!["TC", "JD", "QD", "KD", "AD"]).cards),
        None
    );
}

// Straight: All cards are consecutive values.
fn straight(cards: &[Card]) -> Option<(i32, CardValue)> {
    let mut state = cards[0].value;
    let mut is_straight = true;
    for card in &cards[1..] {
        if card.value as i32 != state as i32 + 1 {
            is_straight = false;
            break;
        }
        state = card.value;
    }

    if !is_straight {
        None
    } else {
        Some((5, state))
    }
}

#[test]
fn test_straight() {
    assert_eq!(
        straight(&Hand::new(&vec!["TC", "JD", "QD", "KD", "AD"]).cards),
        Some((5, CardValue::Ace))
    );
    assert_eq!(
        straight(&Hand::new(&vec!["9D", "JD", "QD", "KD", "AD"]).cards),
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
        None
    } else if found_three {
        Some((4, trip_value, high_card))
    } else {
        None
    }
}

#[test]
fn test_three_of_a_kind() {
    assert_eq!(
        three_of_a_kind(&Hand::new(&vec!["2D", "3C", "2H", "8D", "2C"]).cards),
        Some((4, CardValue::Two, CardValue::Eight))
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

    if high_card == CardValue::None
        || pair_1_value == CardValue::None
        || pair_2_value == CardValue::None
    {
        None
    } else if found_two == 2 {
        if pair_1_value > pair_2_value {
            Some((3, pair_1_value, pair_2_value, high_card))
        } else {
            Some((3, pair_2_value, pair_1_value, high_card))
        }
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
        } else if k > high_card {
            high_card = k;
        }
    }

    if one_pair_value == CardValue::None || high_card == CardValue::None {
        None
    } else if found_one == 1 {
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

// One Pair: Two cards of the same value.
fn high_card(cards: &Vec<Card>) -> Option<(i32, CardValue)> {
    let mut high_card: CardValue = CardValue::None;
    for card in cards {
        if card.value > high_card {
            high_card = card.value;
        }
    }
    if high_card == CardValue::None {
        panic!("Expected these to not be none: {:?} ", high_card);
    } else {
        Some((1, high_card))
    }
}

#[test]
fn test_high_card() {
    assert_eq!(
        high_card(&Hand::new(&vec!["2D", "3C", "5H", "6D", "8C"]).cards),
        Some((1, CardValue::Eight))
    );
}

#[derive(Debug, Hash, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub struct Card {
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
    pub cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    pub fn new(raw_cards: &[&str]) -> Self {
        let mut cards = raw_cards
            .iter()
            .map(|x| Card::new(x))
            .collect::<Vec<Card>>();
        cards.sort_by(|c1, c2| (c1).value.cmp(&c2.value));
        let hand_type = HandType::new(cards.clone());

        Self { cards, hand_type }
    }
}

fn compare(p1: Hand, p2: Hand) -> Option<Winner> {
    match p1.hand_type.cmp(&p2.hand_type) {
        Ordering::Greater => Some(Winner::P1),
        Ordering::Less => Some(Winner::P2),
        Ordering::Equal => None,
    }
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
    for line in lines.iter() {
        let raw_cards = line.split(' ').collect::<Vec<&str>>();
        let (p1, p2) = (Hand::new(&raw_cards[..5]), Hand::new(&raw_cards[5..]));
        match compare(p1, p2) {
            Some(Winner::P1) => p1_wins += 1,
            Some(Winner::P2) => _p2_wins += 1,
            None => unreachable!("Couldn't deduce winner"),
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
