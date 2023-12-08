use std::arch::is_aarch64_feature_detected;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::hash_map::OccupiedEntry;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;
use crate::day7::part1::HandResult::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl Card{

    fn from_char(value: char) -> Option<Card>{
        match value {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            _ => None
        }
    }
    fn to_value(&self) ->i32{
        match self {
            Card::A => {14}
            Card::K => {13}
            Card::Q => {12}
            Card::J => {11}
            Card::T => {10}
            Card::Nine => {9}
            Card::Eight => {8}
            Card::Seven => {7}
            Card::Six => {6}
            Card::Five => {5}
            Card::Four => {4}
            Card::Three => {3}
            Card::Two => {2}
        }
    }
}

impl Eq for Card {}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        i32::partial_cmp(&self.to_value(), &other.to_value())
    }
}

impl Ord for Card{
    fn cmp(&self, other: &Self) -> Ordering {
        i32::cmp(&self.to_value(), &other.to_value())
    }
}

enum HandResult {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl Display for HandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       let value : &str = match self {
            FiveOfAKind => "Five of a kind",
            FourOfAKind => "Four of a kind",
            FullHouse => "Full house",
            ThreeOfAKind => "Three of a kind",
            TwoPair => "Two pairs",
            OnePair => "One pair",
            HighCard => "High card"
        };

        write!(f, "{}", value)
    }
}

impl HandResult{
    fn to_rank(&self) -> i32{
        match self {
            HandResult::FiveOfAKind => {7}
            HandResult::FourOfAKind => {6}
            HandResult::FullHouse => {5}
            HandResult::ThreeOfAKind => {4}
            HandResult::TwoPair => {3}
            HandResult::OnePair => {2}
            HandResult::HighCard => {1}
        }
    }
}

struct Hand{
    pub bid:i32,
    pub result: HandResult,
    pub cards: Vec<i32>
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.result.to_rank() == other.result.to_rank() && self.compare_hand(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let my_value = self.result.to_rank();
        let other_value = other.result.to_rank();

        if my_value == other_value{
            return self.compare_hand(other);
        }

        i32::cmp(&my_value, &other_value)
    }
}

impl Hand{

    pub fn compare_hand(&self, other: &Self) -> Ordering{

        let mut other_iter = other.cards.iter().enumerate();

        for (i, card) in self.cards.iter().enumerate() {
            if let Some((other_i, other_card)) = other_iter.next() {
                let cmp = i32::cmp(card, other_card);
                if cmp == Ordering::Equal {
                    continue;
                }
                return cmp;
            }

        }

        println!("Major issue can't compare hands");
        Ordering::Equal
    }

    pub fn parse_hand(&self){
        let iter = self.cards.iter();

    }
    pub fn from_str(value: &str) -> Option<Hand>{
        let mut hand = Hand{
            bid: 0,
            result: HandResult::HighCard,
            cards: Vec::new()
        };

        let mut row_parts = value.split_whitespace().into_iter();

        let hand_part = row_parts.next();
        let bid_part = row_parts.next();

        if hand_part.is_some() && bid_part.is_some() {
            let bid_value = i32::from_str(bid_part.unwrap());

            if bid_value.is_err() {
                return None;
            }

            hand.bid = bid_value.unwrap();

            let hand_parts = hand_part.unwrap().chars().map(|x| Card::from_char(x));

            let mut test_cards : BTreeMap<Card, i32> = BTreeMap::new();

            for card in hand_parts {
                if card.is_none() {
                    println!("Error parsing cards in {value}");
                    continue;
                }

                let parsed_card = card.unwrap();


                hand.cards.push(parsed_card.to_value());

                //*hand.cards.entry(parsed_card).or_insert(0) += 1;
                *test_cards.entry(parsed_card).or_insert(0) += 1;


            }

            let mut singles = 0;
            let mut pairs = 0;
            let mut triples = 0;
            let mut quads = 0;
            let mut fives = 0;


            for (card, count) in test_cards {
                println!("card: {} - {}", card.to_value(), count);

                match count {
                    1 => singles+=1,
                    2 => pairs+=1,
                    3 => triples+=1,
                    4 => quads+=1,
                    5 => fives+=1,
                    _ => println!("error more then five cards")
                }
            }

            let result: HandResult;

            if singles == 5 {
                result = HighCard
            } else if fives == 1 {
                result = FiveOfAKind
            } else if quads == 1 {
                result = FourOfAKind
            } else if triples == 1 && pairs == 1 {
                result = FullHouse
            } else if triples == 1 {
                result = ThreeOfAKind
            } else if pairs == 2 {
                result = TwoPair
            } else if pairs == 1 {
                result = OnePair
            } else {
                println!("failed to get a hand {value}");
                return None;
            }

            hand.result = result;

            return Some(hand);
        }

        return None;
    }
}

pub fn run(path: &str) -> i32{
    let file_contents = fs::read_to_string(path).expect("missing input file");
    let rows = file_contents.split("\n");
    let mut  hands: Vec<Hand> = Vec::new();
    let mut number_of_hands = 0;
    let mut result = 0;

    for row in rows {
        if let Some(hand) = Hand::from_str(row) {
            println!("{}", hand.result);

            hands.push(hand);
            number_of_hands+=1;
        }
    }

    hands.sort();

    for (i,hand) in hands.iter().enumerate() {
        let score = ((i as i32)+1) * hand.bid;
        result += score;
        println!("{} {} {score} -> {result}", hand.result, hand.bid )
    }

    return result;
}

#[test]
fn test(){
    assert_eq!(run("data/day7/test.txt"), 6440);
}