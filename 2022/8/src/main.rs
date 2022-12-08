#![allow(clippy::explicit_counter_loop)]

use crate::matrix::Matrix;

mod matrix;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let matrix = Matrix::new(INPUT);

    let mut sum = 0;
    let mut scenic = 0;

    for x in 0..matrix.width() {
        for y in 0..matrix.height() {
            if visible(&matrix, x, y) {
                sum += 1;
            }

            let score = scenic_score(&matrix, x, y);
            if score > scenic {
                scenic = score;
            }
        }
    }

    println!("{}", sum);
    println!("{}", scenic);
}

fn visible(matrix: &Matrix, x: usize, y: usize) -> bool {
    let tree = matrix.get(x, y);

    let mut left = true;
    for i in 0..x {
        if matrix.get(i, y) >= tree {
            left = false;
            break;
        }
    }

    if left {
        return true;
    }

    let mut right = true;
    for i in x + 1..matrix.width() {
        if matrix.get(i, y) >= tree {
            right = false;
            break;
        }
    }

    if right {
        return true;
    }

    let mut up = true;
    for i in 0..y {
        if matrix.get(x, i) >= tree {
            up = false;
        }
    }

    if up {
        return true;
    }

    let mut down = true;
    for i in y + 1..matrix.height() {
        if matrix.get(x, i) >= tree {
            down = false;
            break;
        }
    }

    if down {
        return true;
    }

    false
}

fn scenic_score(matrix: &Matrix, x: usize, y: usize) -> i32 {
    let tree = matrix.get(x, y);

    let mut left = 0;
    for i in (0..x).rev() {
        left += 1;
        if matrix.get(i, y) >= tree {
            break;
        }
    }

    let mut right = 0;
    for i in x + 1..matrix.width() {
        right += 1;
        if matrix.get(i, y) >= tree {
            break;
        }
    }

    let mut up = 0;
    for i in (0..y).rev() {
        up += 1;
        if matrix.get(x, i) >= tree {
            break;
        }
    }

    let mut down = 0;
    for i in y + 1..matrix.height() {
        down += 1;
        if matrix.get(x, i) >= tree {
            break;
        }
    }

    left * right * up * down
}
