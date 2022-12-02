use std::collections::HashSet;

fn main() {
    stdsol::solve(2021, 13, part1, part2);
}

pub fn part1() -> i32 {
    let (paper, folds) = parse();
    paper.fold(&folds[0]).len() as i32
}

pub fn part2() -> i32 {
    let (paper, folds) = parse();
    let mut paper = paper;
    for fold in folds {
        paper = paper.fold(&fold);
    }

    let (buf, width, height) = paper.visualize();

    image::save_buffer_with_format(
        "out/paper.png",
        &buf,
        width,
        height,
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )
    .unwrap();

    0
}

type Point = (i32, i32);
type Paper = HashSet<Point>;

enum Fold {
    AlongX(i32),
    AlongY(i32),
}

fn parse() -> (Paper, Vec<Fold>) {
    let mut paper = HashSet::new();
    let mut folds = Vec::new();

    let mut switch = false;
    for line in include_str!("input").trim().lines() {
        if line.is_empty() {
            switch = true;
            continue;
        }

        if switch {
            let mut split = line.split(' ');
            split.nth(1).unwrap();

            let mut split = split.next().unwrap().split('=');
            let axis = split.next().unwrap();
            let val = split.next().unwrap().parse().unwrap();

            folds.push(match axis {
                "x" => Fold::AlongX(val),
                "y" => Fold::AlongY(val),
                _ => panic!(),
            });
        } else {
            let mut split = line.split(',');
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            paper.insert((x, y));
        }
    }

    (paper, folds)
}

trait Foldable {
    fn fold(self, how: &Fold) -> Self;
}

trait Visualize {
    fn visualize(&self) -> (Vec<u8>, u32, u32);
}

impl Foldable for Paper {
    fn fold(self, how: &Fold) -> Self {
        match how {
            Fold::AlongX(x) => self
                .into_iter()
                .map(|p| if p.0 > *x { (2 * *x - p.0, p.1) } else { p })
                .collect(),

            Fold::AlongY(y) => self
                .into_iter()
                .map(|p| if p.1 > *y { (p.0, 2 * *y - p.1) } else { p })
                .collect(),
        }
    }
}

impl Visualize for Paper {
    fn visualize(&self) -> (Vec<u8>, u32, u32) {
        let width = *self.iter().map(|(x, _)| x).max().unwrap() + 1;
        let height = *self.iter().map(|(_, y)| y).max().unwrap() + 1;

        (
            (0..height)
                .map(|y| {
                    (0..width)
                        .map(move |x| {
                            if self.contains(&(x, y)) {
                                [255; 4]
                            } else {
                                [0, 0, 0, 255]
                            }
                        })
                        .flatten()
                })
                .flatten()
                .collect(),
            width as u32,
            height as u32,
        )
    }
}
