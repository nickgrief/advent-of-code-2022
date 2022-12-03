#![feature(iter_array_chunks)]

use std::collections::{HashMap, HashSet};

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn solve_part1(input: &str) -> u32 {
    input.lines().fold(0u32, |mut accum, backpack| {
        let (first_part, second_part) = backpack.split_at(backpack.len() / 2);
        let mut current: u32 = 0;
        for char in first_part.chars() {
            if second_part.contains(char) {
                if char.is_lowercase() {
                    current = char as u32 - 96;
                } else {
                    current = char as u32 - 38;
                }
                break;
            }
        }
        accum += current;
        accum
    })
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .fold(0, |mut accum, chunk| {
            let mut letter_counts: HashMap<char, u32> = HashMap::new();
            for backpack in chunk {
                let mut unique_chars: HashSet<char> = HashSet::new();
                for char in backpack.chars() {
                    unique_chars.insert(char);
                }
                for char in unique_chars {
                    *letter_counts.entry(char).or_insert(0) += 1;
                }
            }
            let group_char = letter_counts
                .iter()
                .max_by(|a, b| a.1.cmp(b.1))
                .map(|(k, _)| k)
                .unwrap();
            if group_char.is_lowercase() {
                accum += *group_char as u32 - 96;
            } else {
                accum += *group_char as u32 - 38;
            }

            accum
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        assert_eq!(solve_part1(input), 157);
    }

    #[test]
    fn part2_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        assert_eq!(solve_part2(input), 70);
    }
}
