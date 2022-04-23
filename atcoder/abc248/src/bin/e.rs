use std::{collections::{BinaryHeap, HashMap, VecDeque}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn check(a: &Vec<isize>, b: &Vec<isize>, c: &Vec<isize>) -> bool {
    let v1 = (b[0] - a[0]) * (c[1] - a[1]);
    let v2 = (b[1] - a[1]) * (c[0] - a[0]);
    v1 == v2
}


fn main() {
    input! {
        N: usize,
        K: usize,
        XY: [[isize; 2]; N],
    }
    if K == 1 {
        println!("Infinity");
        std::process::exit(0);
    }
    let mut used = vec![vec![false; N]; N];
    let mut ans = 0;
    for i in 0..N {
        for j in i+1..N {
            if !used[i][j] {
                let a = &XY[i];
                let b = &XY[j];
                let mut list = vec![i,j];
                let mut count = 2;
                for k in j+1..N{
                    if check(&a, &b, &XY[k]) {
                        list.push(k);
                        count += 1;
                    }
                }
                for indx in 0..list.len() {
                    for indy in indx+1..list.len() {
                        used[list[indx]][list[indy]] = true;
                    }
                }
                if count >= K {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}