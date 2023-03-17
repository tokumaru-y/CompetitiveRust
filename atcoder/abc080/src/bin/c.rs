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
        mut F: [[usize; 10]; N],
        P: [[isize; 11]; N]
    }
    for i in 0..N {
        F[i].reverse();
    }
    let mut ans = std::isize::MIN;
    for bit in 1..(1 << 10) {
        let mut tmp_sum = 0;
        for i in 0..N {
            let mut cnt = 0;
            let open_info = &F[i];
            for j in 0..10 {
                if open_info[j] == 1 && (bit & (1 << j) > 0) {
                    cnt += 1;
                }
            }
            tmp_sum += P[i][cnt];
        }
        ans = max(ans, tmp_sum)
    }

    println!("{}", ans);
}
