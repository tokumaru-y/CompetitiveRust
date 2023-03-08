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
        N: isize,
        V: [isize; N]
    }
    let mut table1: HashMap<isize, isize> = HashMap::new();
    let mut table2: HashMap<isize, isize> = HashMap::new();

    for i in 0..N {
        if i % 2 == 0 {
            *table1.entry(V[i as usize]).or_default() += 1;
        } else {
            *table2.entry(V[i as usize]).or_default() += 1;
        }
    }

    let mut v1: Vec<(isize, isize)> = table1.into_iter().collect();
    let mut v2: Vec<(isize, isize)> = table2.into_iter().collect();

    v1.sort_by(|a, b| b.1.cmp(&a.1));
    v2.sort_by(|a, b| b.1.cmp(&a.1));

    let v1_candidate = if v1.len() == 1 {
        v1
    } else {
        vec![v1[0], v1[1]]
    };

    let v2_candidate = if v2.len() == 1 {
        v2
    } else {
        vec![v2[0], v2[1]]
    };
    if v1_candidate.len() == 1 && v2_candidate.len() == 1 && v1_candidate[0].0 != v2_candidate[0].0
    {
        println!("0");
        exit(0);
    }

    let mut ans = std::isize::MAX;
    for i in 0..(v1_candidate.len()) {
        for j in 0..(v2_candidate.len()) {
            let x = v1_candidate[i];
            let y = v2_candidate[j];
            let a = N / 2 - x.1;
            let b = N / 2 - y.1;
            if x.0 == y.0 {
                ans = min(ans, N / 2 + min(a, b));
            } else {
                ans = min(ans, a + b);
            }
        }
    }

    println!("{}", ans);
}
