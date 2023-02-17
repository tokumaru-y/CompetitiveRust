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
        A: [usize; N],
    }
    let mut A = VecDeque::from(A);
    A.push_front(0);
    let mut div = vec![0; N + 1];
    for i in (1..=N).rev() {
        let mut sum = 0;
        for j in ((i + i)..=N).step_by(i) {
            sum += div[j];
        }
        if sum % 2 != A[i] {
            div[i] = 1;
        }
    }

    let mut ans = vec![];
    for i in 1..=N {
        if div[i] > 0 {
            ans.push(i);
        }
    }

    println!("{}", ans.len());
    if ans.len() > 0 {
        println!("{}", ans.into_iter().join(" "));
    }
}
