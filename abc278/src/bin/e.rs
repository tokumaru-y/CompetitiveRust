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
        N: usize,
        range_h: usize,
        range_w: usize,
        A: [[usize; W]; H]
    }
    let mut minh = vec![10usize.pow(6); N + 1];
    let mut minw = vec![10usize.pow(6); N + 1];
    let mut maxh = vec![0; N + 1];
    let mut maxw = vec![0; N + 1];
    let mut cnt_table = vec![0; N + 1];
    let mut all_kinds = 0;
    let mut ans = vec![vec![0; W - range_w + 1]; H - range_h + 1];

    for h in 0..H {
        for w in 0..W {
            let target = A[h][w];
            minh[target] = min(minh[target], h);
            minw[target] = min(minw[target], w);
            maxh[target] = max(maxh[target], h);
            maxw[target] = max(maxw[target], w);
            if cnt_table[target] == 0 {
                all_kinds += 1;
            }
            cnt_table[target] += 1;
        }
    }

    for h in 0..(H - range_h + 1) {
        for w in 0..(W - range_w + 1) {
            let mut dif = 0;
            let mut tmp_table = cnt_table.clone();

            for x in 0..range_h {
                for y in 0..range_w {
                    let target = A[h + x][w + y];

                    if (h <= minh[target]
                        && maxh[target] < h + range_h
                        && w <= minw[target]
                        && maxw[target] < w + range_w)
                    {
                        tmp_table[target] -= 1;
                        if tmp_table[target] == 0 {
                            dif += 1;
                        }
                    }
                }
            }
            ans[h][w] = all_kinds - dif;
        }
    }

    for h in 0..(H - range_h + 1) {
        println!("{}", ans[h].iter().join(" "));
    }
}
