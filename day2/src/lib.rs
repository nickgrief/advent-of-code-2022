#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn solve_part1(input: &str) -> u32 {
    input.lines().fold(0, |mut accum, line| {
        let enemy = line.chars().next().unwrap();
        let player = line.chars().last().unwrap();

        accum += match enemy {
            'A' => match player {
                'X' => 4,
                'Y' => 8,
                'Z' => 3,
                _ => panic!("Wrond input data!"),
            },
            'B' => match player {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => panic!("Wrond input data!"),
            },
            'C' => match player {
                'X' => 7,
                'Y' => 2,
                'Z' => 6,
                _ => panic!("Wrond input data!"),
            },
            _ => panic!("Wrond input data!"),
        };
        accum
    })
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn solve_part2(input: &str) -> u32 {
    input.lines().fold(0, |mut accum, line| {
        let enemy = line.chars().next().unwrap();
        let result = line.chars().last().unwrap();

        let player = match enemy {
            'A' => match result {
                'X' => 'Z',
                'Y' => 'X',
                'Z' => 'Y',
                _ => panic!("Wrond input data!"),
            },
            'B' => match result {
                'X' => 'X',
                'Y' => 'Y',
                'Z' => 'Z',
                _ => panic!("Wrond input data!"),
            },
            'C' => match result {
                'X' => 'Y',
                'Y' => 'Z',
                'Z' => 'X',
                _ => panic!("Wrond input data!"),
            },
            _ => panic!("Wrond input data!"),
        };

        accum += match enemy {
            'A' => match player {
                'X' => 4,
                'Y' => 8,
                'Z' => 3,
                _ => 0,
            },
            'B' => match player {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => 0,
            },
            'C' => match player {
                'X' => 7,
                'Y' => 2,
                'Z' => 6,
                _ => 0,
            },
            _ => 0,
        };
        accum
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "A Y
B X
C Z";
        assert_eq!(solve_part1(input), 15);
    }

    #[test]
    fn part2_works() {
        let input = "A Y
B X
C Z";
        assert_eq!(solve_part2(input), 12);
    }
}
