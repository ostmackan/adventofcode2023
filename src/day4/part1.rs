use std::fs;
use crate::day4::shared;
use crate::day4::shared::ScratchCard;

pub fn run (path: &str) -> i32{
    let mut result : i32 = 0;
    let file_contents = fs::read_to_string(path).expect("missing input file");

    let mut scratch_cards : Vec<ScratchCard> = Vec::new();

    shared::parse_scratch_cards(file_contents, &mut scratch_cards);

    for scratch_card in scratch_cards {
        result += scratch_card.check();
    }

    result
}

#[test]
fn test(){
    assert_eq!(run("data/day4_test.txt"), 13);
}