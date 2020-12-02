use crate::Solution;

fn find_numbers_summing(target: i32, count: i32, items: &Vec<i32>) -> Option<i32> {
    return match count {
        0 => match target {
            0 => Some(1),
            _ => None,
        },
        _ => items.iter().find_map(|&item| {
            match find_numbers_summing(target - item, count - 1, items) {
                None => None,
                Some(num) => Some(num * item),
            }
        }),
    };
}

pub struct Puzzle {}

impl Solution<i32> for Puzzle {
    fn day() -> i32 {
        1
    }

    fn solve_part_one(input: &Vec<i32>) -> i32 {
        find_numbers_summing(2020, 2, &input).unwrap()
    }

    fn solve_part_two(input: &Vec<i32>) -> i32 {
        find_numbers_summing(2020, 3, &input).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::read_input;

    use super::*;

    #[test]
    fn test_find_sum_default() {
        let input = find_numbers_summing(2020, 2, &vec![1721, 979, 366, 299, 675, 1456]);
        assert_eq!(input, Some(514579))
    }

    #[test]
    fn test_find_sum_default_2() {
        let input = find_numbers_summing(2020, 3, &vec![1721, 979, 366, 299, 675, 1456]);
        assert_eq!(input, Some(241861950))
    }

    #[test]
    fn test_find_sum() {
        let input = read_input::<i32>(1);
        assert_eq!(find_numbers_summing(2020, 2, &input), Some(440979))
    }

    #[test]
    fn test_find_sum_two() {
        let input = read_input::<i32>(1);
        assert_eq!(find_numbers_summing(2020, 3, &input), Some(82498112))
    }
}
