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
        mut XYH: [(isize, isize, isize); N]
    }
    XYH.sort_by(|a, b| b.2.cmp(&a.2));
    for cx in 0..=100 {
        for cy in 0..=100 {
            let (x, y, h) = XYH[0];

            let height = h + (x - cx as isize).abs() + (y - cy as isize).abs();

            let mut is_ok = true;
            for i in 1..N {
                let (x, y, h) = XYH[i];
                is_ok &= (h
                    == max(
                        height - (x - cx as isize).abs() - (y - cy as isize).abs(),
                        0,
                    ));
            }

            if is_ok {
                println!("{} {} {}", cx, cy, height);
                exit(0);
            }
        }
    }
}
