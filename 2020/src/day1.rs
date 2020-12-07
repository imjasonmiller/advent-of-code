use anyhow::Context;

use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part1(input: &str) -> anyhow::Result<u32> {
    let xs = HashSet::<u32>::from_iter(
        input
            .lines()
            .map(|s| s.parse::<u32>().expect("Could not parse value")),
    );

    xs.iter()
        .find_map(|x| xs.get(&(2020 - x)).map(|y| x * y))
        .context("No solution was found")
}

pub fn part2(input: &str) -> anyhow::Result<i32> {
    let xs = HashSet::<i32>::from_iter(
        input
            .lines()
            .map(|s| s.parse::<i32>().expect("Could not parse value")),
    );

    xs.iter()
        .find_map(|x| {
            xs.iter()
                .filter(|&y| x != y)
                .find_map(|y| xs.get(&(2020 - x - y)).map(|z| x * y * z))
        })
        .context("No solution was found")
}
