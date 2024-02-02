# e
dp の計算でほかの人の提出見ててなんか俺だけ遅い気がしたから調べた。
改良前：760msほど
```rust
fn pre(pt: Vec<(usize, usize)>) -> Vec<usize> {
    let lcm: usize = pt
        .iter()
        .map(|(pi, ti)| *pi)
        .reduce(num::integer::lcm)
        .unwrap();
    // dp[i in 0..lcm] = the time it takes to arrive at last stop when start at i time from 1 st stop
    (0..lcm)
        .map(|start| {
            let mut time = start;
            for &(pi, ti) in &pt {
                time = num::integer::div_ceil(time, pi) * pi + ti;
            }
            time - start
        })
        .collect_vec()
}
```
改良後：450msほど
```rust
fn pre(pt: Vec<(usize, usize)>) -> Vec<usize> {
    let lcm: usize = pt
        .iter()
        .map(|(pi, ti)| *pi)
        .reduce(num::integer::lcm)
        .unwrap();
    // dp[i in 0..lcm] = the time it takes to arrive at last stop when start at i time from 1 st stop
    let mut dp = (0..lcm).collect_vec();
    for &(pi, ti) in &pt {
        for time in &mut dp {
            *time = num::integer::div_ceil(*time, pi) * pi + ti;
        }
    }
    for i in 0..lcm {
        dp[i] -= i;
    }
    dp
}
```
前者は &ptを中で使っているので、 `&pt` で値のコピーが time 倍だけ発生しているとかなのか？
後で調べられたら調べたい。
