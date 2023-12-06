use std::fs;
use std::str::FromStr;
use crate::day5::shared::{Mapper, MappperRow};

pub fn run(path: &str) ->i64{
    let mut result = 0;
    let file_contents = fs::read_to_string(path).expect("missing input file");
    let mut rows =file_contents.split("\n").into_iter();
    let mut  seeds: Vec<i64> = Vec::new();
    let mut  mappers: Vec<Mapper> = Vec::new();

    let first = rows.next();

    if first.is_some() {
        let parts = first.unwrap().split(":");

        if let Some(part)= parts.last() {
            let mut  part_iter = part.split(" ").into_iter();
            while let Some(seed) =  part_iter.next() {
                if(seed.is_empty()){ continue;}

                let seed_value = i64::from_str(seed);
                let seed_range = part_iter.next().unwrap();
                                 let range = i64::from_str(seed_range);

                println!("{seed} {seed_range}");

                    if seed_value.is_ok() && range.is_ok(){
                        let start = seed_value.unwrap();
                        let end = start + range.unwrap();

                        println!("seed: {start} {}", end);

                        for i in start..end {
                            seeds.push(i);
                        }
                    }

                else {
                    println!("error seed: {part}")
                }

            }
        }
    }


    while let Some(row) = rows.next() {

        if row.is_empty() { continue; }

        let name_parts = row.split(" ").next();

        if name_parts.is_none() {
            println!("row {row} error");
        }

        let mut names = name_parts.unwrap().split("-to-");

        let input = names.next();
        let mapper_name = names.next();

        if input.is_none() || mapper_name.is_none(){
            println!("row {row} error");
        }

        let mut next_mapper = Mapper {
            name: mapper_name.unwrap().to_string(),
            input: input.unwrap().to_string(),
            rows: Vec::new()
        };

        while let Some(range_row) = rows.next() {
            if range_row.is_empty(){
                break;
            }

            let mut mapper_row = MappperRow{
                destination: 0,
                source: 0,
                range: 0,
                diff: 0
            };

            let ranges = range_row.split(" ").enumerate();

            for range_value in ranges {
                let value = i64::from_str(range_value.1);
                if value.is_err() {
                    println!("Major error: {}", range_value.1);

                    return -1;
                }

                match range_value.0 {
                    0 => mapper_row.destination = value.unwrap(),
                    1 => mapper_row.source = value.unwrap(),
                    2 => mapper_row.range = value.unwrap(),
                    _ => println!("Error {}", range_value.1)
                }
            }

            mapper_row.diff = mapper_row.destination - mapper_row.source;
            next_mapper.rows.push(mapper_row);
        }

        mappers.push(next_mapper);
    }

    let mut mapper_iter = mappers.as_slice();
    let mut list_of_results: Vec<i64> = Vec::new();

    let mut seed_len = seeds.len()/100;

    for seed in seeds.iter().enumerate() {
        let mut  seed_transformed : i64 = *seed.1;
        //println!("{seed}");

        for mapper in mapper_iter {
            let in_value = seed_transformed;
            seed_transformed = mapper.transform(in_value);
            //println!("{}-to-{} {} {}", mapper.input, mapper.name, in_value, seed_transformed);
        }

        list_of_results.push(seed_transformed);
    }

    result = *list_of_results.iter().min().unwrap();

    result
}

#[test]
fn test(){
    assert_eq!(run("data/day5_test.txt"),46);
}