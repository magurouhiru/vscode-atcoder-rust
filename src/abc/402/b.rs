use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut i = 0;
    let mut b = vec![];
    for _ in 0..q {
        input! {
            n: usize,
        }
        if n == 1 {
            input! {
                x: usize,
            }
            b.push(x);
        } else {
            println!("{}", b[i]);
            i += 1;
        }
    }
}
