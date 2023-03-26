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

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        mut Grid: [[usize; W]; H]
    }
    let mut ans = 0;
    let mut ans_move = vec![];
    for h in 0..H {
        for w in 0..W {
            if Grid[h][w] == 0 {
                continue;
            }
            if Grid[h][w] % 2 == 0 {
                ans += 1;
            } else {
                if w == W - 1 && h < H - 1 {
                    Grid[h][w] -= 1;
                    Grid[h + 1][w] += 1;
                    ans_move.push((h + 1, w + 1, h + 2, w + 1));
                } else if w < W - 1 {
                    Grid[h][w] -= 1;
                    Grid[h][w + 1] += 1;
                    ans_move.push((h + 1, w + 1, h + 1, w + 2));
                }

                if Grid[h][w] > 0 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans_move.len());
    for i in 0..(ans_move.len()) {
        let a = ans_move[i];
        println!("{} {} {} {}", a.0, a.1, a.2, a.3);
    }
}
