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
        N: usize,
        M: usize,
        PY: [(usize, usize); M]
    }
    let mut id_by_city = vec![1; N + 1];
    let mut year_and_idx = vec![];
    let mut ans: Vec<String> = vec!["a".to_string(); M];

    for i in 0..M {
        let (p, y) = PY[i];
        year_and_idx.push((y, p, i));
    }
    year_and_idx.sort();

    for (y, p, i) in year_and_idx.into_iter() {
        let mut str = p.to_string();
        let add_zero = 6 - str.len();
        let t = vec!['0'; add_zero];
        let zero_string: String = t.into_iter().collect();
        str = zero_string + &str;

        let mut id_str = id_by_city[p].to_string();
        let add_zero = 6 - id_str.len();
        let zero_string: String = vec!['0'; add_zero].into_iter().collect();
        id_str = zero_string + &id_str;
        ans[i] = str + &id_str;
        id_by_city[p] += 1;
    }

    for i in 0..M {
        println!("{}", ans[i]);
    }
}
