use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", day1::solve_part2(&input));
}
