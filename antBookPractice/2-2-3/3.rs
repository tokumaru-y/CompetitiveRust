// https://atcoder.jp/contests/abc009/tasks/abc009_3
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
const FIRST_VALUE: usize = 10_000_000;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: Chars
    }
    let mut ans = S.clone();
    for i in 0..N {
        let mut ind = i;
        for j in i + 1..N {
            if ans[ind] > ans[j] {
                let mut tmp_copy = ans.clone();
                tmp_copy.swap(i, j);
                let mut diff_cnt = 0;
                for t in 0..N {
                    if tmp_copy[t] != S[t] {
                        diff_cnt += 1;
                    }
                }
                if diff_cnt <= K {
                    ind = j;
                }
            }
        }
        ans.swap(ind, i);
    }
    println!("{}", ans.iter().collect::<String>());
}
