use proconio::input;
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        X: usize,
    }
    if A >= X {
        println!("1");
    } else if B < X {
        println!("0");
    } else {
        let ans = (C) as f64 / (B-A) as f64;
        println!("{}", ans);
    }
}