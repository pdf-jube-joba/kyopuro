# h
解説はわからないけどコードがわからないのに動いてしまった。
ワーシャルフロイド法
```rust
let mut dist: Vec<Vec<usize>> = vec![vec![std::usize::MAX; n]; n];

for &(a, b, c) in &abc {
    dist[a][b] = c;
    dist[b][a] = c;
}

// for i in 0..n {
//     dist[i][i] = 0;
// }

for k in 0..n {
    for i in 0..n {
        for j in 0..n {
            dist[i][j] = std::cmp::min(dist[i][j], dist[i][k].saturating_add(dist[k][j]))
        }
    }
}
```

これってワーシャルフロイド法だとコメントの部分はコメントを外すべきなんだが、
コメントアウトしたら通る。

おそらく理由としてはもともとの問題を考えると、距離が C であって 2 本以上の辺を使った path が存在するかどうかで距離を測りたいというものだった。
なのでそこを考えるといいということだと思うけど。
よくわからなかったので、 `d[a][i] + d[i][b] <= c ` を filter した。