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
// x以上の値の中で、最も左側にあるindexを返す。
// vecに対象が存在しない場合はv.lenを返す
fn lower_bound<T: std::cmp::PartialOrd>(vec: &Vec<T>, x: T) -> usize {
    let mut is_ng: isize = -1;
    let mut is_ok: isize = vec.len() as isize;
    while (is_ok - is_ng).abs() > 1 {
        let mid = (is_ok + is_ng) / 2;

        if x <= vec[mid as usize] {
            is_ok = mid;
        } else {
            is_ng = mid;
        }
    }

    is_ok as usize
}
fn calc(A: &Vec<usize>, val: usize) -> usize {
    let ind = lower_bound(A, val);
    let mut res = std::isize::MAX;
    if ind > 0 {
        res = (A[(ind as isize - 1 as isize) as usize] as isize - val as isize).abs();
    }
    if ind < A.len() {
        res = min(res, (A[ind as usize] as isize - val as isize).abs());
    }

    res as usize
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AC: [(usize, char); 2*N]
    }
    let mut red = vec![];
    let mut green = vec![];
    let mut blue = vec![];
    for (a, c) in AC.into_iter() {
        if c == 'R' {
            red.push(a);
        } else if c == 'G' {
            green.push(a);
        } else {
            blue.push(a);
        }
    }
    red.sort();
    green.sort();
    blue.sort();

    let is_red_even = red.len() % 2 == 0;
    let is_green_even = green.len() % 2 == 0;
    let is_blud_even = blue.len() % 2 == 0;
    if is_red_even && is_green_even && is_blud_even {
        println!("0");
        exit(0);
    }

    let mut even_list = vec![];
    let mut odds_list_one = vec![];
    let mut odds_list_two = vec![];
    if is_red_even {
        even_list = red.clone();
        odds_list_one = green.clone();
        odds_list_two = blue.clone();
    } else if is_green_even {
        even_list = green.clone();
        odds_list_one = red.clone();
        odds_list_two = blue.clone();
    } else {
        even_list = blue.clone();
        odds_list_one = green.clone();
        odds_list_two = red.clone();
    }

    let mut ans = std::usize::MAX;

    // 以下の2パターンは独立に考えて最小値を比較している。数直線を書いて場合分けすると納得できる
    // odds & odds
    for one in odds_list_one.iter() {
        ans = min(ans, calc(&odds_list_two, *one));
    }
    if even_list.len() > 0 {
        // even & odds + even & odds
        let mut tmp = std::usize::MAX;
        for e in even_list.iter() {
            tmp = min(tmp, calc(&odds_list_one, *e));
        }
        let mut add = std::usize::MAX;
        for e in even_list.iter() {
            add = min(add, calc(&odds_list_two, *e));
        }
        ans = min(ans, tmp + add);
    }

    println!("{}", ans);
}
