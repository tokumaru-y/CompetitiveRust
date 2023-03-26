#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
use std::{
    fmt::Debug,
    io::{stdout, Write},
    mem::swap,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}

const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

fn act(grid: &mut Vec<Vec<usize>>, h: usize, w: usize, num: usize) {
    grid[h][w] = num;
}

#[allow(non_snake_case)]
fn main() {
    input! {
        H: isize,
        W: isize,
        N: usize,
        mut A: [usize; N]
    }
    let mut idx = 0;
    let mut grid = vec![vec![0; W as usize]; H as usize];
    let mut is_reversed = false;
    for h in 0..H {
        if is_reversed {
            for w in (0..W).rev() {
                act(&mut grid, h as usize, w as usize, idx + 1);
                A[idx] -= 1;
                if A[idx] > 0 && w == 0 {
                    is_reversed = !is_reversed;
                    break;
                }

                if A[idx] == 0 {
                    idx += 1;
                }
            }
        } else {
            for w in 0..W {
                act(&mut grid, h as usize, w as usize, idx + 1);
                A[idx] -= 1;
                if A[idx] > 0 && w == W - 1 {
                    is_reversed = !is_reversed;
                    break;
                }
                if A[idx] == 0 {
                    idx += 1;
                }
            }
        }
    }

    for h in 0..H {
        println!("{}", grid[h as usize].iter().join(" "));
    }
}
