use std::process;
use proconio::input;
fn main() {
    input!{
        mut V: usize,
        A: usize,
        B: usize,
        C: usize,
    }
    let v = vec![A,B,C];
    let ans = ["F", "M", "T"];
    loop {
        for i in 0..3 {
            if V < v[i] {
                println!("{}", ans[i]);
                process::exit(0);
            } else {
                V -= v[i];
            }
        }
    }
}