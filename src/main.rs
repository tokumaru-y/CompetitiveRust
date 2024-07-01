use proconio::input;

fn main() {
    input! {
        N: usize,
        L: usize,
        K: usize,
        mut A: [usize; N]
    }
    let mut is_ok = 0usize;
    let mut is_ng = L;
    A.push(L);
    let mut new_a = vec![A[0]];
    for i in 0..N {
        new_a.push(A[i + 1] - A[i])
    }
    while is_ng - is_ok > 1 {
        let check = (is_ok + is_ng) / 2;
        if is_acceptable(check, &new_a, K) {
            is_ok = check;
        } else {
            is_ng = check;
        }
    }

    println!("{:?}", is_ok);
}

fn is_acceptable(n: usize, A: &Vec<usize>, K: usize) -> bool {
    let mut leng = 0usize;
    let mut cnt = 0;
    for a in A.iter() {
        leng += a;
        if leng >= n {
            cnt += 1;
            leng = 0;
        }
    }

    return cnt > K;
}
