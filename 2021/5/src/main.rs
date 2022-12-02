use std::collections::HashMap;

fn main() {
    stdsol::solve(2021, 5, part1, part2);
}

const INPUT: &str = include_str!("input");

type Point = (i32, i32);
type Line = (Point, Point);

pub fn part1() -> i32 {
    let lines = {
        let mut lines = lines();
        lines.retain(|l| l.0 .0 == l.1 .0 || l.0 .1 == l.1 .1);
        lines
    };

    let mut points: HashMap<Point, i32> = HashMap::new();

    for line in lines {
        if line.0 .0 == line.1 .0 {
            let min = line.0 .1.min(line.1 .1);
            let max = line.0 .1.max(line.1 .1);

            for i in min..=max {
                let point = (line.0 .0, i);
                *points.entry(point).or_default() += 1;
            }
        } else if line.0 .1 == line.1 .1 {
            let min = line.0 .0.min(line.1 .0);
            let max = line.0 .0.max(line.1 .0);

            for i in min..=max {
                let point = (i, line.0 .1);
                *points.entry(point).or_default() += 1;
            }
        }
    }

    points.iter().filter(|(_, i)| **i > 1).count() as i32
}

pub fn part2() -> i32 {
    let lines = lines();
    let mut points: HashMap<Point, i32> = HashMap::new();

    for line in lines {
        let dir = (line.1 .0 - line.0 .0, line.1 .1 - line.0 .1);
        let sign = (dir.0.signum(), dir.1.signum());
        let step = dir.0.abs().max(dir.1.abs());

        for i in 0..=step {
            let vec = (i * sign.0, i * sign.1);
            let point = (line.0 .0 + vec.0, line.0 .1 + vec.1);
            *points.entry(point).or_default() += 1;
        }
    }

    points.iter().filter(|(_, i)| **i > 1).count() as i32
}

fn lines() -> Vec<Line> {
    INPUT
        .split_terminator('\n')
        .map(|l| {
            let mut points = l.split("->");

            let mut p1 = points.next().unwrap().trim().split(',');
            let mut p2 = points.next().unwrap().trim().split(',');

            let p1: Point = (
                p1.next().unwrap().parse().unwrap(),
                p1.next().unwrap().parse().unwrap(),
            );

            let p2: Point = (
                p2.next().unwrap().parse().unwrap(),
                p2.next().unwrap().parse().unwrap(),
            );

            (p1, p2)
        })
        .collect()
}
