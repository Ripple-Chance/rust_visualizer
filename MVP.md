# RustVisualizer MVP 版本说明

## 📋 项目概述

**项目名称**：RustVisualizer  
**项目目标**：基于 AST 的 Rust 代码所有权与生命周期静态分析工具  
**当前版本**：MVP (Minimum Viable Product)  
**版本号**：v0.1.0  
**开发阶段**：基础框架搭建

---

## 🏗️ 架构设计

### 模块结构

```
src/
├── main.rs                    # CLI 入口，参数解析
├── error.rs                   # 统一错误处理
├── parser/                    # 语法解析层
│   ├── mod.rs                 # 模块导出
│   ├── ast_visitor.rs          # AST 访问者实现
│   └── events.rs              # 分析事件定义
└── graph/                     # 图结构模块
    ├── mod.rs                 # 模块导出
    └── variable_graph.rs      # 变量关系图构建
```

### 核心技术栈

| 组件 | 技术选型 | 用途 |
|------|---------|------|
| AST 解析 | `syn` | Rust 代码解析 |
| 图结构 | `petgraph` | 变量关系图存储 |
| 错误处理 | `thiserror` | 自定义错误类型 |
| CLI 解析 | `clap` | 命令行参数处理 |

### 数据流

```
Rust 源文件 → syn 解析 → AST Visitor 遍历 → 事件收集 → Graph Builder 构建 → 分析报告输出
```

---

## ✅ 已实现功能

### 核心功能

| 功能 | 状态 | 说明 |
|------|------|------|
| Rust 源码解析 | ✅ 已完成 | 使用 `syn` 库解析 `.rs` 文件 |
| 变量定义提取 | ✅ 已完成 | 识别 `let`/`let mut` 定义 |
| 变量使用追踪 | ✅ 已完成 | 识别变量引用和函数调用 |
| 作用域追踪 | ✅ 已完成 | 正确区分不同层级作用域 |
| 宏调用处理 | ✅ 已完成 | 处理 `println!` 等宏中的变量引用 |
| 变量分析报告 | ✅ 已完成 | 显示变量统计和未使用变量 |

### CLI 接口

```bash
# 分析 Rust 文件
rust_visualizer analyze <file.rs>

# 可选参数
--list-vars     # 列出所有变量
--unused        # 显示未使用变量
--events        # 显示分析事件序列
```

---

## 🧪 测试情况

### 功能验证

| 测试用例 | 输入 | 预期结果 | 实际结果 | 状态 |
|----------|------|----------|----------|------|
| 简单变量分析 | `examples/demo.rs` | 识别6个变量 | 识别6个变量 | ✅ 通过 |
| 变量使用追踪 | `examples/demo.rs` | 所有变量标记为已使用 | 6个变量全部已使用 | ✅ 通过 |
| 作用域层级 | `examples/demo.rs` | 正确区分作用域深度 | scope level 1/2/3 | ✅ 通过 |
| 宏内变量 | `examples/demo.rs` | 识别 `println!` 中的变量 | 正确识别 x, y, z | ✅ 通过 |
| 函数参数 | `examples/demo.rs` | 识别 `process(a, b)` 参数 | 识别 a, b 参数 | ✅ 通过 |

### 测试覆盖

#### MVP 阶段已测试场景

- [x] 变量定义提取
- [x] 变量使用追踪
- [x] 作用域层级管理
- [x] 宏调用处理
- [x] 空文件处理
- [x] 未使用变量检测
- [x] 函数参数追踪
- [x] 可变性识别（let mut）
- [x] 嵌套作用域隔离
- [x] 多函数定义
- [ ] 所有权转移场景（v1.0）
- [ ] 借用分析场景（v1.0）
- [ ] 生命周期计算（v1.0）
- [ ] 语法错误处理（v1.0）

#### 详细测试用例

| 测试编号 | 测试用例 | 测试文件 | 验证内容 | 状态 |
|----------|----------|---------|---------|------|
| T001 | 简单变量分析 | `test_simple.rs` | 基本 `let` 变量识别 | ✅ 通过 |
| T002 | 空文件处理 | `test_empty.rs` | 空 main 函数体处理 | ✅ 通过 |
| T003 | 嵌套作用域 | `test_nested_scope.rs` | 多层作用域变量隔离 | ✅ 通过 |
| T004 | 未使用变量检测 | `test_unused.rs` | 识别未使用变量 | ✅ 通过 |
| T005 | 函数参数追踪 | `test_function.rs` | 函数参数作为变量 | ✅ 通过 |
| T006 | 可变性识别 | `test_mutable.rs` | `let mut` 识别 | ✅ 通过 |
| T007 | 宏调用处理 | `test_simple.rs` | `println!` 中变量识别 | ✅ 通过 |
| T008 | 复杂表达式 | `test_function.rs` | `vec!`, `iter()` 等 | ✅ 通过 |
| T009 | 多函数定义 | `test_function.rs` | 主函数+辅助函数 | ✅ 通过 |
| T010 | 结构化模式 | `test_struct_pattern.rs` | `vec!` 模式匹配 | ✅ 通过 |

#### 测试结果详情

**T001 - 简单变量分析**
```
输入：test_simple.rs
期望：识别变量 x
实际：Total: 1, Used: 1, Unused: 0
结果：✅ 通过
```

**T002 - 空文件处理**
```
输入：test_empty.rs（main 函数体为空）
期望：正确处理，输出 0 个变量
实际：Total: 0, Used: 0, Unused: 0
结果：✅ 通过
```

**T003 - 嵌套作用域**
```
输入：test_nested_scope.rs（3层嵌套）
期望：正确识别 a, b, c 三个变量，变量互不冲突
实际：Total: 3, Used: 3, Unused: 0
结果：✅ 通过
```

**T004 - 未使用变量检测**
```
输入：test_unused.rs（包含 unused 变量）
期望：正确识别未使用变量
实际：Unused: ["unused (scope: 2)"]
结果：✅ 通过
```

**T005 - 函数参数追踪**
```
输入：test_function.rs（包含参数 a, b）
期望：正确识别函数参数为变量
实际：Total: 5 (data, output, a, b, result), 全部已使用
结果：✅ 通过
```

**T006 - 可变性识别**
```
输入：test_mutable.rs（包含 let score: mut）
期望：正确识别可变性
实际：输出中显示 "score: mut"
结果：✅ 通过
```

#### 测试覆盖率统计

```
类别                   已测试    待测试    覆盖率
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
基础功能                5         0       100%
边界情况                1         1        50%
所有权与借用            0         3         0%
生命周期                0         1         0%
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
总计                   6         5        55%
```

---

## 📊 MVP 阶段成果

### 代码统计

| 指标 | 数量 |
|------|------|
| 总代码行数 | ~500 行 |
| 模块数量 | 4 个 |
| 核心数据结构 | 5 个 |
| CLI 参数 | 3 个 |

### 关键数据结构

```rust
// 变量定义
struct Variable {
    name: String,           // 变量名
    span: Span,            // 定义位置
    is_mutable: bool,      // 是否可变
    scope_level: usize,    // 作用域层级
}

// 分析事件
enum AnalysisEvent {
    VarDefined(Variable),   // 变量定义
    VarUsed { name, span, scope_level },  // 变量使用
    FuncDefined(Function),  // 函数定义
    ScopeEnter { level },   // 进入作用域
    ScopeExit { level },    // 退出作用域
}
```

---

## 🔄 迭代计划

### 已完成 ✅
- [x] MVP v0.1.0 - 基础框架搭建

### 下一步计划 📋

| 版本 | 目标 | 功能 |
|------|------|------|
| v1.0 | 所有权与借用分析 | 追踪 Move 语义、& 和 &mut 借用 |
| v2.0 | 可视化输出 | DOT 格式导出、SVG 图形生成 |
| v3.0 | 启发式诊断 | 长借用链检测、Arc/Mutex 模式扫描 |

---

## 🚀 使用方法

### 编译项目

```bash
cargo build --release
```

### 运行分析

```bash
# 分析示例文件
cargo run -- analyze examples/demo.rs --list-vars

# 查看详细事件
cargo run -- analyze examples/demo.rs --events

# 显示未使用变量
cargo run -- analyze examples/demo.rs --unused
```

### 示例输出

```
=== Variable Analysis Summary ===
Total variables: 6
Used variables: 6
Unused variables: 0
```

---

## 📝 开发笔记

### 技术要点

1. **访问者模式**：使用 `syn::visit::Visit` 实现自定义 AST 遍历
2. **作用域管理**：通过 `scope_level` 计数器追踪嵌套深度
3. **事件驱动**：将 AST 遍历结果抽象为事件序列，便于后续分析

### 已知限制

1. 暂未实现所有权转移追踪
2. 暂未实现借用分析
3. 暂未实现生命周期计算
4. 暂未实现可视化输出

这些功能将在后续迭代中逐步实现。

---

## 📚 参考资料

- [syn crate 文档](https://docs.rs/syn/)
- [petgraph 文档](https://docs.rs/petgraph/)
- [thiserror 文档](https://docs.rs/thiserror/)
- [Rust AST 访问者模式教程](https://github.com/dtolnay/syn/tree/master/examples)

---

**版本日期**：2026-06-03  
**开发者**：Ripple-Chance
**许可证**：MIT
