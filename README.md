# Geektime Rust 语言训练营

## 知识点

1. 声明宏不区分阔号
```rust
let a = vec![1,2,3];
let b = vec!(1,2,3);
let c = vec!{1,2,3};
```
2. `$(repeat),+`, `$()`表示重复的模式,单独写不起作用，必须搭配`"?,*,+"`的一种，中间部分(这里是逗号)表示分割符



## 过程宏(derive)
1. 声明lib为proc-macro

```toml
[lib]
proc-macro = true
```
