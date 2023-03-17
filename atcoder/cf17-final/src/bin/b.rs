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
        S: Chars
    }
    let mut table: HashMap<char, usize> = HashMap::new();
    table.insert('a', 0);
    table.insert('b', 0);
    table.insert('c', 0);
    for i in 0..(S.len()) {
        *table.entry(S[i]).or_default() += 1;
    }

    let mut values: Vec<usize> = table.values().cloned().collect();
    values.sort();
    let is_ok = values.iter().max().unwrap() - values.iter().min().unwrap() <= 1;

    let ans = if is_ok { "YES" } else { "NO" };
    println!("{}", ans);
}
