use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug)]
#[display("{min}-{max} {letter}: {password}")]
struct PasswordItem {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

use regex::Regex;

use crate::Solution;

impl PasswordItem {
    fn conforms_to_range_policy(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();
        return count >= self.min && count <= self.max;
    }

    fn conforms_to_position_policy(&self) -> bool {
        let mut chars = self.password.chars();
        let a = chars.nth(self.min - 1);
        let b = chars.nth(self.max - self.min - 1);

        return (a == Some(self.letter)) ^ (b == Some(self.letter));
    }
}

fn find_valid_password_count(
    input: &Vec<PasswordItem>,
    policy: &dyn Fn(&PasswordItem) -> bool,
) -> i32 {
    return input.iter().filter(|&item| policy(item)).count() as i32;
}

pub struct Day {}

impl Solution<PasswordItem> for Day {
    fn day() -> i32 {
        2
    }

    fn solve_part_one(input: &Vec<PasswordItem>) -> i32 {
        find_valid_password_count(input, &PasswordItem::conforms_to_range_policy)
    }

    fn solve_part_two(input: &Vec<PasswordItem>) -> i32 {
        find_valid_password_count(input, &PasswordItem::conforms_to_position_policy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sum_default() {
        let policies = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let things = policies
            .iter()
            .map(|pw| -> PasswordItem { pw.parse().unwrap() })
            .collect();
        let count = find_valid_password_count(&things, &PasswordItem::conforms_to_position_policy);
        assert_eq!(1, count);
    }

    #[test]
    fn solution_part_one() {
        assert_eq!(517, Day::part_one());
    }

    #[test]
    fn solution_part_two() {
        assert_eq!(284, Day::part_two());
    }
}
