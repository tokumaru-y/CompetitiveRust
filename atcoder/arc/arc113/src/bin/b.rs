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
        S: Chars
    }
    let mut ans = 0;
    let mut idx = S.len() as isize - 1;
    let mut last_char = '1';
    let mut dict = HashMap::new();
    while idx >= 0isize {
        if last_char == S[idx as usize] && dict.len() > 1 {
            ans += S.len() as isize - idx - 2;
            if dict.contains_key(&S[idx as usize]) {
                ans -= dict.get(&S[idx as usize]).unwrap();
                ans += 1;
                dict.clear();
                dict.insert(S[idx as usize], S.len() as isize - idx);
            }
        } else {
            *dict.entry(S[idx as usize]).or_insert(0) += 1;
        }
        last_char = S[idx as usize];
        idx -= 1;
    }

    println!("{}", ans);
}
