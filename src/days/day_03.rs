use std::str::FromStr;

use crate::util::read_input;

enum Spot {
    Tree,
    Empty,
}

struct Direction {
    right: usize,
    down: usize,
}

struct Line {
    spots: Vec<Spot>,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(|ch| match ch {
                '.' => Ok(Spot::Empty),
                '#' => Ok(Spot::Tree),
                _ => Err(()),
            })
            .collect::<Result<Vec<Spot>, Self::Err>>()
            .map(|spots| Line { spots })
    }
}

fn traverse_slope(input: &[Line], direction: &Direction) -> usize {
    let mut counter = 0;
    let mut distance_right = 0;
    let mut distance_down = 0;

    loop {
        distance_down += direction.down;
        distance_right += direction.right;
        if distance_down >= input.len() {
            break;
        }
        let line = &input[distance_down];
        let spot = &line.spots[distance_right % line.spots.len()];
        counter += match spot {
            Spot::Tree => 1,
            _ => 0,
        };
    }
    return counter;
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Line> {
    read_input(input)
}

#[aoc(day3, part1)]
fn solve_part_one(input: &[Line]) -> usize {
    traverse_slope(input, &Direction { right: 3, down: 1 })
}

#[aoc(day3, part2)]
fn solve_part_two(input: &[Line]) -> usize {
    let directions = [
        Direction { right: 1, down: 1 },
        Direction { right: 3, down: 1 },
        Direction { right: 5, down: 1 },
        Direction { right: 7, down: 1 },
        Direction { right: 1, down: 2 },
    ];

    directions
        .iter()
        .fold(1, |acc, elem| acc * traverse_slope(input, elem))
}
