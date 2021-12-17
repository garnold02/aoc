fn main() {
    stdsol::solve(2021, 10, part1, part2);
}

const INPUT: &str = include_str!("input");

pub fn part1() -> i32 {
    INPUT
        .split_terminator('\n')
        .map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    c if "([{<".contains(c) => stack.push(pair(c).unwrap()),
                    c if ")]}>".contains(c) => {
                        if stack.pop().unwrap() != c {
                            return score1(c).unwrap();
                        }
                    }
                    _ => unreachable!(),
                }
            }

            0
        })
        .sum()
}

pub fn part2() -> i64 {
    let mut scores = INPUT
        .split_terminator('\n')
        .filter(|line| corrupted_score(line).is_none())
        .map(|line| completion_score(line).unwrap())
        .collect::<Vec<_>>();

    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    scores[(scores.len() - 1) / 2]
}

fn corrupted_score(line: &str) -> Option<i32> {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            c if "([{<".contains(c) => {
                stack.push(pair(c).unwrap());
            }
            c if ")]}>".contains(c) => {
                if stack.pop().unwrap() != c {
                    return score2(c);
                }
            }
            _ => unreachable!(),
        }
    }

    None
}

fn completion_score(line: &str) -> Option<i64> {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            c if "([{<".contains(c) => {
                stack.push(pair(c).unwrap());
            }
            c if ")]}>".contains(c) => {
                if stack.pop().unwrap() != c {
                    return None;
                }
            }
            _ => unreachable!(),
        }
    }

    let mut s = 0;
    while let Some(c) = stack.pop() {
        s *= 5;
        s += score2(c).unwrap() as i64;
    }

    Some(s)
}

fn pair(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn score1(c: char) -> Option<i32> {
    match c {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}

fn score2(c: char) -> Option<i32> {
    match c {
        ')' => Some(1),
        ']' => Some(2),
        '}' => Some(3),
        '>' => Some(4),
        _ => None,
    }
}
