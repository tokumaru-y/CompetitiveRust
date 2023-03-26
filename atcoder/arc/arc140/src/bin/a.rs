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
        K: usize,
        S: Chars,
    }

    for i in 1..=N {
        if N % i != 0 {
            continue;
        }

        let div = N / i;
        let mut cnt = 0;
        for j in 0..i {
            let mut num_cnt = vec![0; 26];
            for l in 0..div {
                let pos = j + l * i;
                let ind = S[pos] as usize - 'a' as usize;
                num_cnt[ind] += 1;
            }
            cnt += num_cnt.iter().sum::<usize>() - num_cnt.iter().max().unwrap();
        }

        if cnt <= K {
            println!("{}", i);
            exit(0);
        }
    }
}
