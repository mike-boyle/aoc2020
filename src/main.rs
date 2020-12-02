use std::{fmt::Debug, str::FromStr};

use util::read_input;

mod days;
mod util;

pub trait Solution<TInput>
where
    TInput: FromStr,
    <TInput as FromStr>::Err: Debug,
{
    fn day() -> i32;

    fn read_input() -> Vec<TInput> {
        read_input(Self::day())
    }

    fn part_one() -> i32 {
        let input = Self::read_input();
        return Self::solve_part_one(&input);
    }

    fn part_two() -> i32 {
        let input = Self::read_input();
        return Self::solve_part_one(&input);
    }

    fn solve() -> String {
        let part_one = Self::part_one();
        let part_two = Self::part_two();
        return format!("Part 1: {}, Part 2: {}", part_one, part_two);
    }

    fn solve_part_one(input: &Vec<TInput>) -> i32;
    fn solve_part_two(input: &Vec<TInput>) -> i32;
}

fn main() {
    let result = days::day_01::Puzzle::solve();
    println!("{}", result);
}
