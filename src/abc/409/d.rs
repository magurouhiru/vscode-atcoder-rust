use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }
        match s.iter().zip(s.iter().skip(1)).position(|(a, b)| a > b) {
            Some(l) => {
                let r = s.iter().skip(l + 2).position(|&c| c > s[l]).unwrap_or(n) + l + 2;
                println!(
                    "{}{}{}{}",
                    s[..l].iter().collect::<String>(),
                    if r < n {
                        s[l + 1..r].iter().collect::<String>()
                    } else {
                        s[l + 1..].iter().collect::<String>()
                    },
                    s[l],
                    if r < n {
                        s[r..].iter().collect::<String>()
                    } else {
                        "".to_string()
                    }
                );
            }
            None => {
                println!("{}", s.iter().collect::<String>());
                continue;
            }
        }
    }
}
