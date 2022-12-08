fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", day6::solve_part_1(&input));
}
