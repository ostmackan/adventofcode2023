use std::fs;
use std::str::FromStr;
use crate::day6::shared::RaceRecord;

pub fn run(path: &str) -> i32{
    let file_contents = fs::read_to_string(path).expect("missing input file");
    let mut result = 0;
    let mut race_records: Vec<RaceRecord> = Vec::new();

    let mut rows = file_contents.split("\n").into_iter();

    let mut times_row = rows.next();
    let mut distance_row = rows.next();

    if times_row.is_none() || distance_row.is_none(){
        println!("failed to parse time & distance {file_contents}");
        return -1;
    }

    let mut times= times_row.replace("Time:").unwrap().split_whitespace().into_iter();
    let mut distances= distance_row.replace("Distance:").unwrap().split_whitespace().into_iter();

    while let Some(time) = times.next() {
        if let Some(distance) = distances.next() {
            let time_value = i64::from_str(time);
            let distance_value = i64::from_str(distance);

            if time_value.is_ok() || distance_value.is_ok(){
                race_records.push(RaceRecord{
                    time: time_value.unwrap(),
                    distance: distance_value.unwrap(),
                })
            }else {
                println!("{time}{distance} are not numbers");
            }

        }
    }

    for race_record in race_records {
        println!("Record: \t{}\t{}", race_record.time, race_record.distance);

        let better_records = race_record.find_better_runs();

        if result == 0 {
            result = better_records.len() as i32;
        } else {
            result *= better_records.len() as i32;
        }
    }

    result
}

#[test]
fn test(){
    assert_eq!(run("data/day6/test.txt"), 288);
}