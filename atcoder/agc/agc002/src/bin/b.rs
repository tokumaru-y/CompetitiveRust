#[allow(unused_imports)]
use itertools::Itertools;
use petgraph::algo::is_cyclic_undirected;
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
        N: usize,
        M: usize,
        XY: [(Usize1, Usize1); M]
    }
    let mut is_acceptable = vec![false; N];
    is_acceptable[0] = true;
    let mut cnt_vec = vec![1; N];
    for (x, y) in XY.into_iter() {
        cnt_vec[x] -= 1;
        cnt_vec[y] += 1;
        is_acceptable[y] |= is_acceptable[x];
        if cnt_vec[x] == 0 {
            is_acceptable[x] = false;
        }
    }

    println!("{}", is_acceptable.into_iter().filter(|x| *x).count());
}
