pub fn part1(input: &str) -> anyhow::Result<u32> {
    let (forward, depth) = input
        .lines()
        .filter_map(|s| {
            s.split_once(" ")
                .map(|(dir, val)| (dir, val.parse::<u32>().unwrap()))
        })
        .fold((0, 0), |(x, y), (dir, val)| match dir {
            "forward" => (x + val, y),
            "down" => (x, y + val),
            "up" => (x, y - val),
            _ => unreachable!(),
        });

    Ok(forward * depth)
}

pub fn part2(input: &str) -> anyhow::Result<u32> {
    let (forward, depth, _) = input
        .lines()
        .filter_map(|s| {
            s.split_once(" ")
                .map(|(dir, val)| (dir, val.parse::<u32>().unwrap()))
        })
        .fold((0, 0, 0), |(x, y, aim), (dir, val)| match dir {
            "forward" => (x + val, y + (aim * val), aim),
            "down" => (x, y, aim + val),
            "up" => (x, y, aim - val),
            _ => unreachable!(),
        });

    Ok(forward * depth)
}
