mod day1;

fn main() {
    let result = day1_part1();
    let part2 = day1_part2();
    println!("day1: {result}");
    println!("day1_part2: {part2}")
}

fn day1_part1() -> i32 {
    return day1::part1::run("data/day1_input.txt");
}

fn day1_part2() -> i32 {
    return day1::part2::run("data/day1_input.txt");
}
