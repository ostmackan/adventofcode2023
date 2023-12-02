use std::fs;
use crate::day2::shared;
use crate::day2::shared::GameInstance;

pub fn run(path: &str) -> i32{
    let file_contents = fs::read_to_string(path).expect("missing input file");
    let mut games : Vec<GameInstance> = Vec::new();
    let mut result: i32 = 0;

    shared::get_games(file_contents, &mut games);

    for game in games {
        let max = game.max_pull();
        result += max.score();
    }

    return result;
}

#[test]
fn test() {
    assert_eq!(run("data/day2_test.txt"), 2286);
}