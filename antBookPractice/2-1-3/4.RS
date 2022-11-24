// https://atcoder.jp/contests/agc033/tasks/agc033_a
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        mut grid: [Chars; H]
    }
    let mut ans = 0;
    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    for h in 0..H {
        for w in 0..W {
            if grid[h][w] == '#' {
                deque.push_back((h, w));
            }
        }
    }

    while !deque.is_empty() {
        let len = deque.len();
        for _ in 0..len {
            let (x, y) = deque.pop_front().unwrap();
            for dxy in DXY.iter() {
                let nx = dxy.0 + x as isize;
                let ny = dxy.1 + y as isize;
                if !(0 <= nx && nx < H as isize) || !(0 <= ny && ny < W as isize) {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[nx][ny] == '#' {
                    continue;
                }
                grid[nx][ny] = '#';
                deque.push_back((nx, ny));
            }
        }
        if deque.len() > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
