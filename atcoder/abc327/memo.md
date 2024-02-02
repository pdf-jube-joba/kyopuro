# e
dp の添え字の解釈いつも間違うな
- $dp[i in 1..=n][j in 1..=i]       = max {sum_{k in 1..=j} (0.9)^(j-k) p[l_k]              | 1 <= l_1 < ... < l_j <= i }$      // $p[1..=n]$
- $dp[i in 1..=n][j in 1..=i]       = max {sum_{k in 1..=j} (0.9)^(j-k) p[l_k - 1]          | 1 <= l_1 < ... < l_j <= i }$      // $p[1..n]$ へ
- $dp[i in 1..=n][j in 1..=i]       = max {sum_{k in 1..=j} (0.9)^(j-k) p[l_k]              | 0 <= l_1 < ... < l_j < i }$       // $l_k$ ずらす 
- $dp[i in 0.. n][j in 1..=(i+1)]   = max {sum_{k in 1..=j} (0.9)^(j-k) p[l_k]              | 0 <= l_1 < ... < l_j <= i }$      // i -> i + 1
- $dp[i in 0.. n][j in 0..=i]       = max {sum_{k in 1..=(j+1)} (0.9)^((j+1)-k) p[l_k]      | 0 <= l_1 < ... < l_(j+1) <= i }$  // j -> j + 1
- $dp[i in 0.. n][j in 0..=i]       = max {sum_{k in 0..=j} (0.9)^(j-k) p[l_(k+1)]          | 0 <= l_1 < ... < l_(j+1) <= i }$  // k -> k + 1
- $dp[i in 0.. n][j in 0..=i]       = max {sum_{k in 0..=j} (0.9)^(j-k) p[l_k]              | 0 <= l_0 < ... < l_j <= i }$      // $l_{**}$ ずらす

- $max_{1 \leq j \leq n}    dp[n-1][j+1] / sum_{k=1..=j} (0.9)^(j-k) - 1200/ sqrt(j)$
- $max_{0 \leq j < n}       dp[n-1][j]   / sum_{k=1..=j+1} (0.9)^(j+1-k) - 1200/ sqrt(j+1)$
- $max_{0 \leq j < n}       dp[n-1][j]   / sum_{k=0..=j} (0.9)^(j-k) - 1200/ sqrt(j+1)$