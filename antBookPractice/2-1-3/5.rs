// https://atcoder.jp/contests/arc005/tasks/arc005_3
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
const FIRST_VALUE: usize = 10_000_000;
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        grid: [Chars; H]
    }
    let mut priority_queue = BinaryHeap::new();
    let mut seen = vec![vec![false; W]; H];
    for h in 0..H {
        for w in 0..W {
            if grid[h][w] == 's' {
                priority_queue.push((0, h as usize, w as usize));
                seen[h][w] = true;
            }
        }
    }

    while !priority_queue.is_empty() {
        let (mut cnt, x, y) = priority_queue.pop().unwrap();
        cnt *= -1;
        for dxy in DXY.iter() {
            let nx = x as isize + dxy.0;
            let ny = y as isize + dxy.1;
            if !(0 <= nx && nx < H as isize) || !(0 <= ny && ny < W as isize) {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if seen[nx][ny] {
                continue;
            }
            if grid[nx][ny] == '#' {
                if cnt >= 2 {
                    continue;
                }
                priority_queue.push((-(cnt + 1), nx, ny));
            } else if grid[nx][ny] == 'g' {
                println!("YES");
                std::process::exit(0);
            } else {
                priority_queue.push((-cnt, nx, ny));
            }
            seen[nx][ny] = true;
        }
    }
    println!("NO");
}

// 以下、0-1BFSを使用したもの。
// わかり易い記事:https://betrue12.hateblo.jp/entry/2018/12/08/000020
/* https://atcoder.jp/contests/arc005/tasks/arc005_3

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
const FIRST_VALUE: usize = 10_000_000;
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        grid: [Chars; H]
    }
    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    let mut dist = vec![vec![FIRST_VALUE; W]; H];
    let mut goal = (FIRST_VALUE, FIRST_VALUE);
    for h in 0..H {
        for w in 0..W {
            if grid[h][w] == 's' {
                deque.push_back((h, w));
                dist[h][w] = 0;
            } else if grid[h][w] == 'g' {
                goal = (h, w);
            }
        }
    }
    while !deque.is_empty() {
        let (x, y) = deque.pop_front().unwrap();
        for dxy in DXY.iter() {
            let nx = x as isize + dxy.0;
            let ny = y as isize + dxy.1;
            if !(0 <= nx && nx < H as isize) || !(0 <= ny && ny < W as isize) {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if grid[nx][ny] == '#' {
                if dist[nx][ny] <= dist[x][y] + 1 {
                    continue;
                }
                dist[nx][ny] = dist[x][y] + 1;
                deque.push_back((nx, ny));
            } else {
                if dist[nx][ny] <= dist[x][y] {
                    continue;
                }
                dist[nx][ny] = dist[x][y];
                deque.push_front((nx, ny));
            }
        }
    }
    println!(
        "{}",
        if dist[goal.0][goal.1] <= 2 {
            "YES"
        } else {
            "NO"
        }
    )
}
*/
