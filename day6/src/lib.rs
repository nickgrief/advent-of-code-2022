#[must_use]
pub fn solve_part_1(input: &str) -> u32 {
    let marker_length = 4;
    let mut marker: u32 = marker_length - 1;
    let mut are_unique = false;
    while !are_unique {
        marker += 1;
        let chars: Vec<char> = input[(marker - marker_length) as usize..marker as usize]
            .chars()
            .collect();
        let mut unique_chars = chars.clone();
        unique_chars.sort_unstable();
        unique_chars.dedup();
        are_unique = chars.len() == unique_chars.len();
    }
    marker
}

#[must_use]
pub fn solve_part_2(input: &str) -> u32 {
    let marker_length = 14;
    let mut marker: u32 = marker_length - 1;
    let mut are_unique = false;
    while !are_unique {
        marker += 1;
        let chars: Vec<char> = input[(marker - marker_length) as usize..marker as usize]
            .chars()
            .collect();
        let mut unique_chars = chars.clone();
        unique_chars.sort_unstable();
        unique_chars.dedup();
        are_unique = chars.len() == unique_chars.len();
    }
    marker
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part_1_1() {
        assert_eq!(7, solve_part_1(INPUT1));
    }
    #[test]
    fn part_1_2() {
        assert_eq!(5, solve_part_1(INPUT2));
    }
    #[test]
    fn part_1_3() {
        assert_eq!(6, solve_part_1(INPUT3));
    }
    #[test]
    fn part_1_4() {
        assert_eq!(10, solve_part_1(INPUT4));
    }
    #[test]
    fn part_1_5() {
        assert_eq!(11, solve_part_1(INPUT5));
    }

    #[test]
    fn part_2_1() {
        assert_eq!(19, solve_part_2(INPUT1));
    }
    #[test]
    fn part_2_2() {
        assert_eq!(23, solve_part_2(INPUT2));
    }
    #[test]
    fn part_2_3() {
        assert_eq!(23, solve_part_2(INPUT3));
    }
    #[test]
    fn part_2_4() {
        assert_eq!(29, solve_part_2(INPUT4));
    }
    #[test]
    fn part_2_5() {
        assert_eq!(26, solve_part_2(INPUT5));
    }
}
