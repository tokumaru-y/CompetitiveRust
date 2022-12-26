// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_e
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use std::fmt::Debug;
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        Grid: [[usize; W]; H],
    }
    let mut ans = 0;

    for bit in 0..(1 << H) {
        let mut tmp_grid = Grid.clone();

        for i in 0..H {
            if bit & (1 << i as usize) > 0 {
                for w in 0..W {
                    tmp_grid[i][w] += 1;
                }
            }
        }

        let mut tmp_sum = 0;
        for w in 0..W {
            let mut w_sum = 0;
            for h in 0..H {
                w_sum += tmp_grid[h][w] % 2;
            }
            tmp_sum += max(w_sum, H - w_sum);
        }

        ans = max(ans, tmp_sum);
    }

    println!("{}", ans);
}
