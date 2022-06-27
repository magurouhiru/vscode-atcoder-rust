use proconio::*;

fn main() {
    input! {
        n: usize,
        x: usize,
    }
    let c: char = ((65 + ((x - 1) / n)) as u8) as char;
    println!("{}", c);
}
