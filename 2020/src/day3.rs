fn count_trees(input: &str, down: usize, right: usize) -> usize {
    input
        .lines()
        .step_by(down)
        .enumerate()
        .filter(|(i, row)| {
            row.chars()
                .cycle()
                .nth(i * right)
                .map_or(false, |c| c == '#')
        })
        .count()
}

pub fn part1(input: &str) -> anyhow::Result<usize> {
    Ok(count_trees(input, 1, 3))
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let tree_count = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .copied()
        .map(|(down, right)| count_trees(input, down, right))
        .inspect(|x| println!("x: {:?}", x))
        .product::<usize>();

    Ok(tree_count)
}
