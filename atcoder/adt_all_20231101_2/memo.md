# g
問題としては、
`A[i] = A[j] * A[k]` を満たす `A[i]` の列挙だった。
全探索、`(i,j)` に対する二分探索、 `(i,j)` と尺取り法とかで $O(n^2)$ までは減らせたがそれ以上は思いつかなかった。
約数を列挙するのがよいらしい。
それだけ聞いて次のように書いた。
```rust
fn index_trio(a: Vec<usize>) -> usize {
    let num = {
        let mut num = vec![0; M + 1];
        for ai in a {
            num[ai] += 1;
        }
        num
    };
    // sum_{(x, y, z) in (0..M)^3 | x = yz} num[x] * num[y] * num[z]
    let mut count = 0;
    for x in 0..=M {
        for y in (1..=x.sqrt()).filter(|y| x % y == 0) {
            let nums = num[x] * num[y] * num[x / y];
            count += nums;
        }
    }
    count
}
```
これは間違いで、どちらかというと x を動かすよりは `a[i]` を先に動かす方がよい。
ちゃんと y と z が重なるとかを考えるべき。

# h
確率でDPをするのと個数でDPをするのでは、modが入ると結果が異なる？
確かにそうなのかちゃんと計算したい。
