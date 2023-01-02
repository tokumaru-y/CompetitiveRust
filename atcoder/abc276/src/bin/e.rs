#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use std::fmt::Debug;
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        grid: [Chars; H],
    }
    let mut dist = vec![vec![-1; W]; H];
    let mut sh = 0;
    let mut sw = 0;
    for h in 0..H {
        for w in 0..W {
            if grid[h][w] == 'S' {
                dist[h][w] = 0;
                sh = h;
                sw = w;
            }
        }
    }
    let mut deque = VecDeque::new();
    deque.push_back((sh as isize, sw as isize));
    let mut key = 0;
    let mut key_list = vec![vec![-1; W]; H];
    key_list[sh][sw] = key;

    while deque.len() > 0 {
        let (nh, nw) = deque.pop_front().unwrap();

        for (dh, dw) in DXY.iter() {
            let h = nh + *dh;
            let w = nw + *dw;

            if (!(0 <= h && h < H as isize) || !(0 <= w && w < W as isize)) {
                continue;
            }
            let h = h as usize;
            let w = w as usize;

            if grid[h][w] == '#' {
                continue;
            }
            if grid[h][w] == '.' && dist[h][w] > -1 {
                if dist[nh as usize][nw as usize] + dist[h][w] >= 3
                    && key_list[nh as usize][nw as usize] != key_list[h][w]
                {
                    println!("Yes");
                    exit(0);
                } else {
                    continue;
                }
            }

            if grid[h][w] == 'S' {
                continue;
            }

            if grid[nh as usize][nw as usize] == 'S' {
                key += 1;
                key_list[h][w] = key;
            } else {
                key_list[h][w] = key_list[nh as usize][nw as usize];
            }

            dist[h][w] = dist[nh as usize][nw as usize] + 1;
            deque.push_back((h as isize, w as isize));
        }
    }

    println!("No");
}
