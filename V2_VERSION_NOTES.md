# RustVisualizer v2.0.0 版本说明

**版本号**: v2.0.0  
**发布日期**: 2026年6月8日  
**代号**: 绽放版本

## 版本概述

v2.0.0 是 RustVisualizer 的第三个迭代版本，主要实现了可视化输出功能，包括 DOT 格式导出和 SVG 渲染支持。这个版本使得分析结果可以以图形化的方式展示，极大地提升了工具的可读性和实用性。

## 新增功能

### 1. DOT 格式导出

实现了完整的 DOT 格式导出功能，可以将变量关系图导出为 Graphviz DOT 格式。

**核心组件**:
- `DotConfig`: DOT 导出配置结构
- `DotStyle`: 可视化样式配置
- `DotExporter`: DOT 导出器

**功能特性**:
- ✅ 支持自定义图表标题
- ✅ 可配置是否显示所有权状态
- ✅ 可配置是否显示借用关系
- ✅ 支持作用域分层展示
- ✅ 支持水平/垂直两种布局方向
- ✅ 可选择性显示未使用变量

### 2. SVG 渲染支持

提供了 SVG 渲染功能，支持生成可视化的图表输出。

**核心组件**:
- `SvgConfig`: SVG 渲染配置
- `SvgRenderer`: SVG 渲染器

**功能特性**:
- ✅ 简单 SVG 生成（无需外部依赖）
- ✅ 支持调用 Graphviz dot 命令生成高质量 SVG
- ✅ 可配置的输出尺寸和字体
- ✅ 自动生成图例和统计信息
- ✅ 支持导出到文件

### 3. 可视化样式定制

提供了丰富的可视化样式定制选项。

**样式配置**:
- 节点形状: box, circle, ellipse 等
- 所有权状态颜色:
  - Owned: #4CAF50 (绿色)
  - Moved: #FF9800 (橙色)
  - Borrowed: #2196F3 (蓝色)
  - Dropped: #9E9E9E (灰色)
- 借用类型颜色:
  - Immutable: #2196F3 (蓝色)
  - Mutable: #F44336 (红色)
- 未使用变量样式
- 作用域分组样式

### 4. 作用域分层展示

实现了作用域的分层可视化展示。

**功能特性**:
- 自动根据作用域级别分组
- 使用 Graphviz 子图（subgraph cluster）实现
- 每层作用域用不同颜色标注
- 支持嵌套作用域展示

## 模块结构

```
src/
├── graph/
│   ├── mod.rs          # 图模块入口
│   ├── variable_graph.rs   # 变量关系图
│   ├── dot_export.rs       # DOT 格式导出
│   └── svg_renderer.rs     # SVG 渲染器
```

## 新增依赖

无新增外部依赖，使用现有的 petgraph 库进行图操作。

## 测试覆盖

v2.0 版本包含了完整的集成测试，覆盖以下场景：

1. ✅ DOT 导出基本功能
2. ✅ DOT 配置自定义
3. ✅ 样式定制功能
4. ✅ SVG 渲染基本功能
5. ✅ SVG 配置默认值
6. ✅ 作用域分组
7. ✅ 未使用变量过滤
8. ✅ 所有权状态导出
9. ✅ 水平布局配置

**测试结果**: 9 个测试全部通过

## 与 v1.0.0 的对比

| 功能 | v1.0.0 | v2.0.0 |
|------|--------|--------|
| AST 解析 | ✅ | ✅ |
| 所有权分析 | ✅ | ✅ |
| 借用分析 | ✅ | ✅ |
| 生命周期分析 | ✅ | ✅ |
| DOT 导出 | ❌ | ✅ |
| SVG 渲染 | ❌ | ✅ |
| 可视化样式定制 | ❌ | ✅ |
| 作用域分层展示 | ❌ | ✅ |

## 使用示例

### DOT 导出示例

```rust
use rust_visualizer::graph::dot_export::{DotExporter, DotConfig};
use rust_visualizer::graph::variable_graph::VarGraph;
use rust_visualizer::analysis::AnalysisResult;

let config = DotConfig {
    title: "My Analysis".to_string(),
    show_ownership: true,
    show_borrows: true,
    show_scopes: true,
    horizontal: true,
    show_unused: false,
};

let exporter = DotExporter::with_config(config);
let dot_export = exporter.export(&graph, &analysis_results);

println!("{}", dot_export.content);
```

### SVG 渲染示例

```rust
use rust_visualizer::graph::svg_renderer::SvgRenderer;

let renderer = SvgRenderer::new();
let svg = renderer.render_simple(&graph, &analysis_results);

// 导出到文件
renderer.export_to_file(&svg, Path::new("output.svg"));
```

## 已知限制

1. 简单 SVG 渲染生成的是基础可视化，不包含完整的节点和边信息
2. 高质量 SVG 需要系统安装 Graphviz 工具
3. 目前不支持实时预览功能

## 未来计划

v3.0 版本将继续完善可视化功能，包括：
- 交互式可视化界面
- 实时预览功能
- 更多导出格式支持（PNG, PDF 等）
- 自定义布局算法

## 版本历史

- **v0.1.0 (MVP)**: 基础解析和变量追踪
- **v1.0.0**: 完整所有权与借用分析
- **v2.0.0**: 可视化输出与 DOT 导出

---

**项目地址**: https://github.com/Ripple-Chance/rust_visualizer  
**维护者**: Ripple-Chance  
**许可证**: MIT
