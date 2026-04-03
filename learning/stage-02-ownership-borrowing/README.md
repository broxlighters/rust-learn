# 第 2 阶段：所有权、借用、切片

这一阶段的目标只有一个：真正理解 Rust 和其他语言最不一样的地方，开始建立“为什么这段代码不能这么写”的感觉。

## 这一阶段要学什么

- ownership
- borrowing
- mutable / immutable reference
- `String` 和 `&str`
- slice
- move、copy、clone 的区别
- 函数参数中的所有权转移和借用

## 这一阶段先不追求什么

- 生命周期的细节
- Trait 设计
- 泛型抽象
- 并发与异步

当前重点不是写很复杂的项目，而是把“值到底归谁所有”这件事练明白。

## 学习顺序

建议按下面顺序推进：

1. 先跑 `stage02_01_ownership`
2. 再跑 `stage02_02_borrowing`
3. 然后跑 `stage02_03_references_and_mut`
4. 再看 `stage02_04_slice`
5. 最后做本阶段重点项目

## 目录说明

- `checklist.md`：本阶段完成标准
- `notes-template.md`：每天记录学习笔记
- `projects/`：本阶段训练题和重点项目说明

配套的可运行代码建议放在仓库的 `src/stage02/` 下：

- `src/stage02/01_ownership.rs`
- `src/stage02/02_borrowing.rs`
- `src/stage02/03_references_and_mut.rs`
- `src/stage02/04_slice.rs`
- `src/stage02/guess_number_plus.rs`

命名规则统一为：

- `src/stage{阶段号}/{顺序号}_{主题}.rs`
- 例如：`src/stage02/01_ownership.rs`
- 每新增一个练习，同时在 `Cargo.toml` 增加一个 `[[bin]]`

## 如何使用

运行某个练习文件：

```bash
cargo run --bin stage02_01_ownership
cargo run --bin stage02_02_borrowing
cargo run --bin stage02_03_references_and_mut
cargo run --bin stage02_04_slice
cargo run --bin stage02_guess_number_plus
```

建议方式：

1. 先运行一遍，看输出。
2. 再故意改错，观察编译器报错。
3. 自己解释每个报错为什么出现。
4. 最后不看参考，自己重写一遍。

## 本阶段建议练习方式

这一阶段一定要多做两件事：

1. 猜代码是否能编译，再运行验证。
2. 看见报错后，不要急着改，先自己说出“所有权现在在谁手里”。

你会发现，很多 Rust 报错本质上都在问同一件事：

- 值是不是已经被移动了？
- 现在拿到的是所有权，还是借用？
- 当前作用域里能不能同时存在可变和不可变借用？

## 重点项目

`[重点项目]` 猜数字游戏升级版

项目要求：

- 保留已有的猜数字游戏基础玩法
- 支持多轮游戏
- 支持重新开始
- 对非法输入做处理
- 统计猜对次数和总局数
- 尽量把逻辑拆成多个函数

这个项目重点训练：

- `String` 输入处理
- `&str` 参数传递
- 函数拆分时的借用设计
- `match` 和 `Result` 的基础组合使用
- 可变状态在循环中的管理

## 每天建议节奏

- 20 分钟：看一个所有权/借用知识点
- 20 分钟：改一个报错示例
- 20 分钟：做一个小练习
- 10 分钟：写笔记

## 完成标准

当你能做到下面几点，就可以进入第 3 阶段：

- 能解释 move 为什么发生
- 能区分传值和传引用
- 能解释为什么同一时刻不能同时有可变借用和不可变借用
- 能看懂 `String` 和 `&str` 的常见函数参数设计
- 能使用 slice 处理字符串片段
- 能独立完成猜数字游戏升级版
