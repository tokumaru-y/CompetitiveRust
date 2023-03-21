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
        N: usize,
        mut K: usize,
        AB: [(usize, usize); N]
    }
    let mut table: HashMap<usize, usize> = HashMap::new();
    let mut num_list = vec![];
    for (a, b) in AB.into_iter() {
        if !table.contains_key(&a) {
            num_list.push(a);
        }
        *table.entry(a).or_insert(0) += b;
    }
    num_list.sort();
    for n in num_list.into_iter() {
        let cnt = table.get(&n).unwrap();
        if K <= *cnt {
            println!("{}", n);
            exit(0);
        }
        K -= *cnt;
    }
}
