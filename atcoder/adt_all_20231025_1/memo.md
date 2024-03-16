# g
数式とか添え字に弱すぎる。
あと公式の解説だと、 $R_{i,0} = 0$ にならないので式が間違っている？
- def: $num[i in 0..=n][j in 0..=M] = \#{ c[0..i] | a[k] <= c[k] <= b[k] for k in 0..i && c[i-1] == j }$
    - $num[0] = [1,0,0,...]$
    - $num[i+1][j] = if a[i] <= j <= b[i] then sum_{0 <= k <= j} num[i][k] else 0$
- def: $dp[i in 0..=n][j in 0..=M] = sum_{0 <= k <= j} num[i][k]$
    - dp[0] = [1,1,1,...]
    - dp[i][j+1] - dp[i][j] = num[i][j]