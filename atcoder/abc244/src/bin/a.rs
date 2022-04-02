use proconio::input;
use proconio::marker::Chars;


fn main() {
    input!{
        N: usize,
        S: Chars,
    }
    println!("{}", S[N-1]);
}