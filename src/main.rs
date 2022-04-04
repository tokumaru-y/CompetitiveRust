use proconio::{input, marker::Chars};
// rust-analyzerを利かすため一時配置。
fn main() {
    input! {
        mut s: Chars,
    }
    s.sort();
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}