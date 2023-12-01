mod day1;

fn main() {
    let result = day1_part1();
    println!("day1: {result}");
}

fn day1_part1() -> i32 {
    return day1::part1::run("data/day1_input.txt");
}
