use anyhow::Context;
use itertools::Itertools;

fn parse_seat(input: &str) -> usize {
    input
        .bytes()
        .filter_map(|b| match b {
            b'F' | b'L' => Some(0),
            b'B' | b'R' => Some(1),
            _ => None,
        })
        .fold(0, |a, b| (a << 1) + b)
}

pub fn part1(input: &str) -> anyhow::Result<usize> {
    input
        .lines()
        .map(parse_seat)
        .max()
        .context("No solution was found")
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    input
        .lines()
        .map(parse_seat)
        .sorted()
        .tuple_windows()
        .find_map(|(a, b)| (a + 1 != b).then_some(a + 1))
        .context("No solution was found")
}
