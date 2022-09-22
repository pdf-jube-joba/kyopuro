# max や min などの計算をどう書くか？
何らかの collection （だいたい配列をいじってでてくる iterator ）の最大値を計算するときに、
```Rust
let mut max = 0;
for i in 0..n {
    ...
    max = std::cmp::max(max, a);
}
max
```
のように書く場合と
```Rust
(0..n)...
...
.max()
```
のように書く場合がある。
（究極的には個別に考えたほうが良いとは思うが）どちらにしたほうが良いのか。