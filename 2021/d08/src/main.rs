fn main() {
    stdsol::solve(2021, 8, part1, part2);
}

const INPUT: &str = include_str!("input");

pub fn part1() -> usize {
    let entries = parse();
    entries
        .iter()
        .map(|entry| {
            entry
                .outputs
                .iter()
                .filter(|out| [2, 4, 3, 7].contains(&out.len()))
                .count()
        })
        .sum()
}

pub fn part2() -> i32 {
    let entries = parse();
    entries.iter().map(resolve).sum()
}

struct Entry {
    patterns: [Vec<char>; 10],
    outputs: [Vec<char>; 4],
}

fn resolve(entry: &Entry) -> i32 {
    let dig_1 = entry.patterns.iter().find(|pat| pat.len() == 2).unwrap();
    let dig_4 = entry.patterns.iter().find(|pat| pat.len() == 4).unwrap();
    let dig_7 = entry.patterns.iter().find(|pat| pat.len() == 3).unwrap();
    let dig_8 = entry.patterns.iter().find(|pat| pat.len() == 7).unwrap();

    // The segment which appears in seven, but not in one must be the top segment
    let seg_top = dig_7.iter().find(|s| !dig_1.contains(s)).unwrap();

    // The segments that appear in four, but not in one
    let seg_ul_mid = dig_4
        .iter()
        .filter(|s| !dig_1.contains(s))
        .collect::<Vec<_>>();
    assert_eq!(seg_ul_mid.len(), 2);

    let either_069 = entry
        .patterns
        .iter()
        .filter(|pat| pat.len() == 6)
        .collect::<Vec<_>>();

    assert_eq!(either_069.len(), 3);

    // The only pattern in either_069 that doesn't fully contain one MUST be six
    let dig_6 = either_069
        .iter()
        .find(|pat| dig_1.iter().filter(|s| pat.contains(s)).count() != 2)
        .unwrap();

    let either_09 = {
        let mut vec = either_069.clone();
        vec.retain(|pat| pat != dig_6);
        vec
    };

    // The only pattern in either_09 that fully contains seg_ul_mid MUST be nine, the other
    // MUST be zero

    let dig_9 = either_09
        .iter()
        .find(|pat| seg_ul_mid.iter().filter(|s| pat.contains(s)).count() == 2)
        .unwrap();

    let dig_0 = either_09.iter().find(|pat| *pat != dig_9).unwrap();

    // The one segment in seg_ul_mid that is in dig_0 MUST be the segment ul
    let seg_ul = seg_ul_mid.iter().find(|s| dig_0.contains(s)).unwrap();

    // The other one is self explanatory
    let seg_mid = seg_ul_mid.iter().find(|s| *s != seg_ul).unwrap();

    // The segment that is in both six and one MUST be seg_br
    let seg_br = dig_1.iter().find(|s| dig_6.contains(s)).unwrap();

    // Using the inverse, seg_ur can be found, too
    let seg_ur = dig_1.iter().find(|s| *s != seg_br).unwrap();

    // The segment that is in 8, but not in nine MUST be seg_bl
    let seg_bl = dig_8.iter().find(|s| !dig_9.contains(s)).unwrap();

    let temp = [seg_top, seg_ul, seg_ur, seg_mid, seg_bl, seg_br];

    // The remaining segment MUST be seg_bot
    let seg_bot = dig_8.iter().find(|s| !temp.contains(s)).unwrap();

    let segs = [seg_top, seg_ul, seg_ur, seg_mid, seg_bl, seg_br, seg_bot].map(|c| *c);

    entry
        .outputs
        .iter()
        .map(|out| sigs_to_num(out, &segs).unwrap())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

fn sigs_to_num(sigs: &[char], segs: &[char; 7]) -> Option<char> {
    let indices = segs.map(|seg| sigs.contains(&seg));

    match indices {
        [true, true, true, false, true, true, true] => Some('0'),
        [false, false, true, false, false, true, false] => Some('1'),
        [true, false, true, true, true, false, true] => Some('2'),
        [true, false, true, true, false, true, true] => Some('3'),
        [false, true, true, true, false, true, false] => Some('4'),
        [true, true, false, true, false, true, true] => Some('5'),
        [true, true, false, true, true, true, true] => Some('6'),
        [true, false, true, false, false, true, false] => Some('7'),
        [true, true, true, true, true, true, true] => Some('8'),
        [true, true, true, true, false, true, true] => Some('9'),
        a => panic!("{:?}", a),
    }
}

fn parse() -> Vec<Entry> {
    INPUT
        .split_terminator('\n')
        .map(|line| {
            let mut split = line.split('|');

            let patterns = split
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|pat| pat.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let outputs = split
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|out| out.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            Entry { patterns, outputs }
        })
        .collect()
}
