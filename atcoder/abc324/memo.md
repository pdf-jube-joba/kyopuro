# e 
解説では prefix と suffix のうち、前計算を行うのを suffix のみに絞っていた。
自分のほうでは prefix をとったら TLE した。
確かに計算量的には $O(\lvert T \rvert)$ になるのでTLEになる理由はわかるが解説だとTLEしない理由が書かれている。
それを読むと確かにそうだと思えるのでそこについてメモをとる。

## 前提

ここでは部分文字列は連続していないものを含む。

- $T$: 文字列 と $i \in 0..n$ ごとに $S_i$: 文字列が与えられる。
- $(i,j)$ であって $S_i + S_j$ が $T$ を部分文字列として含むものを数えろ。

制約としては $\lvert T \rvert$, $n$, $\lvert S_i \rvert$ のほかに、 $\sum_i \lvert S_i \rvert$ もある。

## 定義
- $\text{pre}_i$: $\mathbb{N}$ := $S_i$ が含む最長の $T$ の接頭辞の長さ
- $\text{suf}_i$: $\mathbb{N}$ := $S_i$ が含む最長の $T$ の接尾辞の長さ
- $(i,j)$: $S_i + S_j$ が $T$ を部分文字列として含む $\iff$ $\text{pre}_i + \text{suf}_j \geq \lvert T \rvert$
- 前計算として $\text{SUF}_l$ = $\{S_j \mid \text{suf}_j = l\}$ を計算する（これは $O(\sum_{i} \lvert S_i \rvert)$ でできる。）
    - 注意点として、 $\text{SUF}_l$ は $0 \leq l \leq \lvert T \rvert$ の範囲で用意する必要がある。（ $\lvert T \rvert$ に注意！）

ここまでは共通である。

- また、私は前計算として $\text{PRE}_l$= $\{S_j \mid \text{pre}_j = l\}$ を計算した。（これも $O(\sum_{i} \lvert S_i \rvert)$ でできる。）

## 回答の差

解説では次のように計算を行った。
求めるものは次のようになる。
$$\# \{(i,j) \mid \lvert T \rvert \leq \text{pre}_i + \text{suf}_j\}
\\= \# \{ \{j \mid \text{suf}_j \geq \lvert T \rvert - \text{pre}_i\} \mid i \in 0..n\}
\\= \# \{ \bigcup_{\lvert T \rvert - \text{pre}_i \leq l \leq \lvert T \rvert} \text{SUF}_l \mid i \in 0..n\}
\\= \sum_{i \in 0..n} \sum_{\lvert T \rvert - \text{pre}_i \leq l \leq \lvert T \rvert} \text{SUF}_l$$
ここでこの項の足し算が何回行われるかというと、
$$ \sum_{i \in 0..n} \sum_{\lvert T \rvert - \text{pre}_i \leq l \leq \lvert T \rvert} 1 = \sum_{i \in 0..n} (\text{pre}_i + 1) = (\sum_{i} \lvert S_i \rvert) + n$$

一方で私は別のやり方をした。
求めるものは次のようになる。
$$\# \{(i,j) \mid \lvert T \rvert \leq \text{pre}_i + \text{suf}_j\} = \cdots
\\= \# \bigcup_{k + l \leq \lvert T \rvert} \{i \mid \text{pre}_i = k\} \times \{j \mid \text{suf}_j = l\}
\\= \sum_{k + l \geq \lvert T \rvert} \#(\text{PRE}_k \times \text{SUF}_l)
\\= \sum_{0 \leq k \leq n} \sum_{\lvert T \rvert - k \leq l \leq \lvert T \rvert} \# \text{PRE}_k * \# \text{SUF}_l$$

事前計算で $\#\text{PRE}_i, \# \text{SUF}_i$ はわかっているのでこの計算量は次のようになる。

$\sum_{0 \leq k \leq n} \sum_{\lvert T \rvert - k \leq l \leq \lvert T \rvert} 1 = \sum_{0 \leq k \leq n} (k + 1) = \frac{1}{2}n(n+1) + n$

この 2 つの解法の差は、一方は $\sum_{i} \text{pre}_i$ の計算が出てくるのが $\sum_i \lvert S_i \rvert$ に変換され、もう一方は $\sum_{k} k$ が出てきたところにある。

後者はTLEした。
（ $0 \leq n \leq 5 \times 10^5$ なので、 $O(N^2)$ はきつい。）
前者はTLEしない。
（ $0 \leq \sum_i \lvert S_i \rvert \leq 5 \times 10^5$ なので $O(\lvert \sum_i S_i \rvert)$ は通る。）


