mod day1;
mod day2;

fn main() {
    let result = day2_part1();
    let result_part2 = day2_part2();

    println!("day1_part1: {result}");
    println!("day2_part2: {result_part2}");
}

fn day1(){
    let result = day1_part2();
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

fn day2_part1() -> i32 {
    return day2::part1::run("data/day2_input.txt");
}

fn day2_part2() -> i32{
    return day2::part2::run("data/day2_input.txt");
}
