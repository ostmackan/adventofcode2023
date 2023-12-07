use std::fs;
use std::str::FromStr;

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

enum HandResult {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
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
    pub cards: Vec<Card>
}

impl Hand{

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

            for card in hand_parts {
                if card.is_none() {
                    println!("Error parsing cards in {value}");
                    continue;
                }
                card.push(card.unwrap());
            }

            return Some(hand);
        }

        return None;
    }
}

pub fn run(path: &str) -> i32{
    let file_contents = fs::read_to_string(path).expect("missing input file");
    let rows = file_contents.split("\n");
    let mut  hands: Vec<Hand> = Vec::new();

    for row in rows {
        if let Some(hand) = Hand::from_str(row) {
            hands.push(hand);
        }
    }

    return 0;
}

#[test]
fn test(){
    assert_eq!(run("data/day7/test.txt"), 6440);
}