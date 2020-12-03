use crate::util::read_input;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<i32> {
    read_input(input)
}

#[aoc(day1, part1)]
fn solve_part_one(input: &[i32]) -> i32 {
    find_numbers_summing(2020, 2, &input).unwrap()
}

#[aoc(day1, part2)]
fn solve_part_two(input: &[i32]) -> i32 {
    find_numbers_summing(2020, 3, &input).unwrap()
}

fn find_numbers_summing(target: i32, count: i32, items: &[i32]) -> Option<i32> {
    match count {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sum_default() {
        let input = find_numbers_summing(2020, 2, &vec![1721, 979, 366, 299, 675, 1456]).unwrap();
        assert_eq!(input, 514579)
    }

    #[test]
    fn test_find_sum_default_2() {
        let input = find_numbers_summing(2020, 3, &vec![1721, 979, 366, 299, 675, 1456]).unwrap();
        assert_eq!(input, 241861950)
    }
}
