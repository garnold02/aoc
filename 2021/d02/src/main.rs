fn main() {
    stdsol::solve(2021, 2, part1, part2);
}

const INPUT: &str = include_str!("input");

pub fn part1() -> i32 {
    let mut hor = 0;
    let mut ver = 0;

    INPUT
        .split_terminator('\n')
        .map(|l| Direction::new(l).unwrap())
        .for_each(|d| match d {
            Direction::Horizontal(val) => hor += val,
            Direction::Vertical(val) => ver += val,
        });

    hor * ver
}

pub fn part2() -> i32 {
    let mut hor = 0;
    let mut ver = 0;
    let mut aim = 0;

    INPUT
        .split_terminator('\n')
        .map(|l| Direction::new(l).unwrap())
        .for_each(|d| match d {
            Direction::Horizontal(val) => {
                hor += val;
                ver += aim * val;
            }
            Direction::Vertical(val) => aim += val,
        });

    hor * ver
}

enum Direction {
    Horizontal(i32),
    Vertical(i32),
}

impl Direction {
    fn new(line: &str) -> Option<Self> {
        let (cmd, val) = {
            let mut tokens = line.split(' ');
            (tokens.next()?, tokens.next()?)
        };
        let val: i32 = val.parse().ok()?;
        match cmd {
            "forward" => Some(Self::Horizontal(val)),
            "up" => Some(Self::Vertical(-val)),
            "down" => Some(Self::Vertical(val)),
            _ => None,
        }
    }
}
