# Rust Visualizer

**版本**: 2.0  
**更新日期**: 2026-06-09  

一个用于可视化 Rust 代码变量所有权关系的工具，支持 DOT 格式导出和 SVG 渲染。

## 功能特性

- 🔍 **变量分析**：解析 Rust 源代码，识别变量定义、使用、借用和移动
- 📊 **所有权追踪**：追踪变量的所有权状态变化（Owned、Borrowed、Moved、Dropped）
- 📈 **DOT 导出**：生成标准 Graphviz DOT 格式文件
- 🎨 **SVG 渲染**：内置 SVG 渲染器，无需外部依赖
- 📦 **作用域分组**：按作用域层级分组显示变量
- 🎯 **样式定制**：支持自定义颜色和样式

## 安装与运行

### 前置要求

- Rust 1.70+

### 构建

```bash
git clone https://github.com/your-repo/rust_visualizer.git
cd rust_visualizer
cargo build --release
```

### 运行

```bash
# 分析 Rust 文件并生成可视化输出
cargo run --release -- analyze <input.rs> [--dot output.dot] [--svg output.svg]
```

## 使用示例

```bash
# 分析文件并生成 DOT 和 SVG
cargo run --release -- analyze examples/test_scope.rs --dot output.dot --svg output.svg

# 仅生成 DOT 文件
cargo run --release -- analyze examples/test_borrow.rs --dot output.dot

# 仅生成 SVG 文件
cargo run --release -- analyze examples/test_move.rs --svg output.svg
```

## 输出格式

### DOT 文件

生成的 DOT 文件可以在 [Graphviz Online](https://dreampuf.github.io/GraphvizOnline/) 中查看和渲染。

### SVG 文件

生成的 SVG 文件可以直接在浏览器中打开查看。

## 颜色编码

| 状态 | 颜色 | 说明 |
|------|------|------|
| Owned | 🟢 `#4CAF50` | 拥有所有权 |
| Moved | 🟠 `#FF9800` | 已移动 |
| Borrowed(Immutable) | 🔵 `#2196F3` | 不可变借用 |
| Borrowed(Mutable) | 🔴 `#F44336` | 可变借用 |
| Dropped | ⚪ `#9E9E9E` | 已销毁 |
| Unused | ⚪ `#EEEEEE` | 未使用 |

## 项目结构

```
rust_visualizer/
├── src/
│   ├── analysis/
│   │   └── ownership.rs      # 所有权分析
│   ├── graph/
│   │   ├── dot_export.rs     # DOT 格式导出
│   │   ├── svg_renderer.rs   # SVG 渲染器
│   │   └── variable_graph.rs # 变量关系图
│   ├── parser/               # Rust 语法解析
│   └── main.rs               # 命令行入口
├── examples/                 # 示例代码
├── tests/                    # 测试文件
├── README.md                 # 项目说明
├── USAGE.md                  # 使用说明
└── V2_VERSION_NOTES.md       # v2.0 版本说明
```

## 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test --test v2_visualization
```

## 版本历史

### v2.0 (当前)
- 新增 DOT 格式导出功能
- 新增 SVG 渲染支持
- 实现作用域分组显示
- 添加样式定制功能

### v1.0
- 基础变量分析功能
- 所有权追踪
- 命令行界面

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！