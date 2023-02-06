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

fn dfs(ans: &mut Vec<String>, x: &mut VecDeque<char>, y: char, N: usize) {
    if x.len() == N {
        let t: Vec<char> = x.iter().map(|&x| x).collect();
        ans.push(t.into_iter().join(""));
        return;
    }

    for a in (('a' as u8)..=(y as u8)) {
        x.push_back(a as char);
        dfs(ans, x, max((a + 1), y as u8) as char, N);
        x.pop_back();
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }
    let mut ans = Vec::new();
    dfs(&mut ans, &mut VecDeque::from(vec!['a']), 'b', N);
    for _a in ans.into_iter() {
        println!("{}", _a);
    }
}
