## とりあえずメモ
HashMap を使う方法と、 crate: enum-map を使う方法と、 trait: Index を実装する方法がある。

`dp[(A,B)][i]` で「i 日目が A」で「i+1 日目が B」となり「i+1 日目まではすでに定まった予定に合致した」予定が何通りあるかをあらわす
- `i` の範囲は `0..n-1`
- `dp[(A,B)][0]` = 「`s[0]`が`Some(already)`で`already != A`」 または 「`s[1]`が`Some(already)`で`already != B`」なら`0`それ以外は`1`
- `dp[(A,B)][i+1]` = 「`s[A]`が`Some(already)`で`already != A`」 または 「`s[B]`が`Some(already)`で`already != B`」なら`0`
    - それ以外のとき、各 `C` に対して 「`C==A&&A==B` なら `0` それ以外は `dp[(C,A)][i]` 」

これを実際には判定する必要のない部分等をけずればよい。

今回は vec を option ではなく、 push を使って dp を伸ばしていく方法をとる。

添え字の問題がすごいうざい
なにでうけとるかという問題や何を意味しているかをしっかりと考えたほうが良い。