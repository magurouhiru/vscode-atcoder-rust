use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: Usize1,
        di: [usize; n],
    }

    for i in 0..n {
        let mut sum = 0;
        let mut cnt = vec![0; n - i];
        for (j, d) in di.iter().skip(i).enumerate() {
            sum += *d;
            cnt[j] = sum;
        }
        println!(
            "{}",
            cnt.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
