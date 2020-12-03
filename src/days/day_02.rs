use std::str::FromStr;

struct PasswordPolicyRange {
    lower_bound: usize,
    upper_bound: usize,
}
struct PasswordPolicy {
    range: PasswordPolicyRange,
    character: char,
}
struct PasswordItem {
    policy: PasswordPolicy,
    password: String,
}

use regex::Regex;

use crate::Solution;

#[derive(Debug)]
struct ParsePasswordItemError {}

impl FromStr for PasswordItem {
    type Err = ParsePasswordItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "^(?P<lower_bound>\\d+)-(?P<upper_bound>\\d+) (?P<character>\\w): (?P<password>\\w+)$"
            )
            .unwrap();
        }

        let captures = RE.captures(s).unwrap();

        return Ok(PasswordItem {
            password: String::from(captures.name("password").unwrap().as_str()),
            policy: PasswordPolicy {
                character: captures
                    .name("character")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap(),
                range: PasswordPolicyRange {
                    lower_bound: captures
                        .name("lower_bound")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                    upper_bound: captures
                        .name("upper_bound")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                },
            },
        });
    }
}

impl PasswordItem {
    fn conforms_to_range_policy(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|&c| c == self.policy.character)
            .count();
        return count >= self.policy.range.lower_bound && count <= self.policy.range.upper_bound;
    }

    fn conforms_to_position_policy(&self) -> bool {
        let mut chars = self.password.chars();
        let a = chars.nth(self.policy.range.lower_bound - 1).unwrap();
        let b = chars
            .nth(self.policy.range.upper_bound - self.policy.range.lower_bound - 1)
            .unwrap();

        return (a == self.policy.character) ^ (b == self.policy.character);
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
