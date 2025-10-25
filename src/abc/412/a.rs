use proconio::input;

fn main() {
    input! {
        n: usize,
        abi: [(usize, usize); n],
    }

    println!("{}", abi.iter().filter(|&&(a, b)| a < b).count());
}
