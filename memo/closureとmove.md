## 1
closure に move をつけたときよくわからない動作をしたので記録。
```Rust
fn a(a: &[(usize, usize)]) {
    for &x in a {
        for &y in a {
            /* ... */
        }
    }
}
```
と書いていたものを flat_map 等を使って書きたくなった

```Rust
a.iter()
.flat_map(|i|{
    a.iter().map(|j| (i,j))
})
.for_each(|_|{/*...*/})
```
だと、内部の map において
```
closure may outlive the current function, but it borrows `i`, which is owned by the current function
may outlive borrowed value `i`rustcE0373
main.rs(22, 26): `i` is borrowed here
main.rs(22, 9): closure is returned here
main.rs(22, 22): to force the closure to take ownership of `i` (and any other referenced variables), use the `move` keyword: `move `
```
のようにエラーが出る。
"current function" と呼ばれている（ map のこと？）が `i` を所有しているのだが、 closure （ `|j| (i,j)` ）が `i` を借用していてその関数よりも長く生きる可能性がある。
のでだめらしい。
指示に従い closure を `move |j| (i,j)` に書き換えると確かに通る。

- `i` の所有者は `a` ではないのか？
    - `i` を closure に所有させると `a` の所有権を
- closure がなぜ map よりも長く生きるのか？

考えたこととして、`i` の型が `&(usize, usize)` であることを踏まえると、借用もまた所有することができるのではないか。
すなわち、 `a.iter().flat_map(|i|{/*.../*})` において `i` には借用権が入っていて、この権利を明示的に `.map(/*...*/)` に渡す必要があるのではないか。
