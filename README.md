# MyRustLearningRoute

这是一个按主题拆分的 Rust 学习练习仓库。每个目录都是一个独立的 binary crate，
根目录通过 Cargo workspace 统一管理它们。

## 目录

| 路径 | 内容 |
| --- | --- |
| [`basic_concept/`](./basic_concept/) | 变量、函数和类型推导 |
| [`variables/`](./variables/) | 变量遮蔽与作用域 |
| [`caculation/`](./caculation/) | 数值运算 |
| [`caculation/bitwise/`](./caculation/bitwise/) | 位运算 |
| [`floatingpoint/`](./floatingpoint/) | 浮点数表示与精度 |
| [`loop_test/`](./loop_test/) | `for` 循环与字符范围 |
| [`world_hello/`](./world_hello/) | 函数、数组与 Unicode 字符串 |
| [`penguin/`](./penguin/) | 字符串解析与 `if let` |
| [`hw/10_28/prob4/`](./hw/10_28/prob4/) | 练习题 |
| [`notes.md`](./notes.md) | 学习笔记 |

## 常用命令

```bash
# 查看 workspace 中的所有 crate
cargo metadata --no-deps

# 检查全部练习
cargo check --workspace

# 运行指定练习
cargo run -p variables
cargo run -p bitwise

# 格式化并运行测试
cargo fmt --all
cargo test --workspace
```

编译产物统一写入根目录的 `target/`，不会提交到 Git。
