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
        A: [isize; N]
    }
    let mut sum_v = vec![0isize; N + 1];
    let mut ans = 0;
    for i in 0..N {
        sum_v[i + 1] = sum_v[i] + A[i];
    }
    let mut table: HashMap<isize, usize> = HashMap::new();

    for i in 0..=N {
        if table.contains_key(&sum_v[i]) {
            ans += table.get(&sum_v[i]).unwrap();
        }
        *table.entry(sum_v[i]).or_insert(0) += 1;
    }

    println!("{}", ans);
}
