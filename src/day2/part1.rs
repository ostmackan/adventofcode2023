use std::fs;
use crate::day2::shared;
use crate::day2::shared::GameInstance;


pub fn run(path: &str) -> i32 {
    let file_contents = fs::read_to_string(path).expect("missing input file");
    let mut games: Vec<GameInstance> = Vec::new();

    shared::get_games(file_contents, &mut games);

    let number_of_games = games.len();
    let mut result = 0;

    for game in games {
        if game.is_correct(12, 13, 14) {
            result += game.id;
        }
    }

    println!("found {number_of_games} games");

    return result;
}

#[test]
fn test_part1() {
    assert_eq!(run("data/day2_test.txt"), 8);
}
