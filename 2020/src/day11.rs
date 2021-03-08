#[derive(Debug, Clone, PartialEq)]
struct Seats {
    grid: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl Seats {
    fn new(s: &str) -> Self {
        let rows = s.lines().count();
        let cols = s.lines().nth(0).map(|l| l.chars().count()).unwrap();

        let mut grid = vec![vec![Cell::Floor; cols + 2]; rows + 2];

        for (i, row) in s.lines().enumerate() {
            for (j, col) in row.char_indices() {
                grid[i + 1][j + 1] = match col {
                    'L' => Cell::Empty,
                    '.' => Cell::Floor,
                    _ => panic!("Invalid char: \"{:?}\"", col),
                }
            }
        }

        Self { grid, rows, cols }
    }

    fn get_neigbors(&self, row: usize, col: usize) -> [Cell; 8] {
        [
            self.grid[row - 1][col - 1],
            self.grid[row - 1][col],
            self.grid[row - 1][col + 1],
            self.grid[row][col - 1],
            self.grid[row][col + 1],
            self.grid[row + 1][col - 1],
            self.grid[row + 1][col],
            self.grid[row + 1][col + 1],
        ]
    }

    fn curr(&self) -> Vec<Vec<Cell>> {
        self.grid.clone()
    }

    fn count_occupied(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&x| *x == Cell::Occupied).count())
            .sum()
    }

    fn next(&mut self) -> Vec<Vec<Cell>> {
        let mut next = vec![vec![Cell::Floor; self.grid[0].len()]; self.grid.len()];

        for (i, row) in self.grid.iter().enumerate().skip(1).take(self.rows) {
            for (j, col) in row.iter().enumerate().skip(1).take(self.cols) {
                let occupied = self
                    .get_neigbors(i, j)
                    .iter()
                    .filter(|&x| *x == Cell::Occupied)
                    .count();

                next[i][j] = match *col {
                    Cell::Empty if occupied == 0 => Cell::Occupied,
                    Cell::Occupied if occupied >= 4 => Cell::Empty,
                    v => v,
                }
            }
        }

        self.grid = next;
        self.grid.clone()
    }
}

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let mut seats = Seats::new(input);

    // use std::time::Duration;
    // let now = std::time::Instant::now();

    while seats.curr().clone() != seats.next() {}

    let occupied = seats.count_occupied();

    // println!("time: {:?}", now.elapsed().as_millis());

    Ok(occupied)
}

pub fn part2(_input: &str) -> anyhow::Result<i32> {
    todo!()
}
