fn main() {
    stdsol::solve(2021, 18, part1, part2);
}

fn part1() -> i32 {
    let mut nodes = vec![];
    parse("[0,1]", None, &mut nodes);
    let right = right_to(&nodes, 1);
    println!("{:?}", nodes);
    0
}

fn part2() -> i32 {
    0
}

#[derive(Debug)]
struct Node {
    up: Option<usize>,
    kind: NodeKind,
}

#[derive(Debug)]
enum NodeKind {
    Lit(i32),
    Pair { left: usize, right: usize },
}

impl NodeKind {
    fn pair(&self) -> (usize, usize) {
        if let Self::Pair { left, right } = self {
            (*left, *right)
        } else {
            panic!("NodeKind was not Pair");
        }
    }

    fn lit(&mut self) -> &mut i32 {
        if let Self::Lit(lit) = self {
            lit
        } else {
            panic!("NodeKind was not Lit");
        }
    }

    fn is_lit(&self) -> bool {
        if let Self::Lit(_) = self {
            true
        } else {
            false
        }
    }
}

fn parse(input: &str, up: Option<usize>, nodes: &mut Vec<Node>) {
    if input.starts_with('[') {
        let input = input.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
        let delim = {
            let mut depth = 0;
            let mut delim = 0;
            for (i, c) in input.chars().enumerate() {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' => {
                        if depth == 0 {
                            delim = i;
                        }
                    }
                    _ => {}
                }
            }
            delim
        };
        let left = &input[0..delim];
        let right = &input[delim + 1..];

        let node_id = nodes.len();
        nodes.push(Node {
            up,
            kind: NodeKind::Pair { left: 0, right: 0 },
        });

        let left_id = nodes.len();
        parse(left, Some(node_id), nodes);

        let right_id = nodes.len();
        parse(right, Some(node_id), nodes);

        if let NodeKind::Pair { left, right } = &mut nodes[node_id].kind {
            *left = left_id;
            *right = right_id;
        }
    } else {
        nodes.push(Node {
            up,
            kind: NodeKind::Lit(input.parse().unwrap()),
        });
    }
}

fn explode_candidates(nodes: &[Node], current: usize, depth: usize) -> Vec<usize> {
    let mut candidates = vec![];

    if let NodeKind::Pair { left, right } = &nodes[current].kind {
        if depth == 4 {
            candidates.push(current);
        } else {
            candidates.append(&mut explode_candidates(nodes, *left, depth + 1));
            candidates.append(&mut explode_candidates(nodes, *right, depth + 1));
        }
    }

    candidates
}

fn split_candidates(nodes: &[Node], current: usize) -> Vec<usize> {
    let mut candidates = vec![];

    match &nodes[current].kind {
        NodeKind::Lit(lit) => {
            if *lit > 9 {
                candidates.push(current)
            }
        }
        NodeKind::Pair { left, right } => {
            candidates.append(&mut split_candidates(nodes, *left));
            candidates.append(&mut split_candidates(nodes, *right));
        }
    }

    candidates
}

fn add(mut lhs: Vec<Node>, mut rhs: Vec<Node>) -> Vec<Node> {
    let mut vec = Vec::with_capacity(lhs.len() + rhs.len() + 1);
    vec.push(Node {
        up: None,
        kind: NodeKind::Pair {
            left: 1,
            right: 1 + lhs.len(),
        },
    });

    lhs[0].up = Some(0);
    rhs[0].up = Some(0);

    vec.append(&mut lhs);
    vec.append(&mut rhs);

    vec
}

fn left_to(nodes: &[Node], node: usize) -> Option<usize> {
    let mut current = node;
    let mut found = false;
    loop {
        if found {
            match &nodes[current].kind {
                NodeKind::Lit(_) => {
                    return Some(current);
                }
                NodeKind::Pair { right, .. } => {
                    current = *right;
                }
            }
        } else if let Some(up) = nodes[current].up {
            let (left, right) = nodes[up].kind.pair();
            if right == current {
                current = left;
                found = true;
            } else {
                current = up;
            }
        } else {
            return None;
        }
    }
}

fn right_to(nodes: &[Node], node: usize) -> Option<usize> {
    let mut current = node;
    let mut found = false;
    loop {
        if found {
            match &nodes[current].kind {
                NodeKind::Lit(_) => {
                    return Some(current);
                }
                NodeKind::Pair { left, .. } => {
                    current = *left;
                }
            }
        } else if let Some(up) = nodes[current].up {
            let (left, right) = nodes[up].kind.pair();
            if left == current {
                current = right;
                found = true;
            } else {
                current = up;
            }
        } else {
            return None;
        }
    }
}

fn reduced(mut vec: Vec<Node>) -> Vec<Node> {
    loop {
        let mut ops = 0;
        let explode = explode_candidates(&vec, 0, 0).get(0).copied();
        if let Some(explode) = explode {
            let up = vec[explode].up;
            let left = left_to(&vec, explode);
            let right = right_to(&vec, explode);
            vec[explode] = Node {
                up,
                kind: NodeKind::Lit(0),
            };

            if let Some(left) = left {
                *vec[left].kind.lit() +=
            }

            ops += 1;
        }

        let split = split_candidates(&vec, 0).get(0).copied();
        if let Some(split) = split {
            ops += 1;
        }

        if ops == 0 {
            break;
        }
    }

    vec
}
