use proconio::input;

fn main() {
    input! {
        (n,l): (usize, usize),
        di: [usize; n-1],
    }

    // 正三角形に分割できない場合は0を出力して終了
    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut cnt = vec![0usize; l];
    let mut index = 0;
    cnt[index] = 1;
    for &d in &di {
        index += d;
        index %= l;
        cnt[index] += 1;
    }

    let mut total = 0;
    let d = l / 3;
    for i in 0..d {
        total += cnt[i] * cnt[i + d] * cnt[i + 2 * d];
    }

    println!("{}", total);
    // let mut map: HashMap<usize, usize> = std::collections::HashMap::new();
    // let mut index = 0;
    // map.insert(index, 1);
    // for &d in &di {
    //     index += d;
    //     index %= l;
    //     map.entry(index).and_modify(|v| *v += 1).or_insert(1);
    // }

    // let mut total = 0;
    // let d = l / 3;
    // for i in 0..d {
    //     total += map.get(&i).unwrap_or(&0)
    //         * map.get(&(i + d)).unwrap_or(&0)
    //         * map.get(&(i + 2 * d)).unwrap_or(&0);
    // }

    // println!("{}", total);
}
