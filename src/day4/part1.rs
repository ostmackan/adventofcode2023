use std::fs;
use std::str::FromStr;

struct ScratchCard{
    pub id: i32,
    pub winning_numbers: Vec<i32>,
    pub card_numbers: Vec<i32>
}

impl ScratchCard{
    pub fn check(&self) -> i32{
        let mut  result : i32 = 0;

        for winning_number in self.winning_numbers.as_slice() {
            if self.card_numbers.contains(&winning_number){
                if result == 0 { result = 1 } else { result*=2 }
            }
        }

        result
    }
}

pub fn run (path: &str) -> i32{
    let mut result : i32 = 0;
    let file_contents = fs::read_to_string(path).expect("missing input file");

    let mut scratch_cards : Vec<ScratchCard> = Vec::new();

    let rows = file_contents.split("\n");

    for row in rows {
        let mut card_split = row.split(":");
        let mut card = ScratchCard {
            id: 0,
            winning_numbers: Vec::new(),
            card_numbers: Vec::new()
        };

        for card_part in card_split {
            if card_part.contains("Card")
            {
                let name = i32::from_str(card_part.replace("Card", "").trim());

                if name.is_ok(){
                    card.id = name.unwrap();
                }

            }else {
                let mut card_value_parts = card_part.split("|").enumerate();

                let winning_values = card_value_parts.next();
                
                if winning_values.is_some() 
                {
                    for winning_value in winning_values.unwrap().1.split_whitespace() {
                        let value = i32::from_str(winning_value);
                        if value.is_ok()
                        {
                            card.winning_numbers.push(value.unwrap());
                        }
                    }
                }

                let card_values = card_value_parts.next();

                if card_values.is_some()
                {
                    for card_value in card_values.unwrap().1.split_whitespace() {
                        let value = i32::from_str(card_value);
                        if value.is_ok()
                        {
                            card.card_numbers.push(value.unwrap());
                        }
                    }
                }
            }
        }

        scratch_cards.push(card);
    }

    for scratch_card in scratch_cards {
        result += scratch_card.check();
    }

    result
}

#[test]
fn test(){
    assert_eq!(run("data/day4_test.txt"), 13);
}