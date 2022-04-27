use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use proconio::{input, marker::Chars};
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        mut Grid: [Chars; H],
    }
    let mut ans = 0;
    let mut target = 1;
    let mut sh = 0;let mut sw = 0;
    for i in 0..H {
        for j in 0..W{
            if Grid[i][j] == 'S' {
                sh = i;
                sw = j;
                break;
            }
        }
    }
    let dx = [0,1,0,-1];let dy = [1,0,-1,0];
    while N >= target {
        let mut passed = vec![vec![20_000000; W]; H];
        let mut deque = VecDeque::from(vec![(sh, sw)]);
        passed[sh][sw] = 0;
        while deque.len() > 0 {
            let q = deque.pop_front().unwrap();
            for d in dx.iter().zip(dy.iter()){
                let nh = q.0 as isize + d.0;let nw = q.1 as isize + d.1;
                if nh < 0 || nh as usize >= H || nw < 0 || nw as usize >= W { continue;}
                let nh = nh as usize;let nw = nw as usize;
                if passed[nh][nw] < 20_000000 || Grid[nh][nw] == 'X' { continue; }
                passed[nh][nw] = passed[q.0][q.1] + 1;
                if Grid[nh][nw].to_digit(10).unwrap_or(100_000000) as usize == target {
                    target += 1;
                    ans += passed[nh][nw];
                    Grid[nh][nw] = '.';
                    sh = nh;sw = nw;
                    deque.clear();
                    break;
                }
                deque.push_back((nh,nw));
            }
        }
    }
    println!("{}",ans);
}