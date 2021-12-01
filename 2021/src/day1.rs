pub fn part1(input: &str) -> anyhow::Result<usize> {
    let count = input
        .lines()
        .zip(input.lines().skip(1))
        .filter(|(a, b)| a < b)
        .count();

    Ok(count)
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    use itertools::Itertools;

    let count = input
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .tuple_windows()
        .filter(|(a, b, c, d)| a + b + c < b + c + d)
        .count();

    Ok(count)
}
