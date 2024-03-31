# i
- `let t[j in 0..m] = min ({ i in 0..n | s[i][j] } or {n} )`
求めるものを数列にできて、
- `a[j in 0..m] s.t. a[j]-1 >= a[j+1] for j in 0..m-1 && a[j] >= t[j] for j in 0..m && a[j] in 0..=n`
- `n[j in 1..=m][i in 0..=n] = num of { a[0..j] | a[j-1] = i }`
- => `ans = sum_{i in 0..=n} n[m][i]`
再帰的に次のようにできる。
- 便宜上 `n[0][n] = 1, n[0][i in 0..n] = 0` とする。
- `n[(j in 0..m) + 1][i in 0..=n] = if i <= t[j] { sum_{i' in i -* 1..=n} n[j][i'] } else 0`

これを愚直に計算すると、 $O(n^2m)$ になる。
累積和のテクニックを使って減らす。
- `dp[j in 0..=m][i in 0..=n] = sum_{i' in i..=n} n[j][i']` 
- => `ans = dp[m][0]`
- `dp[0][i in 0..=n] = 1`
- `dp[(j in 0..m) + 1][i in t[j]+1..=n] = 0`
- `dp[(j in 0..m) + 1][i in 0..=t[j]] = dp[j+1][i+1] + dp[j][i -* 1]`
