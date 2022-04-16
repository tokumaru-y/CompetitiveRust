use std::{collections::{BinaryHeap, HashMap}, cmp::Reverse};

use proconio::{input, marker::Chars};
fn main() {
    input!{
        N: usize,
        names: [[String; 2]; N],
    }
    for i in 0..N {
        let mut is_ok = false;
        for ni in 0..2{
            let mut n_ok = true;
            for j in 0..N {
                if i == j {continue;}
                if names[i][ni] == names[j][0] || names[i][ni] == names[j][1] {
                    n_ok = false;
                }
            }
            if n_ok == true { is_ok = true;}
        }
        if is_ok == false {
            println!("No");
            std::process::exit(0);
        }
    }
    println!("Yes");
}