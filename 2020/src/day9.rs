use anyhow::bail;

use std::collections::HashSet;

pub fn part1(input: &str) -> anyhow::Result<isize> {
    let xs = input
        .lines()
        .map(|line| line.parse::<isize>().expect("Could not parse number"))
        .collect::<Vec<_>>();

    for (i, x) in xs.iter().enumerate().skip(25) {
        let values = xs[i - 25..i].iter().copied().collect::<HashSet<isize>>();

        if !values.iter().any(|y| values.contains(&(x - y))) {
            return Ok(*x);
        }
    }

    bail!("No solution was found");
}

// 85848519

pub fn part2(input: &str) -> anyhow::Result<isize> {
    let xs = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    // println!("xs: {:?}", xs);

    for i in 0..xs.len() {
        for j in (i + 1)..xs.len() {
            let sum = xs[i..=j].iter().sum::<isize>();

            if sum == 85848519 {
                let min = xs[i..=j].iter().min().unwrap();
                let max = xs[i..=j].iter().max().unwrap();
                println!("min: {:?}, max: {:?}", min, max);
                println!("sum of both: {:?}", min + max);

                println!("values: {:?}", xs[i..=j].iter().collect::<Vec<_>>());

                // println!("i: {:?} j: {:?} sum: {:?}", i, j, sum);
                // println!("i: {:?} j: {:?}, added: {:?}", xs[i], xs[j], xs[i] + xs[j]);
                // println!("we found something...");
                return Ok(sum);
            }
        }
    }

    Ok(0)
    // todo!()
}
