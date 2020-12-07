use std::collections::HashSet;

fn group_union(group: &str) -> Option<usize> {
    group
        .lines()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .fold_first(|a, b| a.union(&b).copied().collect())
        .map(|set| set.len())
}

fn group_intersection(group: &str) -> Option<usize> {
    group
        .lines()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .fold_first(|a, b| a.intersection(&b).copied().collect())
        .map(|set| set.len())
}

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let count = input.split("\n\n").filter_map(group_union).sum();

    Ok(count)
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let count = input.split("\n\n").filter_map(group_intersection).sum();

    Ok(count)
}
