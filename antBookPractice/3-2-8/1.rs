// https://atcoder.jp/contests/joi2008ho/tasks/joi2008ho_e
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
fn compress(x: &mut Vec<usize>, y: &mut Vec<usize>, L: usize) -> usize {
    let mut res = vec![0, L];

    for v in x.iter().zip(y.iter()) {
        res.push(*v.0);
        res.push(*v.1);
    }
    res.sort();
    res.dedup();

    for i in 0..x.len() {
        x[i] = res.binary_search(&x[i]).unwrap();
        y[i] = res.binary_search(&y[i]).unwrap();
    }

    res.len()
}
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        XY: [(usize, usize, usize, usize); N]
    }
    let mut x0 = Vec::new();
    let mut x1 = Vec::new();
    let mut y0 = Vec::new();
    let mut y1 = Vec::new();
    for (a, b, c, d) in XY.into_iter() {
        x0.push(a);
        x1.push(c);
        y0.push(b);
        y1.push(d);
    }

    let len_h = compress(&mut x0, &mut x1, H);
    let len_w = compress(&mut y0, &mut y1, W);

    let mut grid = vec![vec![0; len_w + 1]; len_h + 1];
    for i in 0..N {
        grid[x0[i]][y0[i]] += 1;
        grid[x1[i]][y0[i]] -= 1;
        grid[x0[i]][y1[i]] -= 1;
        grid[x1[i]][y1[i]] += 1;
    }

    for h in 0..=len_h {
        for w in 1..len_w {
            grid[h][w] += grid[h][w - 1];
        }
    }
    for w in 0..=len_w {
        for h in 1..len_h {
            grid[h][w] += grid[h - 1][w];
        }
    }

    let mut que = VecDeque::new();
    let mut ans = 0;
    for h in 0..len_h - 1 {
        for w in 0..len_w - 1 {
            if grid[h][w] == 0 {
                que.push_back((h, w));
                ans += 1;
            }

            while let Some((nh, nw)) = que.pop_front() {
                if grid[nh][nw] > 0 {
                    continue;
                }

                grid[nh][nw] = 1;

                for d in &vec![-1, 1] {
                    let ah = nh as isize + d;
                    let aw = nw as isize + d;

                    if ah >= 0 && ah < (len_h as isize - 1) && grid[ah as usize][nw] == 0 {
                        que.push_back((ah as usize, nw));
                    }
                    if aw >= 0 && aw < (len_w as isize - 1) && grid[nh][aw as usize] == 0 {
                        que.push_back((nh, aw as usize));
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
