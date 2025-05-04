# Rust 学习代码

这个仓库包含了我学习 Rust 编程语言过程中编写的一系列代码示例。每个子文件夹代表一个独立的 Rust 项目，展示了不同的 Rust 语言特性和概念。

## 项目结构

- hello_cargo: 第一个 Rust 项目，展示了 Cargo 的基本使用
- hello-rust: 使用外部依赖库 (ferris_says) 的简单程序
- variables: 演示变量声明、可变性、常量和变量遮蔽
- functions: 函数定义、参数传递和返回值示例
- control-flow: if 表达式、loop、while 和 for 循环等控制流结构
- guessing_game: 一个简单的猜数字游戏，使用了 rand 外部库
- what-is-ownership: 展示 Rust 的所有权系统核心概念
- references-and-borrowing: 引用和借用的使用方法
- slices: 字符串和其他类型的切片示例
- structs: 使用结构体组织相关联的数据
- enums: 枚举和模式匹配
- defining-modules: 包、crates、模块、路径
- common-collections: 常见集合
- error-handing: 错误处理
- generics: 泛型，trait

## 如何运行

每个项目都可以单独编译和运行：

```bash
cd <项目名称>
cargo run
```

## 学习资源

这些代码示例主要基于[《Rust 程序设计语言》(The Rust Programming Language)](https://doc.rust-lang.org/book/) 这本书编写，是学习 Rust 基础概念的实践代码。

## 环境信息

Rust 版本: 1.86.0  
操作系统: macOS (aarch64-apple-darwin)
