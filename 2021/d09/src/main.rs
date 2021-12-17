fn main() {
    stdsol::solve(2021, 9, part1, part2);
}

const INPUT: &str = include_str!("input");

type Grid = [[u8; 100]; 100];

pub fn part1() -> i32 {
    let grid = parse();
    low_points(&grid)
        .iter()
        .copied()
        .map(|(x, y)| grid[y][x] as i32 + 1)
        .sum()
}

pub fn part2() -> i32 {
    let grid = parse();
    let mut flood_state = [[false; 100]; 100];

    let mut basins = low_points(&grid)
        .iter()
        .map(|lp| flood(&grid, lp.0, lp.1, &mut flood_state))
        .collect::<Vec<_>>();

    basins.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    let mut result = 1;

    for basin in &basins[0..3] {
        result *= basin;
    }

    result
}

#[allow(clippy::char_lit_as_u8)]
fn parse() -> Grid {
    INPUT
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| c as u8 - '0' as u8)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn low_points(grid: &Grid) -> Vec<(usize, usize)> {
    let mut vec = Vec::new();

    for x in 0..100 {
        for y in 0..100 {
            let neighbors = [
                grid_get(grid, x, y - 1).copied(),
                grid_get(grid, x, y + 1).copied(),
                grid_get(grid, x - 1, y).copied(),
                grid_get(grid, x + 1, y).copied(),
            ];

            let current = grid[y as usize][x as usize];
            if neighbors.iter().flatten().filter(|n| current < **n).count()
                == neighbors.iter().flatten().count()
            {
                vec.push((x as usize, y as usize));
            }
        }
    }

    vec
}

fn grid_get(grid: &Grid, x: i32, y: i32) -> Option<&u8> {
    if let Some(row) = grid.get(y as usize) {
        if let Some(val) = row.get(x as usize) {
            return Some(val);
        }
    }
    None
}

fn flood(grid: &Grid, x: usize, y: usize, state: &mut [[bool; 100]; 100]) -> i32 {
    let current = grid[y][x] as i32;
    if current == 9 || state[y][x] {
        return 0;
    }

    state[y][x] = true;

    let mut sum = 1;
    let (x, y) = (x as i32, y as i32);
    for (x, y) in [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
        .iter()
        .filter(|(x, y)| (0..100).contains(x) && (0..100).contains(y))
        .map(|(x, y)| (*x as usize, *y as usize))
    {
        sum += flood(grid, x, y, state);
    }

    sum
}
