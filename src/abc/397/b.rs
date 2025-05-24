use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt = 0;
    let mut is_odd = true;
    for ss in s.iter() {
        if (is_odd && *ss != 'i') || (!is_odd && *ss != 'o') {
            cnt += 1;
        } else {
            is_odd = !is_odd;
        }
    }
    if (s.len() + cnt) % 2 != 0 {
        cnt += 1;
    }

    println!("{cnt}");
}
