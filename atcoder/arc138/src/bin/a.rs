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

fn read() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<usize>().unwrap()
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }
    let mut v = vec![];
    for i in 0..N {
        v.push((A[i], i as isize * -1));
    }
    v.sort();
    let mut ans = std::isize::MAX;
    let mut left_ind = std::isize::MIN;
    for (_, i) in v.into_iter() {
        let ind = i * -1;
        if (ind as usize) < K {
            left_ind = max(left_ind, ind)
        } else if left_ind > std::isize::MIN {
            ans = min(ans, (ind - left_ind));
        }
    }
    if ans == std::isize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
