fn main() {
    stdsol::solve(2021, 11, part1, part2);
}

pub fn part1() -> i32 {
    let mut grid = parse();
    (0..100).map(|_| grid.iterate()).sum()
}

pub fn part2() -> i32 {
    let mut grid = parse();
    grid.sync()
}

fn parse() -> [[Octopus; 10]; 10] {
    const INPUT: &str = include_str!("input");
    INPUT
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| Octopus {
                    energy: c as u8 - b'0',
                    flashed: false,
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[derive(Debug)]
struct Octopus {
    energy: u8,
    flashed: bool,
}

trait Grid {
    fn iterate(&mut self) -> i32;
    fn sync(&mut self) -> i32;
}

impl Grid for [[Octopus; 10]; 10] {
    fn iterate(&mut self) -> i32 {
        for o in self.iter_mut().flatten() {
            o.energy += 1;
            o.flashed = false;
        }

        loop {
            let mut stop = true;
            for x in 0..10 {
                for y in 0..10 {
                    let mut current = &mut self[y][x];
                    if current.energy > 9 && !current.flashed {
                        current.flashed = true;
                        for a in -1..=1 {
                            for b in -1..=1 {
                                let a = x as i32 + a;
                                let b = y as i32 + b;
                                if (0..10).contains(&a) && (0..10).contains(&b) {
                                    let a = a as usize;
                                    let b = b as usize;
                                    let mut adjacent = &mut self[b][a];
                                    adjacent.energy += 1;
                                    if adjacent.energy > 9 {
                                        stop = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if stop {
                break;
            }
        }

        for o in self.iter_mut().flatten() {
            if o.flashed {
                o.energy = 0;
            }
        }

        self.iter().flatten().filter(|o| o.flashed).count() as i32
    }

    fn sync(&mut self) -> i32 {
        let mut n = 0;
        while self.iter().flatten().filter(|o| o.flashed).count() != 100 {
            self.iterate();
            n += 1;
        }
        n
    }
}
