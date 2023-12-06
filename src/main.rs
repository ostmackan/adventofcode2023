mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    day5();
}

fn day5(){
    //let part1 = day5::part1::run("data/day5_input.txt");
    let part2 = day5::part2::run("data/day5_input.txt");

    println!("day5_part1 {part1}");
    println!("day5_part2 {part2}");
}

fn day4(){
    let part1 = day4::part1::run("data/day4_input.txt");
    let part2 = day4::part2::run("data/day4_input.txt");

    println!("day4_part1 {part1}");
    println!("day4_part2 {part2}");
}

fn day3() {
    let part1 = day3_part1();
    let part2 = day3_part2();

    println!("day3_part1: {part1}");
    println!("day3_part2: {part2}");
}

fn day2() {
    let result = day2_part1();
    let result_part2 = day2_part2();

    println!("day1_part1: {result}");
    println!("day2_part2: {result_part2}");
}

fn day1() {
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

fn day2_part2() -> i32 {
    return day2::part2::run("data/day2_input.txt");
}

fn day3_part1() -> i32 {
    return day3::part1::run("data/day3_input.txt");
}

fn day3_part2() -> i32 {
    return day3::part2::run("data/day3_input.txt");
}
