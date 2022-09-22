# 問題
前提
- $n$: 整数
- $x$: 整数
---
求めるもの
- $\#\{A \subset \{1..=n\} \mid \#A = 3 , A = \{i,j,k\} として i+j+k = x\}$
---
制約
- $3 \leq n \leq 100$
- $0 \leq x \leq 300$

# 方針
$(i < j < k)$ のうち二つが決まれば残り一つは決まるので、 $(i,j)$ から条件を満たすものを数え上げる。

# 実装のメモ
## 1
よく知っているやり方として、for 文で数え上げる方法をまず使った。

```Rust
let mut count = 0;
for i in 1..n {
    for j in i+1..n {
        let k = x - (i + j);
        if k > i && k > j && n >= k {
            count += 1;
        }
    }
}
return count;
```
- $i,j$ の動く範囲が $n$ に届いてないが、 $0 \leq k$ なのでよい
- `x - (i + j)` は usize の引き算なので場合によってはエラーになる
- `k > i` は $i < j$ から判定する必要がない
```Rust
let mut count = 0;
for i in 1..n-1 {
    for j in i+1..n {
        if (i + j) < x {
            let k = x - (i + j);
            if j < k && k <= n {
                count += 1;
            }
        }
    }
}
return count;
```
- こちらのほうがより正しい書き方になっていると思う。

## 2
上の書き方はよくあると思うけど、もうちょっと別の書き方に挑戦した。
$(i,j)$ であって $1 \leq i < j \leq n-1$ を並べて、そのうち条件を満たすものを数え上げているので次のように書ける。
```Rust
(1..n)
.flat_map(|i| (i+1..n).map(move |j|(i,j)))
.filter(|&(i,j)| {
    i + j < x && {
        let k = x - (i + j);
        j < k && k <= n
    }
})
.count()
```

# input について
proconio を使おうかとも思ったが、入力の最後が 0 0 であることを考えて自分で書くことになった。

```Rust
fn input() -> Vec<(usize, usize)> {
    let mut vec = Vec::new();
    loop {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();
        let tuple: Vec<usize> = str
            .split_whitespace()
            .map(|str|{str.parse().unwrap()})
            .collect();
        let (x,y) = (tuple[0], tuple[1]);
        if x == 0 && y == 0 {
            break
        } else {
            vec.push((x, y));
        }
    }
    vec
}
```
これがよい書き方なのかはわからなかった。
- 入力を考えると、`tuple[0], tuple[1]` は必ずアクセスできるのでよい
- 入力の終わりには必ず `0 0` が入りそれ以外では `0 0` がないので、 loopを必ず正しい位置で抜けるはず