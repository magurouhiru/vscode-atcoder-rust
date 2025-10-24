use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ai: [usize; n],
    }

    // 降順ソート
    ai.sort_unstable_by(|a, b| b.cmp(a));

    for x in 1..=n {
        if ai[x - 1] <= x {
            println!("{}", ai[x - 1]);
            return;
        }
    }
    println!("0");

    // // n から 1 まで逆順で確認
    // for x in (1..=n).rev() {
    //     if x <= ai[x - 1] {
    //         println!("{}", x);
    //         return;
    //     }
    // }
    // println!("0");
}
