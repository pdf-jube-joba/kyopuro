# 問題
前提
- $s$: 英大文字からなる文字列
---
求めるもの
- $\text{max} \{ \text{\(t\) の長さ} \mid \text{\(t\) は \(s\) の部分文字列で \texttt{A}, \texttt{C}, \texttt{G}, \texttt{T} のみを含む} \}$
---
制約
- $1 \leq \text{length of \(S\)} \leq 10$
---
注意
- 部分文字列は先頭と末尾から $0$ 以上を取り去って得られる文字列のこと

# 方針
- 自分で書いたものは、前から文字列を読んでゆきこれまで何個連続して出てきたかと合わせて最大をだした。
- 公式の解説では $S$ の部分文字列をすべて列挙して調べる方法だった

# 実装のメモ
とりあえず文字列を `ACGT` かそれ以外の文字かで判定してしまったものつくることにした。
```Rust
let b: Vec<bool> = s.chars().into_iter().map(|c|{
    match c {
        'A' | 'T' | 'C' | 'G' => true,
        _ => false
    }
}).collect();
```
この `b` は `b[i]` <=> `s[i]` が `ACGT` のように定義されている。

## 1
自分の初めの方法
```Rust
let mut count = 0;
let mut max = count;
b.iter().for_each(|f|{
    if *f {count += 1;} else {count = 0;}
    if count > max {max = count;}
});
max
```
`b.iter().for_each(|b|{...})` では（ `i` で `b` が何番目かを表すと）
- count は「現在何個 `ATCG` が連続で出続けているか」、すなわち `b[i-count+1..=i]` が `ATCG` で構成されている最大の `count` をあらわしている。（そのような count がないときは 0）
- max は `b[..i]` の(...略...)の最大値をあらわしている

各 `b` を処理するごとにこの条件が保たれているはずなのでよい。

## 2
公式の解説に従う場合の考察
- 部分文字列には空の文字列もあり、部分文字列を列挙するには $i \leq j$ により `s[i..=j]` を調べるだけだと不十分になる。
    - この文字列の長さは `j - i + 1` である
    - 空の文字列の場合を特別扱いし、 `let mut max = 0` から始める方法がある
- $i \leq j$ で `s[i..j]` を調べる場合、 $j = i$ の場合に空の文字列が現れるためすべての文字列を得ることができる（重複はある）
    - この文字列の長さは `j - i`

今回は後者を用いた。

```Rust
let n = b.len();
(0..=n)
.flat_map(|i|{
    (i..=n).map(move |j| (i, j))
})
.filter_map(|(i,j)|{
    if b[i..j].iter().all(|&b|b) {Some(j - i)} else {None}
})
.max().unwrap()
```
- `flat_map` の後の collection は $(i,j)$ : $ 0 \leq i \leq j \leq n$ をすべて列挙している
    - 空の文字列を含め、すべての部分文字列は `s[i..j]` によって表現できる
    - そのような $(i,j)$ のうち、 `b[i..j]` がすべて `true` となるものはその長さを述べ、そうでないものはなかったものとして無視する。
    - 添え字のつけ方から `b[i..j]` はアクセスしてよい。
    - $i,j$ の条件から $j-i$ は計算してよい。
- このような長さの collection から最大値を探し出す。
    - $i = j$ のとき空の文字列に対応する部分文字列が得られ、 `b[i..j].iter().all(|&b|b)` は必ず true になるから `filter_map` したあとの collection は 空でない。特に `.max().unwrap()` してよい。

for 文を使ったほうが短くてわかりやすいかも
```Rust
let n = b.len();
let mut max = 0;
for i in 0..=n {
    for j in i..=n {
        if b[i..j].iter().all(|&b|b) {max = std::cmp::max(max, j - i);}
    }
}
max
```
これも正しさの確認は同様にしてできる（ max が for 文を回る中でどういう条件を保っているかは別途気を付けたほうが良い）

# 計算量
自分の実装方法は `b` を一周しているだけなので $O(n)$ だと思う。
公式のほうは for 文を二重にして if 文の判定でさらに配列を $i$ から $j$ までとアクセスしているため、  $O(n^3)$ になりそう。