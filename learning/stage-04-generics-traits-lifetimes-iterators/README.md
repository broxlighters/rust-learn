# 第 4 阶段：泛型、Trait、生命周期、迭代器

这一阶段的目标是开始读懂更像“真实 Rust 项目”的代码，而不是只会写语法片段。

## 这一阶段要学什么

- 泛型函数和泛型结构体
- Trait 与共享行为抽象
- `impl Trait`
- 生命周期的基础含义
- 迭代器与闭包
- 给核心逻辑写基础测试

## 这一阶段先不追求什么

- 生命周期的高级标注技巧
- 复杂 Trait 设计与关联类型
- 宏、异步、并发
- 追求“零拷贝到极致”的代码

当前重点是先建立三种感觉：

1. 为什么很多函数返回的是借用而不是新字符串。
2. 为什么 Trait 能让不同类型共享一套调用方式。
3. 为什么迭代器链经常比手写循环更清晰。

## 学习顺序

建议按下面顺序推进：

1. 先跑 `stage04_01_generics`
2. 再跑 `stage04_02_traits`
3. 然后跑 `stage04_03_lifetimes`
4. 再看 `stage04_04_iterators`
5. 最后做 `stage04_mini_grep`

## 目录说明

- `checklist.md`：本阶段完成标准
- `notes-template.md`：每天记录学习笔记
- `projects/`：本阶段重点项目说明

配套代码建议放在 `src/stage04/`：

- `src/stage04/01_generics.rs`
- `src/stage04/02_traits.rs`
- `src/stage04/03_lifetimes.rs`
- `src/stage04/04_iterators.rs`
- `src/stage04/mini_grep.rs`

## 如何使用

运行知识点示例：

```bash
cargo run --bin stage04_01_generics
cargo run --bin stage04_02_traits
cargo run --bin stage04_03_lifetimes
cargo run --bin stage04_04_iterators
```

运行重点项目：

```bash
cargo run --bin stage04_mini_grep -- frog stage04_poem.txt
$env:IGNORE_CASE=1
cargo run --bin stage04_mini_grep -- body stage04_poem.txt
```

运行测试：

```bash
cargo test --bin stage04_mini_grep
```

## 这一阶段的练习方式

建议每次练习都做这 4 件事：

1. 先读函数签名，判断它是“拿走所有权”还是“借用数据”。
2. 看到 `impl Trait` 或 Trait bound，先问自己“这里到底要求了什么能力”。
3. 看到返回引用的函数，先问自己“返回值和谁的生命周期绑定”。
4. 写完搜索或过滤逻辑后，至少补一个测试，别只手动跑。

## 重点项目

`[重点项目]` mini-grep

项目要求：

- 从文件中搜索关键字
- 支持大小写敏感搜索
- 支持大小写不敏感搜索
- 支持从命令行传参
- 输出匹配到的行
- 给搜索核心逻辑补测试

这个项目重点训练：

- `Config` 结构体设计
- `Result` 与错误返回
- 借用与生命周期
- `Iterator` + `filter`
- 基础单元测试

## 完成标准

当你能做到下面这些点，就可以进入下一阶段：

- 能写出一个泛型函数并解释 Trait bound 的作用
- 能自己定义一个 Trait 并为两个类型实现它
- 能解释 `longest<'a>(...) -> &'a str` 在表达什么关系
- 能读懂常见的迭代器链：`iter / map / filter / collect`
- 能独立完成一个可运行、可测试的 mini-grep
