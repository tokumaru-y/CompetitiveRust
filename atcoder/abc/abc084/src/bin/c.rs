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
        CSF: [(usize, usize, usize); N-1]
    }
    let mut ans = vec![0; N];
    for i in 0..(N - 1) {
        let mut city_idx = i;
        let mut seconds = 0;
        while city_idx < N - 1 {
            let (nc, ns, nf) = CSF[city_idx];
            if ns >= seconds {
                seconds = ns + nc;
            } else {
                let y = (seconds - ns) / nf;
                let next_seconds = if (seconds - ns) % nf == 0 {
                    ns + nf * y + nc
                } else {
                    ns + nf * (y + 1) + nc
                };
                seconds = next_seconds;
            }
            city_idx += 1;
        }
        ans[i] = seconds;
    }
    for a in ans.into_iter() {
        println!("{}", a);
    }
}
