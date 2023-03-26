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

fn dfs(pref: usize, idx: Vec<usize>, S: &Vec<Vec<char>>, ans: &mut Vec<usize>) {
    let mut group = vec![vec![]; 26];

    for i in idx.iter() {
        if pref < S[*i].len() {
            group[(S[*i][pref] as isize - 'a' as isize) as usize].push(*i);
        }
    }

    for i in idx.iter() {
        ans[*i] = pref;
    }

    for g in group.into_iter() {
        if g.len() >= 2 {
            dfs(pref + 1, g, S, ans)
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [Chars; N]
    }
    let mut ans = vec![0; N];
    let idx: Vec<usize> = (0..N).collect();

    dfs(0, idx, &S, &mut ans);

    for a in ans.into_iter() {
        println!("{}", a);
    }
}
