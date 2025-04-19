use proconio::input;

fn main() {
    input! {
        s: u32,
    }

    if (200..=299).contains(&s) {
        println!("Success");
    } else {
        println!("Failure");
    }
}
