#[aoc(day6, part1)]
fn solve_part_one(input: &str) -> usize {
    solve(input, FilterType::Any)
}

#[aoc(day6, part2)]
fn solve_part_two(input: &str) -> usize {
    solve(input, FilterType::All)
}

enum FilterType {
    Any,
    All,
}

fn solve(input: &str, filter_type: FilterType) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            ('a'..='z')
                .into_iter()
                .filter(|&c| {
                    let mut lines = group.lines();
                    let closure = |l: &str| l.contains(c);
                    match filter_type {
                        FilterType::All => lines.all(closure),
                        FilterType::Any => lines.any(closure),
                    }
                })
                .count()
        })
        .sum()
}
