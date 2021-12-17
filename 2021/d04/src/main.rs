use std::{iter::Peekable, str::Lines};

fn main() {
    stdsol::solve(2021, 4, part1, part2);
}

const INPUT: &str = include_str!("input");

pub fn part1() -> i32 {
    let (numbers, mut boards) = parse();
    let winners = find_all_winners(&mut boards, &numbers);
    winners[0]
}

pub fn part2() -> i32 {
    let (numbers, mut boards) = parse();
    let winners = find_all_winners(&mut boards, &numbers);
    *(winners.last().unwrap())
}

struct Cell {
    number: i32,
    marked: bool,
}

impl Cell {
    fn new(number: i32) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

struct Board {
    cells: [[Cell; 5]; 5],
    winner: bool,
}

impl Board {
    fn new(lines: &mut Peekable<Lines>) -> Self {
        let cells = [row(lines), row(lines), row(lines), row(lines), row(lines)];
        Self {
            cells,
            winner: false,
        }
    }

    fn mark(&mut self, number: i32) {
        for cell in self.cells.iter_mut().flatten() {
            if cell.number == number {
                cell.marked = true;
            }
        }
    }

    fn winner(&mut self) -> bool {
        if self.winner {
            return false;
        }

        for flip in [false, true] {
            for i in 0..5 {
                let mut winner = true;
                for j in 0..5 {
                    let check = if flip {
                        !self.cells[i][j].marked
                    } else {
                        !self.cells[j][i].marked
                    };

                    if check {
                        winner = false;
                        break;
                    }
                }
                if winner {
                    self.winner = true;
                    return true;
                }
            }
        }

        false
    }

    fn unmarked_sum(&self) -> i32 {
        self.cells
            .iter()
            .flatten()
            .filter(|c| !c.marked)
            .map(|c| c.number)
            .sum()
    }
}

fn row(lines: &mut Peekable<Lines>) -> [Cell; 5] {
    let numbers = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();

    [numbers[0], numbers[1], numbers[2], numbers[3], numbers[4]].map(|n| Cell::new(n))
}

fn parse() -> (Vec<i32>, Vec<Board>) {
    let mut lines = INPUT.lines().peekable();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();

    lines.next().unwrap();

    let mut boards = Vec::new();
    while let Some(_) = lines.peek() {
        boards.push(Board::new(&mut lines));
        lines.next();
    }

    (numbers, boards)
}

fn find_all_winners(boards: &mut [Board], numbers: &[i32]) -> Vec<i32> {
    let mut winners = Vec::new();
    for number in numbers.iter().copied() {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.winner() {
                winners.push(board.unmarked_sum() * number);
            }
        }
    }
    winners
}
