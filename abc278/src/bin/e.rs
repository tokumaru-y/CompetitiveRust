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
        A: [[usize; W]; H],
    }
    let mut ans = vec![];
    let mut cnt_table = vec![0; N + 1];
    let mut all_kinds = 0;

    for h in 0..H {
        for w in 0..W {
            if cnt_table[A[h][w]] == 0 {
                all_kinds += 1;
            }
            cnt_table[A[h][w]] += 1;
        }
    }

    for h in 0..=(H - range_h) {
        for w in 0..=(W - range_w) {
            if w == 0 {
                for x in 0..range_h {
                    for y in 0..range_w {
                        let target = A[h + x][w + y];
                        cnt_table[target] -= 1;
                        if cnt_table[target] == 0 {
                            all_kinds -= 1;
                        }
                    }
                }
                ans.push(vec![all_kinds]);
            } else {
                for x in h..=(h + range_h - 1) {
                    let target = A[x][w - 1];
                    if cnt_table[target] == 0 {
                        all_kinds += 1;
                    }
                    cnt_table[target] += 1;

                    let target = A[x][w + range_w - 1];
                    cnt_table[target] -= 1;
                    if cnt_table[target] == 0 {
                        all_kinds -= 1;
                    }
                }

                ans[h].push(all_kinds);
            }
            if w == W - range_w {
                for x in 0..range_h {
                    for y in 0..range_w {
                        let target = A[h + x][w + y];
                        if cnt_table[target] == 0 {
                            all_kinds += 1;
                        }
                        cnt_table[target] += 1;
                    }
                }
            }
        }
    }

    for h in 0..ans.len() {
        println!("{}", ans[h].iter().join(" "));
    }
}
