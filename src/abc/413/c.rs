use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut i = 0usize;
    let mut buf = Vec::with_capacity(2 * 100000);
    let mut ans = Vec::with_capacity(2 * 100000);
    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                c: usize,
                x: usize,
            }
            buf.push((c, x));
        } else {
            input! {
                mut k: usize,
            }
            let mut tmp = 0usize;
            while k > 0 {
                let (c, x) = &mut buf[i];
                match k.cmp(c) {
                    std::cmp::Ordering::Less => {
                        tmp += *x * k;
                        *c -= k;
                        k = 0;
                    }
                    std::cmp::Ordering::Equal => {
                        tmp += *x * k;
                        *c = 0;
                        k = 0;
                        i += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        tmp += *x * *c;
                        k -= *c;
                        *c = 0;
                        i += 1;
                    }
                }
            }
            ans.push(tmp);
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
