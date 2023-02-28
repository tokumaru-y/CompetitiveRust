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

fn calc(x: Vec<usize>, y: Vec<usize>, xy: Vec<usize>) -> usize {
    let mut a = x.len();
    let mut b = y.len();
    let mut ab = xy.len();
    if a > b {
        swap(&mut a, &mut b);
    }
    if a == 0 && b == 0 {
        if ab == 0 {
            return 0;
        } else {
            return ab - 1;
        }
    }
    if b - a > 0 {
        if b - a > ab {
            return a + ab;
        } else {
            return b + (ab - (b - a));
        }
    } else {
        return a + ab;
    }
}
fn count(s: &Vec<char>) -> usize {
    let mut res = 0;
    for i in 0..(s.len() - 1) {
        if s[i] == 'A' && s[i + 1] == 'B' {
            res += 1;
        }
    }
    res
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [Chars; N],
    }
    let mut ans = 0;
    let mut first_b = vec![];
    let mut last_a = vec![];
    let mut first_b_last_a = vec![];

    for i in 0..N {
        let s = &S[i];
        ans += count(s);
        let li = s.len() - 1;
        if s[0] == 'B' && s[li] == 'A' {
            first_b_last_a.push(i);
        } else if s[li] == 'A' {
            last_a.push(i);
        } else if s[0] == 'B' {
            first_b.push(i);
        }
    }

    let x = calc(first_b, last_a, first_b_last_a);

    ans += x;
    println!("{}", ans);
}
