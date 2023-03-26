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
        S: Chars
    }
    let mut tmp: Vec<char> = S.clone().into_iter().collect();
    tmp.sort();
    tmp.dedup();
    if tmp.len() == 1 {
        println!("0");
        exit(0)
    }
    let mut ans = std::usize::MAX;
    for c in b'a'..=b'z' {
        let c = c as char;
        let mut cnt = 0;
        let mut sc = S.clone();
        loop {
            let mut is_ok = true;
            let mut scc = vec!['x'; sc.len() - 1];
            for i in 0..(sc.len() - 1) {
                if sc[i] == c || sc[i + 1] == c {
                    scc[i] = c;
                } else {
                    scc[i] = sc[i];
                    is_ok = false;
                }
            }
            sc = scc;
            cnt += 1;
            if is_ok {
                break;
            }
        }

        ans = min(ans, cnt);
    }

    println!("{}", ans)
}
