use std::fmt::{Debug, Display, Formatter};
use std::time::{Duration, Instant};

pub fn solve<A, B>(year: i32, day: i32, part1: impl Fn() -> A, part2: impl Fn() -> B)
where
    A: Debug,
    B: Debug,
{
    println!("Advent of Code {} - Day {}", year, day);

    let start = Instant::now();
    let part1 = part1();
    let middle = Instant::now();
    let part2 = part2();
    let end = Instant::now();

    let time1: Time = (middle - start).into();
    let time2: Time = (end - middle).into();

    println!("Part 1: {:?} ({})", part1, time1);
    println!("Part 2: {:?} ({})", part2, time2);
}

struct Time {
    duration: Duration,
}

impl From<Duration> for Time {
    fn from(duration: Duration) -> Self {
        Self { duration }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.duration.as_micros() < 1000 {
            write!(f, "{}µs", self.duration.as_micros())
        } else if self.duration.as_millis() < 1000 {
            write!(f, "{}ms", self.duration.as_millis())
        } else {
            write!(f, "{}s", self.duration.as_secs())
        }
    }
}
