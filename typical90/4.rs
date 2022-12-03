// https://atcoder.jp/contests/typical90/tasks/typical90_d
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
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        grid: [[usize; W]; H]
    }
    let mut ans = vec![vec![0usize; W]; H];
    let mut w_sum = vec![0; W];
    for h in 0..H {
        let sum = grid[h].iter().sum::<usize>();
        for w in 0..W {
            ans[h][w] = sum;
            w_sum[w] += grid[h][w];
        }
    }
    for h in 0..H {
        for w in 0..W {
            ans[h][w] += w_sum[w];
            ans[h][w] -= grid[h][w];
        }
    }
    for i in 0..H {
        println!("{}", ans[i].iter().join(" "));
    }
}
