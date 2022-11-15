# 問題

# その他
```Rust
vec![
    if 0 < now.0 {Some((now.0 - 1, now.1))} else {None},
    if now.0 < h-1 {Some((now.0 + 1, now.1))} else {None},
    if 0 < now.1 {Some((now.0, now.1 - 1))} else {None},
    if now.1 < w-1 {Some((now.0, now.1 + 1))} else {None},
].into_iter().flatten()
.filter(|&(i,j)| memo[i][j].is_none() )
.for_each(|(i,j)|{
    queue.push_back((i,j));
    memo[i][j] = Some(l + 1);
});
```
は通らない。
memo が `filter` と `for_each` がともに memo を使っているため。
途中で `.collect::<Vec<(usize, usize)>>().iter()` をはさむしかない？