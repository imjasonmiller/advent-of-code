// use anyhow::Context;

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let mut xs = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    xs.sort_unstable();

    let mut count = HashMap::<isize, usize>::new();

    for (a, b) in std::iter::once(0).chain(xs.into_iter()).tuple_windows() {
        *count.entry(b - a).or_default() += 1;
    }

    let result = count.get(&1).unwrap() * (count.get(&3).unwrap() + 1);

    Ok(result)
}

fn find_paths(xs: &HashSet<isize>, mut memo: &mut HashMap<isize, isize>, x: isize) -> isize {
    let mut paths = 0;

    for i in 1..=3 {
        if let Some(count) = memo.get(&(x - i)) {
            paths += count;
        } else if xs.contains(&(x - i)) {
            let count = find_paths(xs, &mut memo, x - i);
            memo.insert(x - i, count);
            paths += count;
        }
    }

    if x - 3 <= 0 {
        paths += 1;
    }

    paths
}

pub fn part2(input: &str) -> anyhow::Result<isize> {
    let xs = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<HashSet<_>>();
    let max = xs.iter().max().copied().unwrap();
    let mut memo = HashMap::<isize, isize>::new();

    let count = find_paths(&xs, &mut memo, max);

    Ok(count)
}
