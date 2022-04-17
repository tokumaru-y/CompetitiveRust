use std::{collections::{BinaryHeap, HashMap, VecDeque}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn operation2(&mut deque: VecDeque<Vec<usize,usize>>, &mut ans: Vec<usize>){
    input!{
        mut c: usize,
    }
    let mut tmp_ans = 0;
    while c > 0 {
        let v = deque.pop_front().unwrap();
        if v[1] > c {
            v[1] -= c;
            tmp_ans += c * v[0];
            v.push_front(v);
        } else {
            tmp_ans += v[1] * v[0];
            c -= v[1];
        }
    }
    ans.append(tmp_ans);
}

fn main() {
    input!{
        Q: usize,
    }
    let mut ans = Vec::new();
    let mut deque = VecDeque::new();
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
            operation2(&mut deque, &mut ans);
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"));
}