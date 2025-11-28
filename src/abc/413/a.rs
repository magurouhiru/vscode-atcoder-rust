use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let sum: usize = a.iter().sum();
    if sum <= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
