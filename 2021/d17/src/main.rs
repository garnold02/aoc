use std::ops::RangeInclusive;

fn main() {
    stdsol::solve(2021, 17, part1, part2);
}

pub fn part1() -> i32 {
    let target = Target {
        hor: 79..=137,
        ver: -176..=-117,
    };

    let mut probes = vec![];

    for sx in 0..1000 {
        for sy in 0..1000 {
            let mut probe = Probe {
                x: 0,
                y: 0,
                vx: sx,
                vy: sy,
                max: 0,
            };

            while !target.hopeless(&probe) {
                probe.step();
                if target.intersects(&probe) {
                    probes.push(probe);
                    break;
                }
            }
        }
    }

    let winner = probes
        .into_iter()
        .max_by(|p1, p2| p1.max.partial_cmp(&p2.max).unwrap())
        .unwrap();

    winner.max
}

pub fn part2() -> i32 {
    let target = Target {
        hor: 79..=137,
        ver: -176..=-117,
    };

    let mut probes = vec![];

    for sx in 0..500 {
        for sy in -1000..1000 {
            let mut probe = Probe {
                x: 0,
                y: 0,
                vx: sx,
                vy: sy,
                max: 0,
            };

            while !target.hopeless(&probe) {
                probe.step();
                if target.intersects(&probe) {
                    probes.push(probe);
                    break;
                }
            }
        }
    }

    probes.len() as i32
}

struct Target {
    hor: RangeInclusive<i32>,
    ver: RangeInclusive<i32>,
}

impl Target {
    fn intersects(&self, probe: &Probe) -> bool {
        self.hor.contains(&probe.x) && self.ver.contains(&probe.y)
    }

    fn hopeless(&self, probe: &Probe) -> bool {
        &probe.x > self.hor.end() || &probe.y < self.ver.start()
    }
}

struct Probe {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    max: i32,
}

impl Probe {
    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.vx -= self.vx.signum();
        self.vy -= 1;

        if self.y > self.max {
            self.max = self.y;
        }
    }
}
