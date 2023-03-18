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
        M: usize,
        AB: [(Usize1, Usize1); M]
    }
    let mut edge_cnt = vec![0; N];
    let mut edge_table = vec![vec![false; N]; N];
    for (a, b) in AB.into_iter() {
        edge_cnt[a] += 1;
        edge_cnt[b] += 1;
        edge_table[a][b] = true;
        edge_table[b][a] = true;
    }
    let mut ans = 0;
    loop {
        let mut is_removed = false;
        for i in 0..N {
            if edge_cnt[i] == 1 {
                is_removed = true;
                edge_cnt[i] -= 1;
                ans += 1;
                for j in 0..N {
                    if edge_table[i][j] {
                        edge_table[i][j] = false;
                        edge_table[j][i] = false;
                        edge_cnt[j] -= 1;
                        break;
                    }
                }
            }
        }
        if !is_removed {
            break;
        }
    }

    println!("{}", ans);
}
