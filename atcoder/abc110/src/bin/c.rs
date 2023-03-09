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
        S: Chars,
        T: Chars,
    }
    let mut table = vec![vec![0; 26]; 26];
    let l = S.len();

    for i in 0..l {
        let a = S[i] as usize - 'a' as usize;
        let b = T[i] as usize - 'a' as usize;
        table[a][b] = 1;
    }

    for i in 0..26 {
        let mut cnt = 0;
        for j in 0..26 {
            cnt += table[i][j];
        }

        if cnt >= 2 {
            println!("No");
            exit(0);
        }
    }

    for j in 0..26 {
        let mut cnt = 0;
        for i in 0..26 {
            cnt += table[i][j];
        }

        if cnt >= 2 {
            println!("No");
            exit(0);
        }
    }

    println!("Yes");
}
