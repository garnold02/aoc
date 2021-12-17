fn main() {
    stdsol::solve(2021, 12, part1, part2);
}

pub fn part1() -> i32 {
    find_paths(&parse(), false).len() as i32
}

pub fn part2() -> i32 {
    find_paths(&parse(), true).len() as i32
}

type Node = &'static str;
type Path = Vec<Node>;

struct Link {
    from: Node,
    to: Node,
}

fn parse() -> Vec<Link> {
    include_str!("input")
        .split_terminator('\n')
        .map(|line| {
            let mut split = line.split('-');
            let a = split.next().unwrap();
            let b = split.next().unwrap();
            [Link { from: a, to: b }, Link { from: b, to: a }]
        })
        .flatten()
        .collect()
}

trait Cave {
    fn is_small(&self) -> bool;
}

impl Cave for &str {
    fn is_small(&self) -> bool {
        self.chars().next().unwrap().is_lowercase()
    }
}

fn find_paths(links: &[Link], visit_twice: bool) -> Vec<Path> {
    fork(links, vec!["start"], visit_twice)
}

fn fork(links: &[Link], path: Path, visit_twice: bool) -> Vec<Path> {
    let last = *path.last().unwrap();
    let mut paths = Vec::new();

    for link in links.iter().filter(|l| l.from == last && l.to != "start") {
        let mut visit = true;
        let mut still_visit_twice = visit_twice;

        if link.to.is_small() && path.contains(&link.to) {
            if visit_twice {
                still_visit_twice = false;
            } else {
                visit = false;
            }
        }

        if visit {
            let mut new_path = path.clone();
            new_path.push(link.to);

            if link.to == "end" {
                paths.push(new_path);
            } else {
                let mut new_paths = fork(links, new_path, still_visit_twice);
                paths.append(&mut new_paths);
            }
        }
    }

    paths
}
