use std::fs;
use std::iter::Enumerate;
use std::os::unix::raw::uid_t;
use std::slice::Iter;
use crate::day4::shared;
use crate::day4::shared::ScratchCard;

pub fn run(path: &str) -> i32 {
    let mut result: i32 = 0;
    let file_contents = fs::read_to_string(path).expect("missing input file");
    let mut scratch_cards: Vec<ScratchCard> = Vec::new();

    shared::parse_scratch_cards(file_contents, &mut scratch_cards);

    let mut enumerator = scratch_cards.iter().enumerate();


    for scratch_card in enumerator {
        let wins = scratch_card.1.check_wins();

        println!("{} {} {result}", scratch_card.1.id, wins);

        if wins > 0 {
            let more_wins = get_more_wins(scratch_cards.to_vec(), scratch_card, wins);
            result += more_wins;
        }

        result+=1;
        println!("result update {result}");
    }


    result
}

fn get_more_wins(scratch_cards:Vec<ScratchCard>, scratch_card: (usize, &ScratchCard), wins: i32) -> i32 {

    let mut result =  0;

    let mut  number = wins;

    let pos = scratch_card.0 + 1;
    //println!("More wins {wins} @ {pos}-{number}");

    let mut cards = scratch_cards.iter().enumerate();

    let count = scratch_cards.len();

    let mut  extra_card = cards.nth(pos);

    while number > 0 {

        //println!("extra: i: {number}");


        if extra_card.is_some() {
            let more_wins = extra_card.unwrap().1.check_wins();
            result+=1;

            //println!("extra: card={} {result} + {more_wins} - ({pos} {number})", extra_card.unwrap().1.id);


            if more_wins > 0 {
                result += get_more_wins(scratch_cards.to_vec(), extra_card.unwrap(), more_wins);
            }

            //println!("extra: result update {result}");
        } else {
            return result;
        }

        number-=1;
        extra_card = cards.next();
    }


    result
}

#[test]
fn test(){
    assert_eq!(run("data/day4_test.txt"), 30);
}