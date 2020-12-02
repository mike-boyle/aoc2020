use std::{fmt::Debug, fs, str::FromStr};

pub fn read_input<T>(day: i32) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let path = format!("input/day-{:02}/input", day);
    let contents = fs::read_to_string(path).expect("Something went wrong with reading the file");
    let lines = contents.split("\n");
    let result: Vec<T> = lines
        .into_iter()
        .map(|line| -> T { line.parse().unwrap() })
        .collect();
    return result;
}
