// https://atcoder.jp/contests/abc038/tasks/abc038_d
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
        mut WH: [(isize, isize); N]
    }
    let mut lis = vec![0; N];
    WH.sort_by(|a, b| (a.1).partial_cmp(&(b.1)).unwrap());
    WH.sort_by(|a, b| (-a.0).partial_cmp(&(-b.0)).unwrap());
    for (w, h) in WH.into_iter() {
        let sh = h * -1;
        let r = lis.binary_search(&sh);
        let ind = if r.is_ok() {
            r.unwrap()
        } else {
            r.unwrap_err()
        };
        lis[ind] = sh;
    }
    for i in (0..N).rev() {
        if lis[i] != 0 {
            println!("{}", i + 1);
            exit(0);
        }
    }
}
