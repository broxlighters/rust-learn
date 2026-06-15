# 第 4 阶段 Checklist

把下面内容做完，再进入下一阶段。

## 泛型

- [ ] 会写泛型函数
- [ ] 会给泛型参数加 Trait bound
- [ ] 会写泛型结构体和泛型 `impl`

## Trait

- [ ] 知道 Trait 是“共享行为”的抽象
- [ ] 会为不同类型实现同一个 Trait
- [ ] 看得懂 `&impl Trait`
- [ ] 看得懂基础的 `T: Trait`

## 生命周期

- [ ] 知道生命周期不是“让引用活更久”，而是描述关系
- [ ] 能读懂 `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`
- [ ] 知道为什么返回引用时经常需要生命周期标注

## 迭代器与闭包

- [ ] 会区分 `iter`、`into_iter`、`iter_mut`
- [ ] 会用 `map`
- [ ] 会用 `filter`
- [ ] 会用 `collect`
- [ ] 能读懂闭包 `|item| ...`

## 测试

- [ ] 会写 `#[test]`
- [ ] 会断言 `Vec<&str>` 之类的结果
- [ ] 知道为什么搜索逻辑适合先写测试

## 实战练习

- [ ] 完成 `01_generics.rs`
- [ ] 完成 `02_traits.rs`
- [ ] 完成 `03_lifetimes.rs`
- [ ] 完成 `04_iterators.rs`
- [ ] 完成 `mini_grep.rs`

## 自测问题

- [ ] 泛型和 Trait 分别解决什么问题
- [ ] `impl Trait` 和具体类型返回值有什么差别
- [ ] 生命周期在这里描述的是谁和谁的关系
- [ ] 为什么 `search` 返回 `Vec<&str>` 而不是 `Vec<String>`
- [ ] 为什么 `mini-grep` 的搜索核心值得写测试
