use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", day2::solve_part1(&input));
}
