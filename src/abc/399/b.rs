use std::collections::BTreeMap;

use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        n: u32,
        pi: [u32; n],
    }

    let mut btm: BTreeMap<u32, u32> = BTreeMap::new();
    for p in &pi {
        match btm.get_mut(p) {
            None => {
                btm.insert(*p, 1);
            }
            Some(x) => {
                *x += 1;
            }
        }
    }

    let mut cnt = 1;
    let mut m: HashMap<u32, u32> = HashMap::new();
    while let Some((i, x)) = btm.pop_last() {
        m.insert(i, cnt);
        cnt += x;
    }

    for x in pi {
        println!("{}", m.get(&x).unwrap());
    }
}
