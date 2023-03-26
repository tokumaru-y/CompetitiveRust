#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
fn main() {
    input! {
        H: usize,
        W: usize,
        grid: [Chars; H],
    }
    let first_value = 10_0000;
    let mut cost_table = vec![vec![first_value; W]; H];
    cost_table[0][0] = 0;
    let mut deque = VecDeque::from(vec![(0,0)]);
    let dx = [0,1,0,-1];let dy = [1,0,-1,0];
    while deque.len() > 0 {
        let next = deque.pop_front().unwrap();
        for d in dx.iter().zip(dy.iter()) {
            let nx = d.0 + next.0 as isize;let ny = d.1 + next.1 as isize;
            if nx < 0 || nx >= H as isize || ny < 0 || ny >= W as isize { continue; }
            let nx = nx as usize; let ny = ny as usize;
            if cost_table[nx][ny] != first_value || grid[nx][ny] == '#' { continue; }
            cost_table[nx][ny] = cost_table[next.0][next.1] + 1;
            deque.push_back((nx,ny));
        }
    }
    if cost_table[H-1][W-1] == first_value {
        println!("-1");
        std::process::exit(0);
    }
    let pass_cnt = cost_table[H-1][W-1] as isize;
    let mut black_cnt = 1;
    for v in grid.iter() { black_cnt += v.iter().filter(|x| **x == '#').count()}
    println!("{}", (H*W) as isize - pass_cnt - black_cnt as isize);
}