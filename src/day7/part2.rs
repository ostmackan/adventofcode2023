use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;
use crate::day7::part2::CardWithJoker::Joker;
use crate::day7::part2::HandResultWithJokers::{FiveOfAKind, HighCard};

enum CardWithJoker {
    A,
    K,
    Q,
    Joker,
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

impl CardWithJoker{

    fn from_char(value: char) -> Option<CardWithJoker>{
        match value {
            'A' => Some(CardWithJoker::A),
            'K' => Some(CardWithJoker::K),
            'Q' => Some(CardWithJoker::Q),
            'J' => Some(CardWithJoker::Joker),
            'T' => Some(CardWithJoker::T),
            '9' => Some(CardWithJoker::Nine),
            '8' => Some(CardWithJoker::Eight),
            '7' => Some(CardWithJoker::Seven),
            '6' => Some(CardWithJoker::Six),
            '5' => Some(CardWithJoker::Five),
            '4' => Some(CardWithJoker::Four),
            '3' => Some(CardWithJoker::Three),
            '2' => Some(CardWithJoker::Two),
            _ => None
        }
    }
    fn to_value(&self) ->i32{
        match self {
            CardWithJoker::A => {14}
            CardWithJoker::K => {13}
            CardWithJoker::Q => {12}
            CardWithJoker::T => {10}
            CardWithJoker::Nine => {9}
            CardWithJoker::Eight => {8}
            CardWithJoker::Seven => {7}
            CardWithJoker::Six => {6}
            CardWithJoker::Five => {5}
            CardWithJoker::Four => {4}
            CardWithJoker::Three => {3}
            CardWithJoker::Two => {2}
            Joker => {0}
        }
    }
}

impl Eq for CardWithJoker {}

impl PartialEq<Self> for CardWithJoker {
    fn eq(&self, other: &Self) -> bool {
        self.to_value() == other.to_value()
    }
}

impl PartialOrd<Self> for CardWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        i32::partial_cmp(&self.to_value(), &other.to_value())
    }
}

impl Ord for CardWithJoker{
    fn cmp(&self, other: &Self) -> Ordering {
        i32::cmp(&self.to_value(), &other.to_value())
    }
}

enum HandResultWithJokers {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl Display for HandResultWithJokers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value : &str = match self {
            HandResultWithJokers::FiveOfAKind => "Five of a kind",
            HandResultWithJokers::FourOfAKind => "Four of a kind",
            HandResultWithJokers::FullHouse => "Full house",
            HandResultWithJokers::ThreeOfAKind => "Three of a kind",
            HandResultWithJokers::TwoPair => "Two pairs",
            HandResultWithJokers::OnePair => "One pair",
            HandResultWithJokers::HighCard => "High card"
        };

        write!(f, "{}", value)
    }
}

impl HandResultWithJokers{
    fn to_rank(&self) -> i32{
        match self {
            HandResultWithJokers::FiveOfAKind => {7}
            HandResultWithJokers::FourOfAKind => {6}
            HandResultWithJokers::FullHouse => {5}
            HandResultWithJokers::ThreeOfAKind => {4}
            HandResultWithJokers::TwoPair => {3}
            HandResultWithJokers::OnePair => {2}
            HandResultWithJokers::HighCard => {1}
        }
    }
}

struct HandWithJokers{
    pub bid:i32,
    pub result: HandResultWithJokers,
    pub cards: Vec<i32>,
    pub raw: String
}

impl Eq for HandWithJokers {}

impl PartialEq<Self> for HandWithJokers {
    fn eq(&self, other: &Self) -> bool {
        self.result.to_rank() == other.result.to_rank() && self.compare_hand(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for HandWithJokers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandWithJokers {
    fn cmp(&self, other: &Self) -> Ordering {
        let my_value = self.result.to_rank();
        let other_value = other.result.to_rank();

        if my_value == other_value{
            return self.compare_hand(other);
        }

        i32::cmp(&my_value, &other_value)
    }
}

impl HandWithJokers{

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
    pub fn from_str(value: &str) -> Option<HandWithJokers>{
        let mut hand = HandWithJokers{
            bid: 0,
            result: HandResultWithJokers::HighCard,
            cards: Vec::new(),
            raw: String::from("")
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
            hand.raw = hand_part.unwrap().to_string();

            let hand_parts = hand_part.unwrap().chars().map(|x| CardWithJoker::from_char(x));

            let mut test_cards : BTreeMap<CardWithJoker, i32> = BTreeMap::new();
            let mut jokers = 0;

            for card in hand_parts {
                if card.is_none() {
                    println!("Error parsing cards in {value}");
                    continue;
                }

                let parsed_card = card.unwrap();

                if parsed_card == Joker {
                    jokers+=1;
                }


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
                print!("card: {} - {} \t", card.to_value(), count);

                if card == CardWithJoker::Joker{
                    continue;
                }

                match count {
                    1 => singles+=1,
                    2 => pairs+=1,
                    3 => triples+=1,
                    4 => quads+=1,
                    5 => fives+=1,
                    _ => println!("error more then five cards")
                }
            }

            let result: HandResultWithJokers;

            if jokers > 0 {
                if jokers == 1 {
                    if quads == 1 {
                        result = FiveOfAKind;
                    } else if triples == 1 {
                        result = HandResultWithJokers::FourOfAKind;
                    }
                    else if pairs == 2 {
                         result = HandResultWithJokers::FullHouse;
                    }
                    else if pairs == 1 {
                        result = HandResultWithJokers::ThreeOfAKind;
                    }
                    else if singles == 4 {
                        result = HandResultWithJokers::OnePair;
                    }
                    else {
                        println!("Can we get here? 1 joker");
                        return None;
                    }
                } else if jokers == 2 {
                    if triples == 1 {
                        result = HandResultWithJokers::FiveOfAKind
                    } else if pairs == 1 {
                        result = HandResultWithJokers::FourOfAKind
                    } else if singles == 3 {
                        result = HandResultWithJokers::ThreeOfAKind
                    }
                    else {
                        println!("Can we get here? 2 jokers");
                        return None;
                    }
                } else if jokers == 3 {
                    if pairs == 1{
                        result = HandResultWithJokers::FiveOfAKind
                    } else if singles == 2 {
                        result = HandResultWithJokers::FourOfAKind
                    } else {
                        println!("Can we get here? 3 jokers");
                        return None;
                    }

                } else if jokers == 4 {
                    result = FiveOfAKind
                }
                else {
                    result = FiveOfAKind;
                }

            }
            else if fives == 1 {
                result = HandResultWithJokers::FiveOfAKind
            } else if quads == 1 {
                result = HandResultWithJokers::FourOfAKind;}
                else if triples == 1 && pairs == 1 {
                    result = HandResultWithJokers::FullHouse
            } else if triples == 1 {
                result = HandResultWithJokers::ThreeOfAKind
            } else if pairs == 2 {
                result = HandResultWithJokers::TwoPair
            } else if pairs == 1 {
                result = HandResultWithJokers::OnePair
            } else if singles == 5 {
                result = HandResultWithJokers::HighCard
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

pub fn run(path: &str) -> i64{
    let file_contents = fs::read_to_string(path).expect("missing input file");
    let rows = file_contents.split("\n");
    let mut hands: Vec<HandWithJokers> = Vec::new();
    let mut number_of_hands = 0;
    let mut result:i64 = 0;

    for row in rows {
        if let Some(hand) = HandWithJokers::from_str(row) {
            println!("{}", hand.result);

            hands.push(hand);
            number_of_hands+=1;
        }
    }

    hands.sort();

    for (i,hand) in hands.iter().enumerate() {
        let score = ((i as i64)+1) * hand.bid as i64;
        result += score;
        println!("{i}: {} {} {} {score} -> {result}", hand.result, hand.raw, hand.bid )
    }

    return result;
}

#[test]
fn test(){
    assert_eq!(run("data/day7/test.txt"), 5905);
}

#[test]
fn test_full(){
    assert_eq!(run("data/day7/input.txt"), 250057090);
}