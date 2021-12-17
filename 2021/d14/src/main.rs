use std::collections::HashMap;

fn main() {
    stdsol::solve(2021, 14, || iterate(10), || iterate(40));
}

fn iterate(iter: usize) -> u64 {
    let (initial, rules) = parse();

    let mut pairs = rules
        .iter()
        .map(|(k, _)| (k.to_string(), 0))
        .collect::<HashMap<_, u64>>();

    for w in initial.chars().collect::<Vec<_>>().windows(2) {
        let str = w.iter().collect::<String>();
        *pairs.get_mut(str.as_str()).unwrap() += 1;
    }

    for _ in 0..iter {
        let mut new_pairs = HashMap::new();
        for (pair, n) in &pairs {
            let inner = *rules.get(pair.as_str()).unwrap();
            let new_pair_a = [pair.chars().next().unwrap(), inner]
                .iter()
                .collect::<String>();

            let new_pair_b = [inner, pair.chars().nth(1).unwrap()]
                .iter()
                .collect::<String>();

            *new_pairs.entry(new_pair_a).or_insert(0) += n;
            *new_pairs.entry(new_pair_b).or_insert(0) += n;
        }

        pairs = new_pairs;
    }

    let mut counts = HashMap::new();
    for (pair, n) in pairs {
        *counts.entry(pair.chars().next().unwrap()).or_insert(0) += n;
        *counts.entry(pair.chars().nth(1).unwrap()).or_insert(0) += n;
    }

    let counts = counts
        .into_iter()
        .map(|(k, v)| (k, if v % 2 == 0 { v / 2 } else { (v + 1) / 2 }))
        .collect::<HashMap<_, _>>();

    let least = *counts
        .iter()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap()
        .1;

    let most = *counts
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap()
        .1;

    most - least
}

fn parse() -> (&'static str, HashMap<&'static str, char>) {
    let mut input = include_str!("input").split_terminator('\n');
    let initial = input.next().unwrap();
    input.next().unwrap();
    let rules = input
        .map(|line| {
            let mut split = line.split(" -> ");
            (
                split.next().unwrap(),
                split.next().unwrap().chars().next().unwrap(),
            )
        })
        .collect();

    (initial, rules)
}
