use anyhow::Context;

#[derive(Debug, Default)]
struct Program {
    index: isize,
    accumulated: isize,
    instructions: Vec<Instruction>,
    visited: HashMap<usize>,
}

#[derive(Debug)]
enum Instruction {
    Jmp(isize),
    Acc(isize),
    Nop(isize),
}

use Instruction::*;

impl Program {
    fn new(input: &str) -> Self {
        let instructions = input
            .lines()
            .map(|s| {
                let (op, arg) = s.split_once(" ").unwrap();
                let arg = arg.parse::<isize>().unwrap();

                match op {
                    "acc" => Acc(arg),
                    "jmp" => Jmp(arg),
                    "nop" => Nop(arg),
                    _ => panic!("Unknown operation: {}", op),
                }
            })
            .collect::<Vec<_>>();

        Self {
            instructions,
            ..Default::default()
        }
    }
}

impl Iterator for Program {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        // println!("Op: {:?}", self.instructions[self.index as usize]);
        match self.instructions[self.index as usize] {
            Acc(arg) => {
                self.accumulated += arg;
                self.index += 1;
            }
            Jmp(arg) => self.index += arg,
            Nop => self.index += 1,
        }

        Some((self.index, self.accumulated))
    }
}

pub fn part1(input: &str) -> anyhow::Result<isize> {
    let result = Program::new(input)
        .zip(Program::new(input).step_by(2))
        .skip(2)
        .inspect(|(a, b)| {
            if a.0 == b.0 {
                println!("---\nCycle detected");
            }

            println!("A: {:<?} B: {:>?}", a, b);

            if a.0 == b.0 {
                println!("---");
            }
        })
        .take(1000)
        .collect::<Vec<_>>();
    // .find_map(|((i, x), (j, y))| (i == j).then_some(x))
    Ok(5)
    // .context("No solution was found.")
}

pub fn part2(input: &str) -> anyhow::Result<isize> {}
