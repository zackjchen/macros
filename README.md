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
2. 导入必要的lib
3. Ident和Type是大多时候可以互换，但更建议用Type信息更全
4. 使用范型时，不要带尖括号

```toml
[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1"
quote = "1" // 生成TokenStream的
syn = "2" // 解析TokenStream的
darling = "" // extract data from DeriveInput
```

```rust
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // 。。。。。省略1w行代码
    // quote return proc-macro2::TokenStream, so we need to convert it to TokenStream
    quote!{
        // #用来插入变量
        // #()* 类似$()，表示重复0次或多次，他会自动展开可迭代的内容
        // 需要注意的是，这里的 iter 必须是一个实现了 IntoIterator trait 的类型，
        // 这样 quote 宏才能对其进行迭代和展开。如果 iter 是一个迭代器，那么它会被展开。
        #( #from_impls )*
    }
}
```
