use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    let mut cnt = vec![0; t];
    for c in cnt.iter_mut() {
        input! {
            n: usize,
            s: [usize; n],
        }
        let mut ans = 2;
        let mut l = s[0];
        let r = s[n - 1];
        let mut index = 0usize;
        let buf: Vec<usize> = s.into_iter().skip(1).take(n - 2).sorted().collect();
        loop {
            if 2 * l >= r {
                *c = ans;
                break;
            }
            match buf
                .iter()
                .enumerate()
                .skip(index)
                .take_while(|(_, &v)| 2 * l >= v)
                .map(|(j, _)| j)
                .last()
            {
                Some(i) => {
                    index = i + 1;
                    l = buf[i];
                    ans += 1;
                }
                None => {
                    *c = -1;
                    break;
                }
            }
        }
    }
    println!(
        "{}",
        cnt.into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
