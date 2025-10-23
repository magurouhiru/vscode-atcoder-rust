use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut ai: Vec<usize> = (1..=n).collect();
    let mut offset = 0;

    for _ in 0..q {
        input! {
            (t, pk) :(usize, Usize1),
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                ai[(pk + offset) % n] = x;
            }
            2 => {
                println!("{}", ai[(pk + offset) % n]);
            }
            3 => {
                let k = (pk + 1) % n;
                offset = (offset + k) % n;
            }
            _ => unreachable!(),
        }
    }
}
