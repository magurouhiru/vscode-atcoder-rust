use proconio::input;

fn main() {
    input! {
        r: usize,
        x: usize,
    }

    if (x == 1 && (1600..3000).contains(&r)) || (x == 2 && (1200..2400).contains(&r)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
