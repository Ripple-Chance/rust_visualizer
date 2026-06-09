# Rust Visualizer v2.0 版本说明

**版本**: 2.0  
**发布日期**: 2026-06-09  

## 概述

Rust Visualizer v2.0 是一个用于可视化 Rust 代码变量所有权关系的工具，支持 DOT 格式导出和 SVG 渲染。

## 新增功能

### 1. 变量关系图构建
- 基于 AST 解析构建变量关系图
- 支持变量定义、使用、借用、移动等事件追踪
- 按作用域层级分组显示变量

### 2. DOT 格式导出
- 生成标准 Graphviz DOT 格式文件
- 支持自定义节点样式和颜色
- 作用域分组显示（使用 subgraph cluster）
- 所有权状态可视化（Owned、Borrowed、Moved、Dropped）

### 3. SVG 渲染支持
- 内置 SVG 渲染器，无需外部依赖
- 支持通过 Graphviz 工具生成高质量 SVG
- 响应式布局和自定义样式

### 4. 可视化样式定制
- 所有权状态颜色编码：
  - Owned（拥有）：绿色 `#4CAF50`
  - Moved（已移动）：橙色 `#FF9800`
  - Borrowed(Immutable)（不可变借用）：蓝色 `#2196F3`
  - Borrowed(Mutable)（可变借用）：红色 `#F44336`
  - Dropped（已销毁）：灰色 `#9E9E9E`
  - Unused（未使用）：浅灰色 `#EEEEEE`

### 5. 作用域分层展示
- 自动识别变量的作用域层级
- 使用子图（subgraph）实现作用域分组
- 每个作用域显示为带标题的框

## 核心模块

### graph/dot_export.rs
- `DotConfig`: DOT 导出配置
- `DotStyle`: 样式配置（颜色、字体等）
- `DotExporter`: DOT 格式生成器

### graph/svg_renderer.rs
- `SvgConfig`: SVG 渲染配置
- `SvgRenderer`: SVG 渲染器
- 支持简单渲染和 Graphviz 渲染两种模式

### analysis/ownership.rs
- `OwnershipAnalyzer`: 所有权分析器
- 追踪变量所有权状态变化
- 生成分析结果供可视化使用

### graph/variable_graph.rs
- `VarNode`: 变量节点
- `VarGraph`: 变量关系图
- `GraphBuilder`: 图构建器

## 修复的问题

1. **所有节点灰色问题**：修复了 Dropped 状态过滤逻辑，取最后一个非 Dropped 状态
2. **DOT 和 SVG 颜色不一致**：统一了可变借用和不可变借用的颜色定义
3. **作用域分组缺失**：SVG 渲染器添加了作用域分组支持
4. **未使用变量显示**：添加了未使用变量的灰色显示
5. **作用域框定位错误**：修复了节点和作用域框的位置计算

## 测试结果

| 测试模块 | 测试数量 | 通过 | 失败 |
|----------|----------|------|------|
| 单元测试 | 7 | 7 | 0 |
| 集成测试 | 9 | 9 | 0 |
| 总计 | 16 | 16 | 0 |

## 版本对比

| 功能 | v1.0 | v2.0 |
|------|------|------|
| 变量分析 | ✅ | ✅ |
| 所有权追踪 | ✅ | ✅ |
| DOT 导出 | ❌ | ✅ |
| SVG 渲染 | ❌ | ✅ |
| 作用域分组 | ❌ | ✅ |
| 样式定制 | ❌ | ✅ |
| 可视化输出 | ❌ | ✅ |

## 使用示例

```bash
# 分析 Rust 文件并生成 DOT 和 SVG
cargo run --release -- analyze examples/test_scope.rs --dot output.dot --svg output.svg

# 仅生成 DOT 文件
cargo run --release -- analyze examples/test_borrow.rs --dot output.dot

# 仅生成 SVG 文件
cargo run --release -- analyze examples/test_move.rs --svg output.svg
```

## 输出文件

- **.dot**：Graphviz DOT 格式文件，可用于 Graphviz Online 可视化
- **.svg**：SVG 图像文件，可直接在浏览器中查看

## 技术栈

- Rust 1.70+
- petgraph（图数据结构）
- syn（Rust 语法解析）
- quote（代码生成）

## 项目结构

```
rust_visualizer/
├── src/
│   ├── analysis/
│   │   └── ownership.rs      # 所有权分析
│   ├── graph/
│   │   ├── dot_export.rs     # DOT 导出
│   │   ├── svg_renderer.rs   # SVG 渲染
│   │   └── variable_graph.rs # 变量关系图
│   ├── parser/               # 语法解析
│   └── main.rs               # 主入口
├── examples/                 # 示例代码
├── tests/                    # 测试文件
└── README.md                 # 项目说明
```