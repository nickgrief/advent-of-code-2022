use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, char, digit1, multispace1, newline},
    },
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Move {
    amount: u32,
    from: u32,
    to: u32,
}

fn crate_parser(input: &str) -> IResult<&str, &str> {
    alt((delimited(char('['), alpha1, char(']')), tag("   ")))(input)
}

fn crate_line_parser(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(" "), crate_parser)(input)
}

fn stacks_parser(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list1(newline, crate_line_parser)(input)
}

fn number_line_parser(input: &str) -> IResult<&str, Vec<&str>> {
    terminated(many1(preceded(multispace1, digit1)), multispace1)(input)
}

fn move_parser(input: &str) -> IResult<&str, Move> {
    let (input, amount) = preceded(tag("move "), character::complete::u32)(input)?;
    let (input, from) = preceded(tag(" from "), character::complete::u32)(input)?;
    let (input, to) = preceded(tag(" to "), character::complete::u32)(input)?;
    let (input, _) = opt(multispace1)(input)?;

    Ok((input, Move { amount, from, to }))
}

fn moves_parser(input: &str) -> IResult<&str, Vec<Move>> {
    many1(move_parser)(input)
}

#[allow(clippy::type_complexity)]
fn input_parser(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<&str>, Vec<Move>)> {
    tuple((stacks_parser, number_line_parser, moves_parser))(input)
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn solve_part1(input: &str) -> String {
    let (_, (parsed_crate_data, _, moves)) = input_parser(input).unwrap();
    let mut crates = vec![];
    for _ in 0..parsed_crate_data[0].len() {
        crates.push(vec![]);
    }
    for line in parsed_crate_data.iter().rev() {
        for (i, c) in line.iter().enumerate() {
            if c != &"   " {
                crates[i].push(c);
            }
        }
    }

    for m in moves {
        for _ in 0..m.amount {
            let element = crates[m.from as usize - 1].pop().unwrap();
            crates[m.to as usize - 1].push(element);
        }
    }

    let mut output = vec![];
    for c in crates {
        output.push(**c.last().unwrap());
    }

    output.concat()
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn solve_part2(input: &str) -> String {
    let (_, (parsed_crate_data, _, moves)) = input_parser(input).unwrap();
    let mut crates = vec![];
    for _ in 0..parsed_crate_data[0].len() {
        crates.push(vec![]);
    }
    for line in parsed_crate_data.iter().rev() {
        for (i, c) in line.iter().enumerate() {
            if c != &"   " {
                crates[i].push(c);
            }
        }
    }

    for m in moves {
        let len = crates[m.from as usize - 1].len();
        let elements = crates[m.from as usize - 1].split_off(len - m.amount as usize);
        crates[m.to as usize - 1].extend(elements);
    }

    let mut output = vec![];
    for c in crates {
        output.push(**c.last().unwrap());
    }

    output.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let answer = solve_part1(INPUT);
        assert_eq!(answer, "CMZ");
    }

    #[test]
    fn part2_works() {
        let answer = solve_part2(INPUT);
        assert_eq!(answer, "MCD");
    }
}
