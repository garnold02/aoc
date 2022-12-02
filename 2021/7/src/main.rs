fn main() {
    stdsol::solve(2021, 7, part1, part2);
}

const INPUT: &str = include_str!("input");

pub fn part1() -> i32 {
    let positions = parse();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .map(|i| positions.iter().map(|p| cost1(*p, i)).sum())
        .min()
        .unwrap()
}

pub fn part2() -> i32 {
    let positions = parse();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .map(|i| positions.iter().map(|p| cost2(*p, i)).sum())
        .min()
        .unwrap()
}

fn parse() -> Vec<i32> {
    INPUT
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn cost1(a: i32, b: i32) -> i32 {
    (b - a).abs()
}

fn cost2(a: i32, b: i32) -> i32 {
    let n = cost1(a, b);
    n * (n + 1) / 2
}
