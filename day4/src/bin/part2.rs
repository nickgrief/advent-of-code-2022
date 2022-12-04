fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", day4::solve_part2(&input));
}
