static INPUT: &str = include_str!("input.txt");

fn main() {
    let mut monkeys1 = parse_monkeys();
    let mut monkeys2 = monkeys1.clone();
    solve(&mut monkeys1, 20, 3);
    solve(&mut monkeys2, 10000, 1);
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    test: i64,
    if_true: usize,
    if_false: usize,
    inspections: i64,
}

#[derive(Clone, Copy)]
enum Op {
    Add(OpNum),
    Mul(OpNum),
}

impl Op {
    fn get(self, old: i64) -> i64 {
        match self {
            Op::Add(num) => old + num.get(old),
            Op::Mul(num) => old * num.get(old),
        }
    }
}

#[derive(Clone, Copy)]
enum OpNum {
    Old,
    Num(i64),
}

impl OpNum {
    fn get(self, old: i64) -> i64 {
        match self {
            OpNum::Old => old,
            OpNum::Num(num) => num,
        }
    }
}

fn solve(monkeys: &mut [Monkey], rounds: i64, div: i64) {
    let mut rem = 1;

    for monkey in monkeys.iter() {
        rem *= monkey.test;
    }

    for _ in 0..rounds {
        simulate(monkeys, div, rem);
    }

    monkeys.sort_unstable_by(|a, b| b.inspections.cmp(&a.inspections));
    let monkey_business = monkeys[0].inspections * monkeys[1].inspections;
    println!("{}", monkey_business);
}

fn simulate(monkeys: &mut [Monkey], div: i64, rem: i64) {
    for i in 0..monkeys.len() {
        let current = &mut monkeys[i];
        current.inspections += current.items.len() as i64;

        let items: Vec<_> = current.items.drain(..).collect();
        let op = current.op;
        let test = current.test;
        let if_true = current.if_true;
        let if_false = current.if_false;

        for item in items {
            let item = (op.get(item) / div) % rem;
            if item % test == 0 {
                monkeys[if_true].items.push(item);
            } else {
                monkeys[if_false].items.push(item);
            }
        }
    }
}

fn parse_monkeys() -> Vec<Monkey> {
    let mut lines = INPUT.lines();
    let mut monkeys = Vec::new();

    while lines.next().is_some() {
        let items = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|v| v.parse().unwrap())
            .collect();

        let op_line = lines.next().unwrap();
        let op_kind = op_line.chars().nth(23).unwrap();
        let op_num = op_line.split(op_kind).nth(1).unwrap().trim();

        let op_num = if op_num == "old" {
            OpNum::Old
        } else {
            OpNum::Num(op_num.parse().unwrap())
        };

        let op = match op_kind {
            '+' => Op::Add(op_num),
            '*' => Op::Mul(op_num),
            _ => panic!(),
        };

        let test = lines
            .next()
            .unwrap()
            .split("by ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let if_true = lines
            .next()
            .unwrap()
            .split("monkey ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let if_false = lines
            .next()
            .unwrap()
            .split("monkey ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        monkeys.push(Monkey {
            items,
            op,
            test,
            if_true,
            if_false,
            inspections: 0,
        });

        if lines.next().is_none() {
            break;
        }
    }

    monkeys
}
