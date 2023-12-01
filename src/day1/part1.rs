use std::fs;
use std::ops::Index;
use std::str::FromStr;

// Solves day1 part 1.
pub fn run(path: &str) -> i32{
    let file_contents = fs::read_to_string(&path).expect("load file error");

    let rows = file_contents.split("\n");

    let mut result = 0;

    for row in rows {
        let first = row.find(char::is_numeric);
        let last = row.rfind(char::is_numeric);

        if(first == None || last == None)
        {
            panic!("error");
        }

        let mut number = String::from("");

        number.push(row.chars().nth(first.unwrap()).unwrap());
        number.push(row.chars().nth(last.unwrap()).unwrap());

        let value = i32::from_str(&number);

        if(value.is_ok()){
            result += value.unwrap();
        }
    }

    return result;
}

#[test]
fn test_run(){
    assert_eq!(run("data/day1_test.txt"), 142);
}