# Rust Visualizer MVP

**版本**: 0.1.0 (MVP)  
**日期**: 2026-06-09  

一个用于可视化 Rust 代码变量所有权关系的工具。

## 功能特性

- 解析 Rust 源代码
- 变量声明检测
- 作用域分析
- 基础所有权分析

## 使用方法

```bash
cargo run -- analyze <input_file.rs>
```

## 项目结构

```
rust_visualizer/
├── src/
│   ├── main.rs
│   ├── analysis/
│   │   └── ownership.rs
│   └── graph/
│       └── mod.rs
├── examples/
├── Cargo.toml
└── README.md
```

## 许可证

MIT License