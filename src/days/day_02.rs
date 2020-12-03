use crate::util::read_input;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug)]
#[display("{min}-{max} {letter}: {password}")]
struct PasswordItem {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<PasswordItem> {
    read_input(input)
}

#[aoc(day2, part1)]
fn solve_part_one(input: &[PasswordItem]) -> usize {
    find_valid_password_count(input, |item| {
        let count = item.password.chars().filter(|&c| c == item.letter).count();
        return count >= item.min && count <= item.max;
    })
}

#[aoc(day2, part2)]
fn solve_part_two(input: &[PasswordItem]) -> usize {
    find_valid_password_count(input, |item| {
        let mut chars = item.password.chars();
        let a = chars.nth(item.min - 1);
        let b = chars.nth(item.max - item.min - 1);

        return (a == Some(item.letter)) ^ (b == Some(item.letter));
    })
}

fn find_valid_password_count<P>(input: &[PasswordItem], policy: P) -> usize
where
    P: Fn(&PasswordItem) -> bool,
{
    input.iter().filter(|&item| policy(item)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Vec<PasswordItem> {
        let text = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let items = text
            .iter()
            .map(|pw| -> PasswordItem { pw.parse().unwrap() })
            .collect();
        return items;
    }

    #[test]
    fn test_part_one() {
        assert_eq!(2, solve_part_one(&data()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1, solve_part_two(&data()));
    }
}
