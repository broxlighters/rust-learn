# 第 3 阶段：集合、模块与错误处理

这一阶段的目标只有一个：开始写更像“项目”的 Rust 代码，而不只是单个语法示例。

## 这一阶段要学什么

- `Vec`
- `HashMap`
- 模块拆分
- `use`
- `panic!`
- `Result<T, E>`
- `?` 运算符
- 文件读写

## 这一阶段先不追求什么

- Trait 设计
- 泛型抽象
- 生命周期深入
- 并发与异步

当前重点是先把“数据怎么组织、代码怎么拆、错误怎么往上返回”练顺手。

## 学习顺序

建议按下面顺序推进：

1. 先跑 `stage03_01_vec`
2. 再跑 `stage03_02_hashmap`
3. 然后跑 `stage03_03_modules`
4. 再看 `stage03_04_result_and_question`
5. 最后做 `stage03_todo_cli`

## 目录说明

- `checklist.md`：本阶段完成标准
- `notes-template.md`：每天记录学习笔记
- `projects/`：本阶段重点项目说明

配套的可运行代码建议放在仓库的 `src/stage03/` 下：

- `src/stage03/01_vec.rs`
- `src/stage03/02_hashmap.rs`
- `src/stage03/03_modules.rs`
- `src/stage03/04_result_and_question.rs`
- `src/stage03/todo_cli.rs`

命名规则统一为：

- `src/stage{阶段号}/{顺序号}_{主题}.rs`
- 例如：`src/stage03/01_vec.rs`
- 每新增一个练习，同时在 `Cargo.toml` 增加一个 `[[bin]]`

## 如何使用

运行某个练习文件：

```bash
cargo run --bin stage03_01_vec
cargo run --bin stage03_02_hashmap
cargo run --bin stage03_03_modules
cargo run --bin stage03_04_result_and_question
cargo run --bin stage03_todo_cli -- list
```

建议方式：

1. 先运行一遍，看输出。
2. 再自己修改集合里的数据，观察结果怎么变化。
3. 看见 `Result` 时，先问自己：这里失败后应该怎么处理？
4. 做 Todo 项目时，先把最小可运行版本做出来，再整理函数和模块。

## 本阶段建议练习方式

这一阶段一定要多做 4 件事：

1. 先把集合用顺手：能创建、修改、遍历。
2. 每写一个可能失败的操作，先判断它该 `panic!` 还是返回 `Result`。
3. 文件读写先追求流程打通，不要一开始设计太复杂的数据格式。
4. 代码一变长，就主动拆函数、拆模块，而不是全堆在 `main` 里。

## 重点项目

`[重点项目]` 命令行 Todo 程序

项目要求：

- 支持新增任务
- 支持查看任务列表
- 支持完成任务
- 支持删除任务
- 支持把数据保存到本地文件

这个项目重点训练：

- `Vec<Task>`
- 模块组织
- `Result` 和 `?`
- 文件持久化
- 命令行参数处理

## 每天建议节奏

- 20 分钟：看一个集合或错误处理知识点
- 20 分钟：改一个可运行示例
- 20 分钟：推进 Todo 项目一个小功能
- 10 分钟：写笔记

## 完成标准

当你能做到下面几点，就可以进入第 4 阶段：

- 能用 `Vec` 管理一组结构化数据
- 能用 `HashMap` 做基本查找或统计
- 能把代码拆到模块里并用 `use` 导入
- 能区分 `panic!` 和 `Result` 的适用场景
- 能用 `?` 传播基础错误
- 能独立完成带本地持久化的命令行 Todo 程序
