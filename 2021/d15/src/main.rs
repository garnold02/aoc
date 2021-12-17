use std::collections::HashSet;

fn main() {
    stdsol::solve(2021, 15, part1, part2);
}

pub fn part1() -> i32 {
    let graph = Graph::new();
    let path = dijkstra(&graph);
    path.iter().map(|n| graph.storage[*n] as i32).sum()
}

pub fn part2() -> i32 {
    let graph = Graph::new_repeat();
    let path = dijkstra(&graph);
    path.iter().map(|n| graph.storage[*n] as i32).sum()
}

fn dijkstra(graph: &Graph) -> Vec<usize> {
    let arr_len = graph.width * graph.width;

    let mut rem = (0..arr_len).collect::<HashSet<usize>>();
    let mut dist = vec![i32::MAX; arr_len];
    let mut prev = vec![usize::MAX; arr_len];

    dist[0] = 0;

    while !rem.is_empty() {
        let node = *rem
            .iter()
            .min_by(|u1, u2| dist[**u1].partial_cmp(&dist[**u2]).unwrap())
            .unwrap();

        rem.remove(&node);

        if node == arr_len - 1 {
            break;
        }

        for neighbor in graph.neighbors(node).iter().filter(|n| rem.contains(*n)) {
            let neighbor = *neighbor;
            let alt = dist[node] + graph.storage[neighbor] as i32;
            if alt < dist[neighbor] {
                dist[neighbor] = alt;
                prev[neighbor] = node;
            }
        }
    }

    let mut current = arr_len - 1;
    let mut path = Vec::with_capacity(arr_len);

    while current != 0 {
        path.push(current);
        current = prev[current];
    }

    path.shrink_to_fit();
    path
}

struct Graph {
    pub storage: Vec<u8>,
    pub width: usize,
}

impl Graph {
    fn new() -> Self {
        let mut storage = Vec::with_capacity(100 * 100);
        for line in include_str!("input").split_terminator('\n') {
            for num in line.chars() {
                storage.push(num as u8 - b'0');
            }
        }

        Self {
            storage,
            width: 100,
        }
    }

    fn new_repeat() -> Self {
        let mut storage = vec![0u8; 500 * 500];
        let mut lines = include_str!("input").split_terminator('\n');
        for y in 0..100 {
            let mut line = lines.next().unwrap().chars();
            for x in 0..100 {
                let num = line.next().unwrap() as u8 - b'0';
                for i in 0..5 {
                    for j in 0..5 {
                        let mut num = num + i as u8 + j as u8;
                        while num > 9 {
                            num -= 9;
                        }

                        storage[(y + i * 100) * 500 + (x + j * 100)] = num;
                    }
                }
            }
        }
        Self {
            storage,
            width: 500,
        }
    }

    fn neighbors(&self, node: usize) -> Vec<usize> {
        let mut ret = Vec::new();

        let x = node % self.width;
        let y = node / self.width;

        if y > 0 {
            ret.push((y - 1) * self.width + x);
        }
        if y < self.width - 1 {
            ret.push((y + 1) * self.width + x);
        }
        if x > 0 {
            ret.push(y * self.width + (x - 1));
        }
        if x < self.width - 1 {
            ret.push(y * self.width + (x + 1));
        }

        ret
    }
}
