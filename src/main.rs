use std::{collections::{BinaryHeap, HashMap, VecDeque}, cmp::Reverse};

use proconio::{input, marker::Chars};
fn main() {
    input!{
        Q: usize,
    }
    let mut ans = Vec::new();
    let mut deque = VecDeque::new();
    let mut pre_cnt = 0;
    let mut pre_num = 0;
    for _ in 0..Q{
        input!{
            cmd: usize,
        }
        if cmd == 1 {
            input!{
                x: usize,
                c: usize,
            }
            deque.push_back([x,c]);
        } else {
            input!{
                mut c: usize,
            }
            let mut tmp = 0;
            if pre_cnt < c {
                tmp = pre_num * pre_cnt;
                c -= pre_cnt;
                pre_cnt = 0;
                pre_num = 0;
                while c > 0 {
                    let num_cnt = deque.pop_front().unwrap();
                    let num = num_cnt[0];
                    let cnt = num_cnt[1];
                    if c >= cnt {
                        c -= cnt;
                        tmp += cnt * num;
                    } else {
                        pre_cnt = cnt - c;
                        pre_num = num;
                        tmp += c * num;
                        c=0;
                    }
                }
                if tmp >0 {
                    ans.push(tmp);
                }
            } else {
                ans.push(c * pre_num);
                pre_cnt -= c;
                c = 0;
            }
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"));
}