use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

fn main() {
    simulate(2);
    simulate(10);
}

fn simulate(n: usize) {
    let mut rope = Rope {
        knots: vec![Knot { x: 0, y: 0 }; n],
    };

    let mut points = HashSet::new();

    for line in INPUT.lines() {
        let mut split = line.split(' ');

        let dir = match split.next().unwrap() {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!(),
        };

        points.insert(rope.knots.last().cloned().unwrap());

        let n: i32 = split.next().unwrap().parse().unwrap();
        for _ in 0..n {
            rope.step(dir);
            points.insert(rope.knots.last().cloned().unwrap());
        }
    }

    println!("{}", points.len());
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    fn step(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.knots[0].y -= 1,
            Dir::Down => self.knots[0].y += 1,
            Dir::Left => self.knots[0].x -= 1,
            Dir::Right => self.knots[0].x += 1,
        }

        for i in 0..self.knots.len() - 1 {
            let head = self.knots[i].clone();
            let tail = &mut self.knots[i + 1];
            tail.follow(head);
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn follow(&mut self, head: Knot) {
        let dx = head.x - self.x;
        let dy = head.y - self.y;

        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        }

        self.x += dx.signum();
        self.y += dy.signum();
    }
}
