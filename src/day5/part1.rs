use std::fs;

pub fn run(path: &str) ->i32{
    let mut result = 0;
    let file_contens = fs::read_to_string(path).expect("missing input file");

    result
}

#[test]
fn test(){
    assert_eq!(run("data/day5_test.txt"),0);
}