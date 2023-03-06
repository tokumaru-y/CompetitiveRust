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
        mut N: Chars,
    }
    let mut n_num = 0;
    let mut len = N.len();
    N.reverse();
    for i in 0..len {
        n_num += N[i].to_digit(10u32).unwrap() as usize * 10usize.pow(i as u32);
    }
    let mut deque = VecDeque::new();
    deque.push_back((3, 1, [true, false, false]));
    deque.push_back((5, 1, [false, true, false]));
    deque.push_back((7, 1, [false, false, true]));
    let mut ans = 0;
    let nn = [3, 5, 7];
    while deque.len() > 0 {
        let (num, cnt, v) = deque.pop_front().unwrap();
        if cnt == len {
            if num <= n_num && v[0] && v[1] && v[2] {
                ans += 1;
            }
            continue;
        }
        if v[0] && v[1] && v[2] {
            ans += 1;
        }

        for i in 0..3 {
            let mut tmp = v.clone();
            tmp[i] = true;
            deque.push_back((num * 10 + nn[i], cnt + 1, tmp));
        }
    }

    println!("{}", ans);
}
