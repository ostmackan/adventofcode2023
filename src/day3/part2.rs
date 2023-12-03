use crate::day3::part1::{get_parts_and_numbers, PartNumber, SchematicSymbol};
use std::fs;

pub fn run(path: &str) -> i32 {
    let file_contents = fs::read_to_string(path).expect("Missing input file");
    let mut result = 0;
    let mut symbols: Vec<SchematicSymbol> = Vec::new();
    let mut numbers: Vec<PartNumber> = Vec::new();

    let mut rows = file_contents.split('\n').enumerate();

    get_parts_and_numbers(&mut symbols, &mut numbers, &mut rows);

    for symbol in symbols {
        let value = symbol.check_gear(numbers.as_slice());

        println!(
            "symbol: {} @ {} {} = {value} ",
            symbol.symbol, symbol.pos.row, symbol.pos.column
        );

        result += value;
    }

    return result;
}

#[test]
fn test() {
    assert_eq!(run("data/day3_test.txt"), 467835);
}
