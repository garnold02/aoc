fn main() {
    stdsol::solve(2021, 6, part1, part2);
}

const INPUT: &str = include_str!("input");

pub fn part1() -> usize {
    let mut colony = Colony::new();
    for _ in 0..80 {
        colony.iterate();
    }
    colony.len()
}

pub fn part2() -> usize {
    let mut colony = Colony::new();
    for _ in 0..256 {
        colony.iterate();
    }
    colony.len()
}
struct Colony {
    fish: [usize; 9],
}

impl Colony {
    fn new() -> Self {
        let mut fish = [0; 9];

        INPUT
            .trim()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .for_each(|n| fish[n] += 1);

        Self { fish }
    }

    fn iterate(&mut self) {
        self.fish.rotate_left(1);
        self.fish[6] += self.fish[8];
    }

    fn len(&self) -> usize {
        self.fish.iter().sum()
    }
}
