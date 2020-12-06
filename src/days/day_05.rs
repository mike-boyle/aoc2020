use crate::util::read_input;

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<usize> {
    read_input::<String>(input)
        .into_iter()
        .map(|line| {
            line.chars()
                .fold(0, |acc, ch| (acc << 1) | matches!(ch, 'B' | 'R') as usize)
        })
        .collect()
}

#[aoc(day5, part1)]
fn solve_part_one(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part_two(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort_unstable();
    let gap = input
        .windows(2)
        .find(|window| window[0] + 1 != window[1])
        .unwrap();

    gap[0] + 1
}
