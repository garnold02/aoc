use std::{collections::VecDeque, str::Lines};

static INPUT: &str = include_str!("input.txt");

fn main() {
    let mut stacks: Vec<VecDeque<char>> =
        vec![VecDeque::new(); (INPUT.lines().next().unwrap().len() + 1) / 4];

    let mut lines = INPUT.lines();

    for line in &mut lines {
        if !line.starts_with('[') {
            break;
        }

        let mut chars = line.chars();

        for stack in &mut stacks {
            let char = chars.nth(1).unwrap();

            if char != ' ' {
                stack.push_back(char);
            }

            chars.nth(1);
        }
    }

    lines.next().unwrap();

    part1(&mut stacks, &mut lines);

    let mut top = String::with_capacity(stacks.len());
    for stack in stacks {
        top.push(stack.front().copied().unwrap());
    }

    println!("{}", top);
}

fn part1(stacks: &mut [VecDeque<char>], lines: &mut Lines) {
    for line in lines {
        let mut split = line.split(' ');
        let quantity: i32 = split.nth(1).unwrap().parse().unwrap();
        let from: usize = split.nth(1).unwrap().parse().unwrap();
        let to: usize = split.nth(1).unwrap().parse().unwrap();

        for _ in 0..quantity {
            let c = stacks[from - 1].pop_front().unwrap();
            stacks[to - 1].push_front(c);
        }
    }
}

fn part2(stacks: &mut [VecDeque<char>], lines: &mut Lines) {
    for line in lines {
        let mut split = line.split(' ');
        let quantity: usize = split.nth(1).unwrap().parse().unwrap();
        let from: usize = split.nth(1).unwrap().parse().unwrap();
        let to: usize = split.nth(1).unwrap().parse().unwrap();

        let mut all = Vec::with_capacity(quantity);

        for _ in 0..quantity {
            let c = stacks[from - 1].pop_front().unwrap();
            all.push(c);
        }

        all.reverse();

        for c in all {
            stacks[to - 1].push_front(c);
        }
    }
}
