use std::fs;
use std::str::FromStr;

pub fn run(path: &str) -> i32 {
    let file_contents = fs::read_to_string(&path).expect("load file error");

    let rows = file_contents.split("\n");

    let numbers = vec![
        "one".to_string(),
        "two".into(),
        "three".into(),
        "four".into(),
        "five".into(),
        "six".into(),
        "seven".into(),
        "eight".into(),
        "nine".into(),
    ];

    let mut result = 0;

    for row in rows {
        let mut number = String::from("");

        let mut first = row.find(char::is_numeric);
        let mut last = row.rfind(char::is_numeric);
        let mut first_value: char;
        let mut last_value: char = ' ';

        if first.is_some() {
            first_value = row.chars().nth(first.unwrap()).unwrap();
        } else {
            first_value = ' '
        }

        if last.is_some() {
            last_value = row.chars().nth(last.unwrap()).unwrap();
        }

        if first.is_none() {
            first = Some(0);
        }

        if last.is_none() {
            last = Some(number.len() + 1);
        }

        for n in &numbers {
            let first_n = row.find(n.as_str());

            if first_n.is_some() {
                if first >= first_n {
                    first = first_n;
                    first_value = match n.as_str() {
                        "one" => '1',
                        "two" => '2',
                        "three" => '3',
                        "four" => '4',
                        "five" => '5',
                        "six" => '6',
                        "seven" => '7',
                        "eight" => '8',
                        "nine" => '9',
                        _ => '0',
                    };
                }
            }
        }

        for n in &numbers {
            let last_n = row.rfind(n.as_str());

            if last_n.is_some() {
                if last < last_n {
                    last = last_n;
                    last_value = match n.as_str() {
                        "one" => '1',
                        "two" => '2',
                        "three" => '3',
                        "four" => '4',
                        "five" => '5',
                        "six" => '6',
                        "seven" => '7',
                        "eight" => '8',
                        "nine" => '9',
                        _ => '0',
                    };
                }
            }
        }

        number.push(first_value);
        number.push(last_value);

        println!("{row} -> {number}");

        let value = i32::from_str(&number);

        if value.is_ok() {
            result += value.unwrap();
        }
    }

    return result;
}

#[test]
fn test_run() {
    assert_eq!(run("data/day1_part2_test.txt"), 281);
}
