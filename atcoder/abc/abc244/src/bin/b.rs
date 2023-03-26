use proconio::input;
use proconio::marker::Chars;

struct Point {
    x: isize,
    y: isize,
    direction: isize,
}

impl Point {

    fn step(&mut self) {
        match self.direction {
            0 => self.x += 1,
            1 => self.y -= 1,
            2 => self.x -= 1,
            3 => self.y += 1,
            _ => (),
        }
    }


    fn rotate(&mut self) {
        match self.direction {
            0 => self.direction = 1,
            1 => self.direction = 2,
            2 => self.direction = 3,
            3 => self.direction = 0,
            _ => (),
        }
    }
}

fn main() {
    input!{
        N: usize,
        T: Chars,
    }
    let mut p = Point {
        x: 0,
        y: 0,
        direction: 0,
    };
    for i in 0..N{
        if T[i] == 'S' {
            p.step();
        } else {
            p.rotate();
        }
    }
    println!("{} {}", p.x, p.y);
}