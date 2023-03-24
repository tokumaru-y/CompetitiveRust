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
        mut K: usize,
    }
    let mut ans = S.clone();

    for i in 0..(S.len()) {
        if S[i] == 'a' {
            continue;
        }
        let dis = ('z' as u8 - (S[i] as u8)) as usize;
        if dis + 1 <= K {
            K -= dis + 1;
            ans[i] = 'a';
        }
        if K == 0 {
            break;
        }
    }

    if K > 0 {
        ans[S.len() - 1] = (((ans[S.len() - 1] as usize - 'a' as usize + K) % 26) as usize
            + 'a' as usize) as u8 as char;
    }

    println!("{}", ans.into_iter().join(""));
}
