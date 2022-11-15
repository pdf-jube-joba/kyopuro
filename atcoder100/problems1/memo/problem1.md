# 問題
$3 \leq n \leq 100$ の整数と $0 \leq x \leq 300$ の整数が与えられる。
$\{1 ..=n\}$ の部分集合 $A$ であって、
- $A$ は大きさ $3$ かつ $A = \{i,j,k\}$ としたとき $i+j+k=x$
を満たすものの個数を求めよ。

# 回答 1
## 設計
### 方針
$i+j+k=x$ を満たす $(1 \leq i < j < k \leq n)$ は、$i,j$ が決まれば $k$ は決まる。
そのため、 $(1 \leq i < j < n)$ であって 「$k = x - (i+j)$ とおいたとき $j < k \leq n$ を満たす」ものを数え上げればよい

### 計算量
$(1 \leq i < j < n)$ を満たす $i,j$ を全て調べ、
各 $i,j$ については（$i,j$ によらない）定数時間で判定することから $O(n^2)$ となる。

### データ構造と入出力
$n$, $x$ および回答は全て正の整数であるから `usize` で受け取る。
回答もまた正の整数であり、その大きさは $#(\{1..=n\}^3) = n^3$ を超えないから、 `usize` を用いる。
`(usize, usize) -> usize` なる関数を作る。

## 実装 1
### 実装方針
よく知っているやり方として、for 文で数え上げる方法をまず使った。
また、入出力についても、やや特殊であるため自分で書いてみた。

### コード
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

### 検証
- 入出力について
    - 入力を考えると、`tuple[0], tuple[1]` は必ずアクセスできるのでよい
    - 入力の終わりには必ず `0 0` が入りそれ以外では `0 0` がないので、 loopを必ず正しい位置で抜けるはず
- 本体について
    - `x - (i + j)` は usize の引き算なので場合によってはエラーになる
    - `k > i` は $i < j$ から判定する必要がない

### 改善点
- `x - (i+j)` を行う前に検査する
- 判定を減らす

## 実装 2
### 実装方針
実装 1 における改善点を改善する。

### コード
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

### 検証
- 変数 count が変化するのは6行目の if 文の中だけで、その条件を見れば確かに正しい。

## 実装 3
### 実装方針
上の書き方はよくあると思うけど、もうちょっと別の書き方に挑戦した。
iterator を用いて $i,j$ を列挙する部分と条件を満たす $i,j$ を検査する部分にわける。
変数 `count` を用いずに、実際に count を行う標準ライブラリを用いる。

### コード
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

### 検証
- 特になし