#[allow(unused_imports)]
use itertools::Itertools;
use num_traits::Pow;
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

fn calc(table: &mut Vec<Vec<isize>>, a: usize, b: usize) -> isize {
    if b == 0 || a == b {
        return 1;
    }
    if table[a][b] >= 0 {
        return table[a][b];
    }
    table[a][b] = calc(table, a - 1, b - 1) + calc(table, a - 1, b);
    return table[a][b];
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: usize,
        A: [usize; N]
    }
    let max_line = 51;
    let mut table = vec![vec![-1isize; max_line]; max_line];

    let mut odd = 0;
    let mut even = 0;
    for i in 0..N {
        if A[i] % 2 == 1 {
            odd += 1;
        } else {
            even += 1;
        }
    }

    let mut ans = 0;
    let mut now_idx = P;
    while now_idx <= odd {
        ans += calc(&mut table, odd, now_idx) as usize;
        now_idx += 2;
    }
    while even > 0 {
        ans *= 2;
        even -= 1;
    }
    println!("{}", ans);
}
