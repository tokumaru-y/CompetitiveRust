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
    let unreach_value = 1000_000_000;
    let mut count_table = vec![vec![unreach_value; W]; H];
    let mut deque = VecDeque::new();
    for i in 0..H{
        for j in 0..W {
            if grid[i][j] == '#' {
                count_table[i][j] = 0;
                deque.push_back((i,j));
            }
        }
    }
    let dx = [0,1,0,-1];let dy = [1,0,-1,0];
    while deque.len() > 0 {
        let start_point = deque.pop_front().unwrap();
        for d in dx.iter().zip(dy.iter()) {
            let nx: isize = start_point.0 as isize + d.0;
            let ny: isize = start_point.1 as isize + d.1;
            if nx < 0 || nx >= H as isize || ny < 0 || ny >= W as isize { continue; }
            let nx = nx as usize;let ny = ny as usize;
            if count_table[nx][ny] != unreach_value || grid[nx][ny] == '#' {continue;}
            count_table[nx][ny] = count_table[start_point.0][start_point.1] + 1;
            deque.push_back((nx,ny));
        }
    }
    let mut ans = 0;
    for v in count_table.iter(){ ans = std::cmp::max(ans, *v.iter().max().unwrap() as isize )}
    println!("{}", ans);
}