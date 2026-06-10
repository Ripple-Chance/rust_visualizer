# Rust Visualizer

**版本**: 3.0  
**更新日期**: 2026-06-10  

一个用于可视化 Rust 代码变量所有权关系的工具，支持 DOT 格式导出、SVG 渲染、交互式可视化和 Web 服务。

## 功能特性

- ✅ Rust 源代码解析
- ✅ 变量声明检测
- ✅ 作用域分析
- ✅ 完整所有权分析
- ✅ 借用分析（不可变/可变）
- ✅ 所有权移动分析
- ✅ 生命周期分析
- ✅ 未使用变量检测
- ✅ DOT 格式导出
- ✅ SVG 渲染
- ✅ **交互式 SVG**（v3 新增）
- ✅ **时间线动画**（v3 新增）
- ✅ **Web 服务 API**（v3 新增）
- ✅ **批量分析**（v3 新增）

## 安装

```bash
git clone https://github.com/Ripple-Chance/rust_visualizer.git
cd rust_visualizer
cargo build --release
```

## 使用方法

### 基础分析

```bash
# 分析单个文件并生成 SVG
cargo run -- analyze examples/test_scope.rs --svg output.svg

# 生成 DOT 文件
cargo run -- analyze examples/test_scope.rs --dot output.dot

# 生成交互式 SVG
cargo run -- analyze examples/test_scope.rs --interactive interactive.svg

# 生成时间线动画
cargo run -- analyze examples/test_scope.rs --animation animation.svg

# 生成 JSON 输出
cargo run -- analyze examples/test_scope.rs --json output.json
```

### Web 服务模式

```bash
# 启动 Web 服务（默认端口 8080）
cargo run -- server

# 指定端口
cargo run -- server --port 3000
```

### 批量分析

```bash
# 批量分析目录中的所有 Rust 文件
cargo run -- batch --input-dir src --output-dir output
```

## 输出格式

### 颜色编码

| 颜色 | 状态 | 说明 |
|------|------|------|
| 🟢 绿色 | Owned | 变量拥有所有权 |
| 🟠 橙色 | Moved | 所有权已移动 |
| 🔵 蓝色 | Borrowed(Immutable) | 不可变借用 |
| 🔴 红色 | Borrowed(Mutable) | 可变借用 |
| ⚪ 灰色 | Unused | 未使用变量 |

### JSON 输出示例

```json
{
  "file": "input.rs",
  "total_variables": 5,
  "used_variables": 4,
  "unused_variables": 1,
  "variables": [
    {
      "name": "x",
      "type": "i32",
      "used": true,
      "scope": 1
    }
  ]
}
```

## 项目结构

```
rust_visualizer/
├── src/
│   ├── analysis/
│   │   ├── mod.rs
│   │   └── ownership.rs
│   ├── graph/
│   │   ├── mod.rs
│   │   ├── variable_graph.rs
│   │   ├── dot_export.rs
│   │   ├── svg_renderer.rs
│   │   ├── interactive_svg.rs    # v3 新增
│   │   └── timeline_animator.rs  # v3 新增
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── ast_visitor.rs
│   │   └── events.rs
│   ├── web/                       # v3 新增
│   │   └── service.rs
│   ├── lib.rs
│   └── main.rs
├── examples/
├── tests/
├── README.md
├── USAGE.md
├── V3_VERSION_NOTES.md
└── Cargo.toml
```

## API 文档

### POST /api/analyze

分析 Rust 代码并返回分析结果。

**请求体**:
```json
{
  "code": "fn main() { let x = 42; }"
}
```

**响应**:
```json
{
  "success": true,
  "message": "Analysis completed successfully",
  "data": {
    "total_variables": 1,
    "used_variables": 0,
    "unused_variables": 1,
    "dot_output": "...",
    "svg_output": "..."
  }
}
```

## 测试报告

### 单元测试

```
running 7 tests
test graph::dot_export::tests::test_dot_export_basic ... ok
test graph::dot_export::tests::test_dot_style_defaults ... ok
test graph::dot_export::tests::test_sanitize_id ... ok
test graph::svg_renderer::tests::test_render_to_dot ... ok
test graph::svg_renderer::tests::test_render_simple ... ok
test graph::dot_export::tests::test_dot_config_defaults ... ok
test graph::svg_renderer::tests::test_svg_config_defaults ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured
```

### 测试文件

| 测试文件 | 描述 |
|----------|------|
| `examples/test_scope.rs` | 测试嵌套作用域 |
| `examples/test_borrow_rules.rs` | 测试借用规则 |
| `examples/test_function_ownership.rs` | 测试函数间所有权转移 |
| `examples/test_struct_ownership.rs` | 测试结构体所有权 |
| `examples/test_nested_scope.rs` | 测试嵌套作用域与所有权转移 |

### 编译状态

- ✅ `cargo build --release` - 无警告
- ✅ `cargo test --release` - 全部通过

## 许可证

MIT License