use anyhow::Context;
use itertools::Itertools;

fn parse_seat(input: &str) -> Option<usize> {
    let mut row = 0;
    let mut col = 0;

    for byte in input.bytes() {
        match byte {
            b'F' => row <<= 1,
            b'B' => row = (row << 1) + 1,
            b'L' => col <<= 1,
            b'R' => col = (col << 1) + 1,
            _ => return None,
        }
    }

    Some(row * 8 + col)
}

pub fn part1(input: &str) -> anyhow::Result<usize> {
    input
        .lines()
        .filter_map(parse_seat)
        .max()
        .context("No solution was found")
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    input
        .lines()
        .filter_map(parse_seat)
        .sorted()
        .tuple_windows()
        .find_map(|(a, b)| (a + 1 != b).then_some(a + 1))
        .context("No solution was found")
}
