# RustVisualizer 使用说明

本文档提供 RustVisualizer 工具的详细使用指南。

## 目录

- [安装](#安装)
- [基本用法](#基本用法)
- [命令行参数](#命令行参数)
- [输出格式](#输出格式)
- [API 使用](#api-使用)
- [示例代码](#示例代码)
- [常见问题](#常见问题)

---

## 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/Ripple-Chance/rust_visualizer.git
cd rust_visualizer

# 编译 Release 版本（优化过的二进制文件）
cargo build --release

# 编译 Debug 版本（包含调试信息）
cargo build
```

### 验证安装

```bash
# 查看帮助信息
cargo run --release -- --help
```

---

## 基本用法

### 分析单个文件

```bash
# 基本分析
cargo run --release -- examples/demo.rs

# 指定输入文件
cargo run --release -- /path/to/your/rust_file.rs
```

### 生成可视化输出

```bash
# 生成 DOT 格式（Graphviz）
cargo run --release -- examples/demo.rs --dot output.dot

# 生成 SVG 格式
cargo run --release -- examples/demo.rs --svg output.svg
```

---

## 命令行参数

| 参数 | 说明 | 示例 |
|------|------|------|
| `<INPUT>` | 输入的 Rust 源文件路径 | `examples/demo.rs` |
| `--dot <FILE>` | 导出 DOT 格式到指定文件 | `--dot graph.dot` |
| `--svg <FILE>` | 导出 SVG 格式到指定文件 | `--svg graph.svg` |
| `--show-unused` | 显示未使用的变量 | `--show-unused` |
| `--horizontal` | 使用水平布局（默认垂直） | `--horizontal` |
| `--no-scope` | 不显示作用域分组 | `--no-scope` |
| `-h, --help` | 显示帮助信息 | `--help` |
| `-V, --version` | 显示版本信息 | `--version` |

### 示例

```bash
# 完整示例：分析文件并生成水平布局的 SVG
cargo run --release -- examples/ownership_test.rs --svg output.svg --horizontal

# 不显示未使用变量，使用垂直布局
cargo run --release -- examples/demo.rs --dot analysis.dot --show-unused=false
```

---

## 输出格式

### 1. DOT 格式（Graphviz）

DOT 是 Graphviz 使用的图形描述语言，生成的 `.dot` 文件可以使用以下工具打开：

- **命令行工具**: `dot -Tpng graph.dot -o graph.png`
- **在线工具**: [Graphviz Online](https://dreampuf.github.io/GraphvizOnline/)
- **桌面应用**: [Graphviz for macOS/Windows](https://www.graphviz.org/download/)

**DOT 输出特性**:
- 节点颜色表示所有权状态
  - 🟢 绿色: Owned（被拥有）
  - 🟠 橙色: Moved（已转移）
  - 🔵 蓝色: Borrowed（已借用）
  - ⚪ 灰色: Dropped（已销毁）
- 子图（subgraph cluster）表示不同作用域
- 箭头表示变量关系

### 2. SVG 格式

SVG 是一种矢量图形格式，可以直接在浏览器中打开或嵌入到文档中。

**SVG 输出特性**:
- 自动生成图例
- 显示统计信息（节点数、边数）
- 可缩放，不失真
- 支持在浏览器中直接查看

### 3. 文本输出

默认情况下，工具会输出分析事件的文本描述：

```
[DEFINE] x: mut (scope: 0)
[DEFINE] y: immut (scope: 0)
[USE] x
[MOVE] x -> y
[OWNERSHIP] y -> Borrowed(Immutable) at ...
```

---

## API 使用

RustVisualizer 也可以作为库在代码中使用。

### 添加依赖

```toml
[dependencies]
rust_visualizer = { path = "../rust_visualizer" }
```

### 基本使用示例

```rust
use rust_visualizer::parser::ast_visitor::AstVisitor;
use rust_visualizer::analysis::{OwnershipAnalyzer, BorrowAnalyzer, LifetimeAnalyzer};
use rust_visualizer::graph::dot_export::{DotExporter, DotConfig};
use rust_visualizer::graph::svg_renderer::SvgRenderer;
use rust_visualizer::graph::variable_graph::GraphBuilder;
use std::path::Path;

// 1. 读取并解析源代码
let source_code = std::fs::read_to_string("examples/demo.rs").unwrap();
let syntax = syn::parse_file(&source_code).unwrap();

// 2. 提取分析事件
let visitor = AstVisitor::new();
let events = visitor.visit_file(&syntax);

// 3. 执行各种分析
let mut ownership_analyzer = OwnershipAnalyzer::new();
let ownership_results = ownership_analyzer.analyze(&events);

let mut borrow_analyzer = BorrowAnalyzer::new();
let borrow_results = borrow_analyzer.analyze(&events);

let mut lifetime_analyzer = LifetimeAnalyzer::new();
let lifetime_results = lifetime_analyzer.analyze(&events);

// 4. 构建变量关系图
let graph_builder = GraphBuilder::build_from_events(&events);
let graph = graph_builder.get_graph();

// 5. 生成可视化输出
let exporter = DotExporter::new();
let dot_export = exporter.export(graph, ownership_results);
println!("{}", dot_export.content);

// 6. 生成 SVG
let renderer = SvgRenderer::new();
let svg = renderer.render_simple(graph, ownership_results);
renderer.export_to_file(&svg, Path::new("output.svg")).unwrap();
```

### 配置选项

#### DotConfig 配置

```rust
use rust_visualizer::graph::dot_export::{DotExporter, DotConfig};

let config = DotConfig {
    title: "My Analysis".to_string(),
    show_ownership: true,      // 显示所有权状态
    show_borrows: true,       // 显示借用关系
    show_scopes: true,        // 显示作用域分组
    horizontal: false,        // false=垂直, true=水平
    show_unused: false,       // 不显示未使用变量
};

let exporter = DotExporter::with_config(config);
```

#### SvgConfig 配置

```rust
use rust_visualizer::graph::svg_renderer::{SvgRenderer, SvgConfig};

let config = SvgConfig {
    width: 1200,                    // 宽度（像素）
    height: 800,                    // 高度（像素）
    background: Some("#FFFFFF".to_string()),  // 背景色
    font_family: "Arial, sans-serif".to_string(),
    font_size: 12,
};

let renderer = SvgRenderer::with_config(config);
```

---

## 示例代码

### 示例 1: 基本所有权

`examples/ownership_basic.rs`:

```rust
fn main() {
    let x = String::from("hello");  // x 获得所有权
    let y = x;                      // 所有权转移到 y
    
    println!("{}", y);
}
```

**分析结果**:
- `x` 在第2行被定义（Owned）
- 第3行 `x` 的所有权转移到 `y`（Moved）
- `x` 在转移后不可使用

### 示例 2: 借用分析

`examples/borrow_analysis.rs`:

```rust
fn main() {
    let mut data = vec![1, 2, 3];
    
    // 不可变借用
    let print_data = || {
        println!("{:?}", data);
    };
    print_data();
    
    // 可变借用
    data.push(4);
    
    println!("{:?}", data);
}
```

**分析结果**:
- 不可变借用：`&data`
- 可变借用：`&mut data`（隐式）
- 借用在作用域结束时释放

### 示例 3: 生命周期

`examples/lifetime_demo.rs`:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long");
    let s2 = String::from("short");
    
    let result = longest(&s1, &s2);
    println!("{}", result);
}
```

**分析结果**:
- 识别生命周期参数 `'a`
- 追踪引用的有效性
- 检测潜在的悬垂引用

---

## 常见问题

### Q: 为什么我的 DOT 文件打开是乱码？

A: 确保使用 UTF-8 编码保存文件。Windows 用户可以用 VS Code 等编辑器转换编码。

### Q: SVG 输出为什么没有显示节点？

A: 简单 SVG 模式（不使用 Graphviz）仅生成包含图例和统计信息的基础框架。需要完整可视化请安装 [Graphviz](https://www.graphviz.org/download/) 并使用 `--dot` 参数导出。

### Q: 如何安装 Graphviz？

**Windows**:
```powershell
winget install graphviz
```

**macOS**:
```bash
brew install graphviz
```

**Linux (Ubuntu/Debian)**:
```bash
sudo apt-get install graphviz
```

安装后，使用以下命令生成 PNG：
```bash
dot -Tpng output.dot -o output.png
```

### Q: 支持哪些 Rust 语法？

当前版本支持：
- ✅ 变量定义和赋值
- ✅ 函数定义和调用
- ✅ 所有权转移（移动语义）
- ✅ 不可变借用和可变借用
- ✅ 作用域和生命周期
- ✅ 基本表达式和控制流

计划支持：
- 🔄 结构体和方法
- 🔄 泛型和 trait
- 🔄 闭包
- 🔄 并发原语（Arc, Mutex 等）

### Q: 工具会修改源代码吗？

不会。RustVisualizer 是纯静态分析工具，只读取源代码，不会进行任何修改。

---

## 相关资源

- [Rust 所有权文档](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Graphviz 文档](https://graphviz.org/documentation/)
- [SVG 教程](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial)

---

## 反馈与支持

如遇到问题或有功能建议，请提交 [GitHub Issue](https://github.com/Ripple-Chance/rust_visualizer/issues)。
