# Rust Visualizer

**版本**: 1.0.0  
**日期**: 2026-06-08  

一个用于可视化 Rust 代码变量所有权关系的工具。

## 功能特性

- ✅ Rust 源代码解析
- ✅ 变量声明检测
- ✅ 作用域分析
- ✅ 完整所有权分析
- ✅ 借用分析（不可变/可变）
- ✅ 所有权移动分析
- ✅ 生命周期分析
- ✅ 未使用变量检测

## 使用方法

```bash
# 分析 Rust 文件
cargo run -- analyze <input_file.rs>

# 显示帮助信息
cargo run -- --help
```

## 分析结果输出

工具会输出：
- 变量总数
- 使用/未使用变量统计
- 所有权状态变化
- 借用关系分析

## 项目结构

```
rust_visualizer/
├── src/
│   ├── main.rs
│   ├── analysis/
│   │   ├── mod.rs
│   │   └── ownership.rs
│   ├── graph/
│   │   └── mod.rs
│   └── parser/
│       └── mod.rs
├── examples/
├── Cargo.toml
└── README.md
```

## 许可证

MIT License