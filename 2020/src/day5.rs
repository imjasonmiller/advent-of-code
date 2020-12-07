use anyhow::Context;
use itertools::Itertools;

fn parse_seat(input: &str) -> usize {
    input
        .bytes()
        .map(|b| matches!(b, b'B' | b'R') as usize)
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
