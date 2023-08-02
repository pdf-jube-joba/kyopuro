# 問題
$2 \leq n \leq 8$ なる整数に対して、 各 $i \in \{1..=n\}$ ごとに $p_i = (x_i, y_i)$ なる整数の組が与えられる。
各順列 $I = (i_1, \ldots, i_n)$ ごとに $r_I = \sum _{i=1}^{n-1} \lvert p_{i+1} - p_i \rvert$ なる値を考える。
$I$ を全ての順列で動かしたとき $r_I$ の平均を求めよ。

# 回答 1
## 方針
全ての順列に対して $r_I$ を計算し平均を計算する。

## 実装
itertools が atcoder では使えるようだが、今回は自分で順列の列挙を書いた。
順列の生成は辞書順に生成する方法があり

# 順列の生成
## 方針
順列の間に辞書による順序を入れることで、 $a_1, \ldots, a_n$ の次が次のように計算できる。
- $a_i < a_{i+1} $ を満たす最大の $i$ を求めて $i'$ とおく。もしそのような $i$ がない場合はこの順列が最大である。
- $a_{i'} < a_{j}$ を満たす最大の $j \in \{i'+1..\}$ を求めて $j'$ とおく。必ず存在する。
- $a_{i'}$ と $a_{j'}$ を交換する。
- $a_{i'}$ よりも後の要素を反転する。
これによりえられた $a$ が次の要素になる。

## 実装
```Rust
fn permutation(n: usize) -> impl Iterator<Item = Vec<usize>> {
    struct Perm {
        permutation: Vec<usize>,
    }
    impl Iterator for Perm {
        type Item = Vec<usize>;
        fn next(&mut self) -> Option<Self::Item> {
            let result = Some(self.permutation.clone());
            let n = self.permutation.len();
            let max_index = match (0..n-1).filter(|&i|{self.permutation[i] < self.permutation[i+1]}).max() {
                Some(i) => i,
                None => {return None;}
            };
            let swap_index = {
                (max_index+1..n).filter(|&i|{self.permutation[i] > self.permutation[max_index]}).max().unwrap()
            };
            { // swap a[max_index] and a[swap_index]
                let temp = self.permutation[max_index];
                self.permutation[max_index] = self.permutation[swap_index];
                self.permutation[swap_index] = temp;
            }
            self.permutation[max_index+1..n].reverse();
            return result;
        }
    }
    Perm {permutation: (0..n).collect()}
}
```