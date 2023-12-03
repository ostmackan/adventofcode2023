use std::fs;
use std::iter::Enumerate;
use std::str::{FromStr, Split};

pub struct SchematicSymbol {
    pub pos: Position,
    pub symbol: char,
}
pub struct Position {
    pub row: usize,
    pub column: usize,
}
pub struct PartNumber {
    pub row: usize,
    pub columns: Vec<usize>,
    pub values: String,
}

impl SchematicSymbol {
    pub fn check_gear(&self, numbers: &[PartNumber]) -> i32 {
        let mut result = 0;
        let mut matched = 0;

        if self.symbol != '*' {
            return 0;
        }

        for number in numbers {
            if number.check_position(self.pos.row, self.pos.column) {
                println!("{}", number.values);
                matched += 1;

                if result == 0 {
                    result = number.to_i32();
                } else {
                    result *= number.to_i32();
                }
            }
        }

        if matched != 2 {
            return 0;
        }

        return result;
    }
    pub fn check(&self, numbers: &[PartNumber]) -> i32 {
        let mut result = 0;

        for number in numbers {
            if number.check_position(self.pos.row, self.pos.column) {
                println!("{}", number.values);
                result += number.to_i32();
            }
        }

        result
    }
}

impl PartNumber {
    pub fn to_i32(&self) -> i32 {
        let result = i32::from_str(self.values.as_str());

        if result.is_err() {
            return 0;
        }

        result.unwrap()
    }

    pub fn check_position(&self, row: usize, column: usize) -> bool {
        let row_diff = self.row.abs_diff(row);

        if self.columns.len() == 0 {
            return false;
        }

        if row_diff > 1 {
            return false;
        }

        let first_check = self.columns.first().unwrap().checked_sub(1);

        let first: usize;

        if first_check.is_some() {
            first = first_check.unwrap();
        } else {
            first = 0;
        }

        let last = self.columns.last().unwrap() + 1;

        if first <= column && last >= column {
            return true;
        }

        false
    }
}

pub fn run(path: &str) -> i32 {
    let file_contents = fs::read_to_string(path).expect("Missing input file");
    let mut result = 0;
    let mut symbols: Vec<SchematicSymbol> = Vec::new();
    let mut numbers: Vec<PartNumber> = Vec::new();

    let mut rows = file_contents.split('\n').enumerate();

    get_parts_and_numbers(&mut symbols, &mut numbers, &mut rows);

    for symbol in symbols {
        let value = symbol.check(numbers.as_slice());

        println!(
            "symbol: {} @ {} {} = {value} ",
            symbol.symbol, symbol.pos.row, symbol.pos.column
        );

        result += value;
    }

    return result;
}

pub fn get_parts_and_numbers(
    symbols: &mut Vec<SchematicSymbol>,
    numbers: &mut Vec<PartNumber>,
    rows: &mut Enumerate<Split<char>>,
) {
    while let Some(row) = rows.next() {
        let mut row_enumerator = row.1.chars().enumerate();
        let mut next_number = String::from("");
        let mut columns: Vec<usize> = Vec::new();

        while let Some(pos) = row_enumerator.next() {
            if pos.1.is_numeric() {
                columns.push(pos.0);
                next_number.push(pos.1);

                continue;
            } else if columns.len() > 0 {
                numbers.push(PartNumber {
                    row: row.0,
                    columns: columns.clone(),
                    values: next_number.clone(),
                });

                columns.clear();
                next_number.clear();
            }

            match pos.1 {
                '.' => continue,
                _ => {
                    symbols.push(SchematicSymbol {
                        pos: Position {
                            row: row.0,
                            column: pos.0,
                        },
                        symbol: pos.1,
                    });
                }
            }
        }

        if columns.len() > 0 {
            numbers.push(PartNumber {
                row: row.0,
                columns: columns.clone(),
                values: next_number.clone(),
            });

            columns.clear();
            next_number.clear();
        }
    }
}

#[test]
fn test() {
    assert_eq!(run("data/day3_test.txt"), 4361);
}
