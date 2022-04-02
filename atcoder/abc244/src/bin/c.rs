use proconio::input;
use proconio::marker::Chars;
use std::io::{stdin, stdout, Write};
use std::process::exit;


fn main() {
    let mut N_in = String::new();
    stdin().read_line(&mut N_in).ok();
    let N: usize = N_in.trim().parse().ok().unwrap();
    let mut num_list = vec![false; 2*N+2];
    num_list[0] = true;

    let mut next_ans = 1;
    loop {
        println!("{}", next_ans);
        num_list[next_ans] = true;
        let mut res_in = String::new();
        stdin().read_line(&mut res_in).ok();
        let res: usize = res_in.trim().parse().ok().unwrap();
        num_list[res] = true;
        for i in 1..2*N+2{
            if !num_list[i] {
                next_ans = i;
                break;
            }
        }
        if res == 0 {
            exit(0);
        }
    }
}