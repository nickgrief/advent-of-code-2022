#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn solve_part1(input: &str) -> u32 {
    input.lines().fold(0, |mut accum, line| {
        let ranges: Vec<(u32, u32)> = line
            .split(',')
            .map(|part| {
                let bounds: Vec<u32> = part
                    .split('-')
                    .map(|side| side.parse::<u32>().unwrap())
                    .collect();
                (bounds[0], bounds[1])
            })
            .collect();
        if ranges[0].0 <= ranges[1].0 && ranges[1].1 <= ranges[0].1
            || ranges[0].0 >= ranges[1].0 && ranges[0].1 <= ranges[1].1
        {
            accum += 1;
        }
        accum
    })
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn solve_part2(input: &str) -> u32 {
    input.lines().fold(0, |mut accum, line| {
        let ranges: Vec<(u32, u32)> = line
            .split(',')
            .map(|part| {
                let bounds: Vec<u32> = part
                    .split('-')
                    .map(|side| side.parse::<u32>().unwrap())
                    .collect();
                (bounds[0], bounds[1])
            })
            .collect();
        if ranges[0].1 >= ranges[1].0 && ranges[0].1 <= ranges[1].1
            || ranges[0].0 <= ranges[1].1 && ranges[0].0 >= ranges[1].0
            || ranges[0].0 <= ranges[1].0 && ranges[1].1 <= ranges[0].1
            || ranges[0].0 >= ranges[1].0 && ranges[0].1 <= ranges[1].1
        {
            accum += 1;
        }
        accum
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve_part1(input), 2);
    }

    #[test]
    fn part2_works() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve_part2(input), 4);
    }
}
