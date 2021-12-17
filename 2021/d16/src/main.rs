fn main() {
    stdsol::solve(2021, 16, part1, part2);
}

pub fn part1() -> i32 {
    let packet = Packet::parse();
    packet.version_sum()
}

pub fn part2() -> u64 {
    let packet = Packet::parse();
    packet.evaluate()
}

struct Packet {
    version: u8,
    kind: PacketKind,
}

enum PacketKind {
    Operator(Operator),
    Literal(u64),
}

struct Operator {
    kind: OperatorKind,
    subs: Vec<Packet>,
}

impl Operator {
    fn operate(&self) -> u64 {
        let mut iter = self.subs.iter().map(|p| p.evaluate());
        match self.kind {
            OperatorKind::Sum => iter.sum(),
            OperatorKind::Product => iter.product(),
            OperatorKind::Minimum => iter.min().unwrap(),
            OperatorKind::Maximum => iter.max().unwrap(),
            OperatorKind::Gt => (iter.next() > iter.next()) as u64,
            OperatorKind::Lt => (iter.next() < iter.next()) as u64,
            OperatorKind::Eq => (iter.next() == iter.next()) as u64,
        }
    }
}

enum OperatorKind {
    Sum,
    Product,
    Minimum,
    Maximum,
    Gt,
    Lt,
    Eq,
}

enum LengthTypeId {
    Size(usize),
    Count(usize),
}

impl Packet {
    fn parse() -> Self {
        let hex = hex::decode(include_str!("input").trim()).unwrap();
        let bits = hex.iter().map(bits).flatten().collect::<Vec<_>>();
        let mut ptr = 0;
        packet(&bits, &mut ptr)
    }

    fn version_sum(&self) -> i32 {
        let mut sum = self.version as i32;
        if let PacketKind::Operator(op) = &self.kind {
            sum += op.subs.iter().map(|p| p.version_sum()).sum::<i32>();
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        match &self.kind {
            PacketKind::Operator(op) => op.operate(),
            PacketKind::Literal(lit) => *lit,
        }
    }
}

fn bits(b: &u8) -> [bool; 8] {
    let mut res = [false; 8];
    for i in 0..8 {
        res[i] = ((b >> (7 - i)) & 1) == 1;
    }
    res
}

fn packet(bits: &[bool], ptr: &mut usize) -> Packet {
    let version = num(bits, ptr, 3) as u8;
    let type_id = num(bits, ptr, 3) as u8;

    let kind = match type_id {
        4 => PacketKind::Literal(literal(bits, ptr)),
        i => PacketKind::Operator(Operator {
            kind: op_kind(i),
            subs: operator(bits, ptr),
        }),
    };

    Packet { version, kind }
}

fn literal(bits: &[bool], ptr: &mut usize) -> u64 {
    let mut new_bits = Vec::new();
    loop {
        let cont = bits[*ptr];
        *ptr += 1;

        let bits = &bits[*ptr..*ptr + 4];
        *ptr += 4;

        new_bits.append(&mut bits.to_vec());

        if !cont {
            break;
        }
    }

    num(&new_bits, &mut 0, new_bits.len())
}

fn operator(bits: &[bool], ptr: &mut usize) -> Vec<Packet> {
    *ptr += 1;
    let length = match bits[*ptr - 1] {
        false => LengthTypeId::Size(num(bits, ptr, 15) as usize),
        true => LengthTypeId::Count(num(bits, ptr, 11) as usize),
    };

    let subs = match length {
        LengthTypeId::Size(size) => {
            let mut subs = Vec::new();
            let lim = *ptr + size;
            while *ptr < lim {
                subs.push(packet(bits, ptr));
            }
            subs
        }
        LengthTypeId::Count(count) => {
            let mut subs = Vec::new();
            for _ in 0..count {
                subs.push(packet(bits, ptr));
            }
            subs
        }
    };

    subs
}

fn op_kind(kind: u8) -> OperatorKind {
    match kind {
        0 => OperatorKind::Sum,
        1 => OperatorKind::Product,
        2 => OperatorKind::Minimum,
        3 => OperatorKind::Maximum,
        5 => OperatorKind::Gt,
        6 => OperatorKind::Lt,
        7 => OperatorKind::Eq,
        _ => unreachable!(),
    }
}

fn num(bits: &[bool], ptr: &mut usize, len: usize) -> u64 {
    let bits = &bits[*ptr..*ptr + len];
    *ptr += len;

    let mut res = 0;
    for (i, b) in bits.iter().rev().enumerate() {
        if *b {
            res |= 1 << i;
        }
    }
    res
}

fn mul8(x: u8) -> u8 {
    let x = x as i32;
    ((x + 7) & (-8)) as u8
}
