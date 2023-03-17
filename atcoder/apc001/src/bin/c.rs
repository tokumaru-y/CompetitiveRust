#[allow(unused_imports)]
use itertools::Itertools;
use proconio::source::line::LineSource;
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
    io::{stdin, stdout, BufReader, Write},
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

fn check(sleft: &String, sright: &String, left: usize, right: usize) -> bool {
    if sleft == sright {
        (right - left) % 2 == 1
    } else {
        (right - left) % 2 == 0
    }
}

#[allow(non_snake_case)]
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        N: usize,
    }
    let v_str = "Vacant";
    let mut left = 0;
    let mut right = N / 2;
    println!("{}", left);
    input! {
        from &mut source,
        mut sleft: String
    }
    if sleft == v_str {
        exit(0);
    }
    println!("{}", right);
    input! {
        from &mut source,
        mut sright: String
    }
    if sright == v_str {
        exit(0);
    }

    if !check(&sleft, &sright, left, right) {
        swap(&mut sleft, &mut sright);
        left = right;
        right = N;
    }

    while (right - left > 1) {
        let mid = (left + right) / 2;
        println!("{}", mid);
        input! {
            from &mut source,
            tmp_str: String
        }

        if tmp_str == v_str {
            break;
        }
        if check(&sleft, &tmp_str, left, mid) {
            sright = tmp_str;
            right = mid;
        } else {
            sleft = tmp_str;
            left = mid;
        }
    }
}
