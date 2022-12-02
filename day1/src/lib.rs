#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn solve_part1(input: &str) -> i32 {
    let mut max = 0;
    let mut total = 0;

    input.lines().for_each(|line| {
        if line.is_empty() {
            total = 0;
        } else {
            total += line.parse::<i32>().unwrap();
            if total > max {
                max = total;
            }
        }
    });

    max
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn solve_part2(input: &str) -> i32 {
    let mut max = vec![0, 0, 0];
    let mut total = 0;

    input.lines().for_each(|line| {
        if line.is_empty() {
            max.sort_unstable();
            total = 0;
        } else {
            total += line.parse::<i32>().unwrap();
            if max[0] < total {
                max[0] = total;
            }
        }
    });

    max.iter().sum()
}
