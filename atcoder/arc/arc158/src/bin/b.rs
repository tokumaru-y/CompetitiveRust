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
        X: [f64; N]
    }
    let mut v = vec![];
    for x in X.into_iter() {
        v.push((1 as f64 / x, x));
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut candidate = vec![];
    for i in 0..N {
        if i < 3 || i + 3 >= N {
            candidate.push(v[i].1);
        }
    }
    let mut ans = vec![];
    let l = candidate.len();
    for i in 0..l {
        for j in i + 1..l {
            for k in j + 1..l {
                ans.push(
                    (candidate[i] + candidate[j] + candidate[k])
                        / (candidate[i] * candidate[j] * candidate[k]),
                );
            }
        }
    }
    ans.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:.12}", ans[0]);
    println!("{:.12}", ans[ans.len() - 1]);
}
