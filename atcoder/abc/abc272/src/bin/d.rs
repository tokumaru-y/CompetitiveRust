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
        N: usize,
        M: usize,
    }
    let mut dist = vec![vec![-1; N]; N];
    let mut deque = VecDeque::new();
    dist[0][0] = 0;
    deque.push_back((0, 0));

    while deque.len() > 0 {
        let (nx, ny) = deque.pop_front().unwrap();

        for add_x in 0..(((M as f64).sqrt() + 1.0) as usize) {
            let add_x = add_x as isize;
            let add_y = M as isize - add_x * add_x;
            let tmp_y = (add_y as f64).sqrt() as usize;
            if tmp_y * tmp_y != add_y as usize {
                continue;
            }
            let add_y = tmp_y as isize;

            for i in 0..4 {
                let x = nx + add_x;
                let y = ny + add_y;
                let mut x = x as isize;
                let mut y = y as isize;
                if i == 1 {
                    x = nx + add_y;
                    y = ny - add_x;
                } else if i == 2 {
                    x = nx - add_x;
                    y = ny - add_y;
                } else if i == 3 {
                    x = nx - add_y;
                    y = ny + add_x;
                }

                if !(0 <= x && x < N as isize) || !(0 <= y && y < N as isize) {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;

                if dist[x][y] != -1 {
                    continue;
                }
                dist[x][y] = dist[nx as usize][ny as usize] + 1;
                deque.push_back((x as isize, y as isize));
            }
        }
    }

    for i in 0..N {
        println!("{}", dist[i].iter().join(" "));
    }
}
