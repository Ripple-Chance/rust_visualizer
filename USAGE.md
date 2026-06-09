# Rust Visualizer 使用说明

**版本**: 2.0  
**更新日期**: 2026-06-09  

## 概述

Rust Visualizer 是一个用于可视化 Rust 代码变量所有权关系的工具。它可以分析 Rust 源代码，追踪变量的所有权状态变化，并生成可视化输出。

## 快速开始

### 安装

```bash
# 克隆仓库
git clone https://github.com/your-repo/rust_visualizer.git
cd rust_visualizer

# 构建项目
cargo build --release
```

### 基本用法

```bash
# 分析 Rust 文件
cargo run --release -- analyze <input_file.rs>

# 生成 DOT 文件
cargo run --release -- analyze <input_file.rs> --dot output.dot

# 生成 SVG 文件
cargo run --release -- analyze <input_file.rs> --svg output.svg

# 同时生成 DOT 和 SVG
cargo run --release -- analyze <input_file.rs> --dot output.dot --svg output.svg
```

## 命令行参数

```
rust_visualizer analyze <input> [--dot <output>] [--svg <output>]

位置参数：
  input               要分析的 Rust 源文件路径

选项：
  --dot <output>      生成 DOT 格式文件
  --svg <output>      生成 SVG 格式文件
  --help              显示帮助信息
```

## 使用示例

### 示例 1：分析示例文件

```bash
# 分析 examples 目录下的测试文件
cargo run --release -- analyze examples/test_scope.rs --dot examples/test_scope.dot --svg examples/test_scope.svg
```

### 示例 2：分析自定义文件

```bash
# 创建一个简单的 Rust 文件
echo 'fn main() { let x = 42; let y = x; }' > test.rs

# 分析并生成可视化
cargo run --release -- analyze test.rs --dot test.dot --svg test.svg
```

### 示例 3：查看分析摘要

```bash
cargo run --release -- analyze examples/test_borrow.rs
```

输出示例：
```
=== Variable Analysis Summary ===
Total variables: 5
Used variables: 5
Unused variables: 0

DOT file exported to: examples/test_borrow.dot
  Nodes: 5, Edges: 0
  Use 'dot -Tpng examples/test_borrow.dot -o output.png' to generate PNG
SVG file exported to: examples/test_borrow.svg
  Open this file in a web browser to view
```

## 输出文件说明

### DOT 文件

DOT 文件是标准的 Graphviz 格式文件，可以用于：

1. **在线查看**：复制内容到 [Graphviz Online](https://dreampuf.github.io/GraphvizOnline/)
2. **生成图像**：使用 Graphviz 命令行工具

```bash
# 使用 Graphviz 生成 PNG
dot -Tpng output.dot -o output.png

# 使用 Graphviz 生成 SVG
dot -Tsvg output.dot -o output.svg
```

### SVG 文件

SVG 文件可以直接在浏览器中打开查看，无需额外工具。

## 可视化效果说明

### 颜色编码

| 状态 | 颜色 | 说明 |
|------|------|------|
| Owned | 🟢 绿色 | 变量拥有所有权 |
| Moved | 🟠 橙色 | 变量已移动 |
| Borrowed(Immutable) | 🔵 蓝色 | 变量被不可变借用 |
| Borrowed(Mutable) | 🔴 红色 | 变量被可变借用 |
| Dropped | ⚪ 灰色 | 变量已销毁 |
| Unused | ⚪ 浅灰色 | 变量未被使用 |

### 作用域分组

变量会按作用域层级分组显示，每个作用域显示为一个带标题的浅蓝色框：

- Scope 2：顶层作用域
- Scope 3：函数内部作用域
- Scope 4：块作用域（如 if、loop、match 等）

### 图例

SVG 文件包含图例，解释各种颜色的含义：

- Legend：图例区域
- Statistics：统计信息（节点数、边数）

## 支持的分析功能

### 变量识别
- 识别 let 绑定定义的变量
- 识别函数参数
- 识别结构体字段

### 所有权状态追踪
- Owned：变量刚定义时的状态
- Borrowed：变量被借用（& 或 &mut）
- Moved：变量所有权被移动
- Dropped：变量离开作用域

### 作用域分析
- 自动识别作用域层级
- 区分函数、块、循环等作用域

## 示例文件

项目包含以下示例文件：

| 文件 | 描述 |
|------|------|
| examples/demo.rs | 演示基本功能 |
| examples/test_borrow.rs | 测试借用场景 |
| examples/test_move.rs | 测试移动语义 |
| examples/test_scope.rs | 测试作用域分组 |
| examples/test_ownership.rs | 测试所有权变化 |
| examples/test_unused.rs | 测试未使用变量 |
| examples/test_mutable.rs | 测试可变变量 |

## 常见问题

### Q: 为什么所有节点都是灰色的？

A: 这通常是因为变量在作用域退出时被标记为 Dropped。工具会自动忽略 Dropped 状态，取最后一个非 Dropped 状态的颜色。如果问题仍然存在，请检查代码逻辑。

### Q: DOT 和 SVG 显示不一致？

A: 请确保使用最新版本。如果问题仍然存在，请报告 Issue。

### Q: 如何在没有 Graphviz 的情况下使用？

A: SVG 渲染器是内置的，无需安装 Graphviz。DOT 文件可以在 [Graphviz Online](https://dreampuf.github.io/GraphvizOnline/) 中查看。

## 技术支持

如果遇到问题或有建议，请提交 Issue 到项目仓库。

## 更新日志

### v2.0
- 新增 DOT 格式导出
- 新增 SVG 渲染支持
- 添加作用域分组显示
- 修复颜色显示问题

### v1.0
- 基础变量分析功能
- 所有权追踪
- 命令行界面