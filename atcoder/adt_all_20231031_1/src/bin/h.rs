use ac_library::ModInt998244353 as ModInt;

fn main() {
    proconio::input! {
        n: usize,
        d: usize,
    }
    println!("{}", distance_on_large_perfect_binary_tree(n, d));
}

fn distance_on_large_perfect_binary_tree(n: usize, d: usize) -> ModInt {
    // p[i] = 2^i
    let mut pre_power_2: Vec<ModInt> = vec![ModInt::new(1); 2 * n];
    for i in 0..2 * n - 1 {
        pre_power_2[i + 1] = pre_power_2[i] * 2;
    }
    let f1 = |d_: usize| -> ModInt {
        if d_ + d <= n - 1 {
            pre_power_2[d + 1]
        } else {
            ModInt::new(0)
        }
    };
    let f2 = |d_: usize| -> ModInt {
        if 2 * (n - 1 - d_) < d || d == 1 {
            ModInt::new(0)
        } else if d_ + d <= n - 1 {
            pre_power_2[d - 1] * (d - 1)
        } else {
            pre_power_2[d - 1] * (2 * n - 2 * d_ - d - 1)
        }
    };
    (0..n).map(|d_| pre_power_2[d_] * (f1(d_) + f2(d_))).sum()
}
