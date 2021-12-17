fn main() {
    stdsol::solve(2021, 3, part1, part2);
}

const INPUT: &str = include_str!("input");

pub fn part1() -> i32 {
    let mut ones = [0; 12];
    let mut count = 0;

    INPUT.split_terminator('\n').for_each(|l| {
        for (i, b) in l.chars().enumerate() {
            if b == '1' {
                ones[i] += 1;
            }
        }
        count += 1;
    });

    let bits = ones.map(|o| (count < 2 * o) as i32);
    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, bit) in bits.iter().rev().enumerate() {
        gamma += (1 << i) * bit;
        epsilon += (1 << i) * (1 - bit);
    }

    gamma * epsilon
}

pub fn part2() -> i32 {
    let o2 = rating(false);
    let co2 = rating(true);
    o2 * co2
}

fn rating(co2: bool) -> i32 {
    let mut list = INPUT.split_terminator('\n').collect::<Vec<_>>();
    let mut pos = 0;

    while list.len() > 1 {
        let ones = list
            .iter()
            .filter(|l| l.chars().nth(pos).unwrap() == '1')
            .count();

        let check = if co2 {
            if list.len() < 2 * ones {
                '0'
            } else if list.len() > 2 * ones {
                '1'
            } else {
                '0'
            }
        } else {
            if list.len() < 2 * ones {
                '1'
            } else if list.len() > 2 * ones {
                '0'
            } else {
                '1'
            }
        };

        list.retain(|l| l.chars().nth(pos).unwrap() == check);
        pos += 1;
    }

    i32::from_str_radix(list[0], 2).unwrap()
}
