use proconio::input;
use std::process::exit;

fn main () {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut used = [false; 2002];
    for iter in A {
        used[iter] = true;
    }
    for i in 0..2002 {
        if !used[i] {
            println!{"{}", i};
            exit(0);
        }
    }
}