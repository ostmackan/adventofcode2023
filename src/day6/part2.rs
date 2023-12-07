use std::fs;

pub fn run(path: &str) -> i32{
    let file_contents = fs::read_to_string(path).expect("missing input file");

    return 0;
}

#[test]
fn test(){
    assert_eq!(run("data/day6/test.txt"), 0);
}