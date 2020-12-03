use std::{fmt::Debug, str::FromStr};

pub fn read_input<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .into_iter()
        .map(|line| -> T { line.parse().unwrap() })
        .collect()
}
