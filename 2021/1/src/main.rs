fn main() {
    stdsol::solve(2021, 1, part1, part2);
}

fn part1() -> i32 {
    include_str!("input")
        .split_terminator('\n')
        .map(|str| str.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count() as i32
}

fn part2() -> i32 {
    include_str!("input")
        .split_terminator('\n')
        .map(|str| str.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count() as i32
}
