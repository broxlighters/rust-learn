# 第 1 阶段：语法基础与控制流

这一阶段的目标只有一个：把 Rust 的基础语法写顺手，能独立完成简单的小程序。

## 这一阶段要学什么

- 变量、常量、基本类型
- 函数、表达式、返回值
- `if`、`match`、`loop`、`while`、`for`
- 数组、元组、结构体、枚举
- `Option` 和 `Result` 的基本用法

## 这一阶段先不追求什么

- 所有权和借用的深水区
- 生命周期
- Trait 设计
- 并发与异步

这些内容后面再学，当前重点是先把“写得出来”建立起来。

## 学习顺序

建议按下面顺序推进：

1. 先跑 `stage01_01_variables`
2. 再跑 `stage01_02_functions`
3. 然后跑 `stage01_03_control_flow`
4. 再看 `stage01_04_struct_enum`
5. 最后跑 `stage01_05_option_result`
6. 做 3 个小项目题

## 目录说明

- `checklist.md`：本阶段完成标准
- `notes-template.md`：每天记录学习笔记
- `projects/`：3 个阶段小项目题

配套的可运行代码在仓库的 `src/bin/` 下：

- `stage01_01_variables/main.rs`
- `stage01_02_functions/main.rs`
- `stage01_03_control_flow/main.rs`
- `stage01_04_struct_enum/main.rs`
- `stage01_05_option_result/main.rs`

命名规则统一为：

- `stage{阶段号}_{顺序号}_{主题}/main.rs`
- 例如：`stage02_01_ownership/main.rs`
- 这样后续新增阶段时不需要改 Cargo 配置

## 如何使用

运行某个练习文件：

```bash
cargo run --bin stage01_01_variables
cargo run --bin stage01_02_functions
cargo run --bin stage01_03_control_flow
cargo run --bin stage01_04_struct_enum
cargo run --bin stage01_05_option_result
```

建议方式：

1. 先运行一遍，看输出。
2. 再逐行改代码，看结果怎么变化。
3. 最后自己重写一遍，不看参考。

## 每天建议节奏

- 20 分钟：看一个知识点
- 20 分钟：改一个示例
- 20 分钟：做一个小练习
- 10 分钟：写笔记

## 完成标准

当你能做到下面几点，就可以进入第 2 阶段：

- 不看资料写出基本变量、函数和控制流
- 看懂 `match` 的基本分支写法
- 能区分数组、元组、结构体、枚举
- 知道什么时候用 `Option`
- 知道什么时候用 `Result`
- 能独立完成温度转换器、斐波那契、简单计算器
