use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut r = vec![];
    let u = "ABCDEFGHIJKLNMOPQRSTUVWXYZ";

    for ss in s {
        if u.contains(ss) {
            r.push(ss);
        }
    }

    let r: String = r.iter().collect();
    if r.is_empty() {
        print!("");
    } else {
        println!("{r}");
    }
}
