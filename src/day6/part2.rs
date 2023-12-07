use std::{fs, i64};
use std::str::FromStr;
use crate::day6::shared;

pub fn run(path: &str) -> i32{
    let file_contents = fs::read_to_string(path).expect("missing input file");

    let mut rows = file_contents.split("\n").into_iter();

    let mut times_row = rows.next();
    let mut distance_row = rows.next();

    if times_row.is_none() || distance_row.is_none(){
        println!("failed to parse time & distance {file_contents}");
        return -1;
    }

    let mut time= times_row.unwrap().replace("Time:", "").replace(char::is_whitespace, "");
    let mut distance= distance_row.unwrap().replace("Distance:", "").replace(char::is_whitespace, "");

    println!("time: {} distance {}", time, distance);

    let time_value = i64::from_str(time.as_str());
    let distance_value = i64::from_str(distance.as_str());

    if time_value.is_ok() && distance_value.is_ok() {
        let race_record = shared::RaceRecord{
            time: time_value.unwrap(),
            distance: distance_value.unwrap()
        };

        let better_records = race_record.find_better_runs();

        return better_records.len() as i32;
    }


    println!("error parsing time & distance");


    return 0;
}

#[test]
fn test(){
    assert_eq!(run("data/day6/test.txt"), 71503);
}