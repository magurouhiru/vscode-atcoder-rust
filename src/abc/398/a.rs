use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let s1 = "-";
    let s2 = "=";
    if n % 2 == 0 {
        let s1 = s1.repeat(n / 2 - 1);
        let s2 = s2.repeat(2);
        println!("{}{}{}", s1, s2, s1);
    } else {
        let s1 = s1.repeat(n / 2);
        println!("{}{}{}", s1, s2, s1);
    }
}
