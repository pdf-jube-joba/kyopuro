# 問題
前提
- $a$, $b$, $c$, $x$, $y$: 整数
---
求めるもの
- $\text{min} \{ al + bm + cn \mid x \leq l + \lfloor \frac{n}{2} \rfloor, y \leq m + \lfloor \frac{n}{2} \rfloor, \text{for \(l,m,n\)} \}$
---
制約
- $1 \leq a,b,c \leq 5000$
- $1 \leq x,y \leq 10^5$

# 方針
## 1
もし $n$ の個数を固定した場合、最小値を与える $l$, $m$ の値がわかる（ $x \dot{-} y$ で `if x > y {x-y} else {0}` とすると、 $l = x \dot{-} \lfloor \frac{n}{2} \rfloor, m = y \dot{-} \lfloor \frac{n}{2} \rfloor $ が最小値を与える。）
$n$ について全探索をすることを考える。
$n$ の範囲は $0$ から $l,m$ がともに $0$ になる $\text{max}(2x, 2y)$ を探索すれば最小値が見つかる。
（それよりも大きいと無駄が発生している。
## 2
上の考察から、 $n$ についての関数 $a (x \dot{-} \lfloor \frac{n}{2} \rfloor) + b (y \dot{-} \lfloor \frac{n}{2} \rfloor) + c n$ を最小化すればよいとわかるのでこれを考察する。
$n=2k+1$ のような奇数の場合は $n=2n$ のほうがこの値がより小さいから初めから偶数であるとしてよい。
そのため、 $f(n: \text{0以上の整数}) = a * (x \dot{-} n) + c * (y \dot{-} n) + 2 c * n $ を最小化すればよい。
それぞれの項の形を見れば、全体は区分線形のようになっているから、最小値はその方向の変化するところのみを調べればよい。
すなわち $f(0), f(x), f(y)$ をみればよい。

# 実装のメモ
最初のコミットで書いたものは間違っている。
## 1
一つ目の実装は次のようになる。
```Rust
let max = std::cmp::max(2*x,2*y);
(0..=max)
.map(|n|{
    let l = if x > (n/2) {x - (n/2)} else {0};
    let m = if y > (n/2) {y - (n/2)} else {0};
    a * l + b * m + c * n
})
.min()
.unwrap()
```
- subtract with overflow はその前の検査により発生しない
- unwrap は空でない iterator に対して行っているので安全

## 2
二つ目の実装は次のようになる。
```Rust
let f = |n|{
    let l = if x > n {x - n} else {0};
    let m = if y > n {y - n} else {0};
    a * l + b * m + c * (2 * n)
};
std::cmp::min(f(0), 
    std::cmp::min(f(x), f(y))
)
```

# 計算量
## 1
一つ目の実装は
$\{0..=\text{max}\}$ の各元に対して定数倍の処理をして $O(\text{max}(x,y))$ を使った後にその min を計算しているから全体としては $O(\text{max}(x,y))$ になる。

## 2
二つ目の実装は計算する値の候補の個数が初めから決まっているので $O(1)$ である。