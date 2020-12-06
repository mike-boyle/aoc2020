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
    let input_length = input[0].spots.len();
    let spot_iter = (0..input_length)
        .cycle()
        .skip(direction.right)
        .step_by(direction.right);
    input
        .into_iter()
        .skip(direction.down)
        .step_by(direction.down)
        .zip(spot_iter)
        .filter(|(line, index)| {
            if let Spot::Tree = line.spots[*index] {
                true
            } else {
                false
            }
        })
        .count()
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
