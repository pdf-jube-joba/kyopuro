# B
以下と未満をうまく使った方がいいらしい
あと、 $ \lfloor x/M \rfloor $ を関数として定義しておいてテスト可能にするのがいい？

前
```
fn compute(a: isize, m: usize, l: isize, r: isize) -> usize {
    // min k of l <= a + km <=> (l - a) <= km
    let left_most: isize = {
        let l = l - a;
        if l <= 0 {
            -((-l) / (m as isize))
        } else {
            (l - 1) / (m as isize) + 1
        }
    };

    // max k of a + km <= r <=> km <= (r - a)
    let right_most: isize = {
        let r = r - a;
        if r < 0 {
            -((-r - 1) / (m as isize) + 1)
        } else {
            r / (m as isize)
        }
    };

    if right_most < left_most {
        0
    } else {
        (right_most - left_most) as usize + 1
    }
}
```
確かに場合分けがめんどくさくて大変だった。

div_euclid を使うのがよい。

```
```