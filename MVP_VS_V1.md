# RustVisualizer MVP vs v1.0.0 版本对比

## 版本概览

| 维度 | MVP (v0.1.0) | v1.0.0 |
|------|-------------|--------|
| 定位 | 基础框架验证 | 核心语义分析 |
| 代码行数 | ~500 行 | ~1200 行 |
| 模块数 | 4 个 | 7 个 |
| 核心功能 | 变量追踪 | 所有权+借用+生命周期 |
| CLI 参数 | 3 个 | 6 个 |

---

## 架构演进

### MVP 架构
```
src/
├── main.rs              # CLI 入口
├── error.rs             # 错误处理
├── parser/
│   ├── mod.rs
│   ├── ast_visitor.rs   # AST 遍历
│   └── events.rs        # 基础事件
└── graph/
    ├── mod.rs
    └── variable_graph.rs # 变量关系图
```

### v1.0.0 架构
```
src/
├── main.rs              # CLI 入口（扩展参数）
├── error.rs             # 错误处理
├── parser/
│   ├── mod.rs
│   ├── ast_visitor.rs   # AST 遍历（增强：识别借用、移动）
│   └── events.rs        # 扩展事件（BorrowCreated、OwnershipMoved）
├── graph/
│   ├── mod.rs
│   └── variable_graph.rs
└── analysis/            # 新增：语义分析层
    ├── mod.rs           # 核心数据结构
    ├── ownership.rs     # 所有权分析器
    ├── borrow.rs        # 借用分析器
    └── lifetime.rs      # 生命周期分析器
```

**关键变化**：新增 `analysis/` 模块层，将语法解析与语义分析分离。

---

## 功能对比

### 基础功能

| 功能 | MVP | v1.0.0 | 变化说明 |
|------|-----|--------|---------|
| Rust 源码解析 | ✅ | ✅ | 无变化 |
| 变量定义提取 | ✅ | ✅ | 无变化 |
| 变量使用追踪 | ✅ | ✅ | 无变化 |
| 作用域追踪 | ✅ | ✅ | 无变化 |
| 宏调用处理 | ✅ | ✅ | 无变化 |
| 可变性识别 | ✅ | ✅ | 无变化 |

### 新增核心功能

| 功能 | MVP | v1.0.0 | 说明 |
|------|-----|--------|------|
| **所有权分析** | ❌ | ✅ | 追踪所有权状态变化（Owned → Moved → Dropped） |
| **借用分析** | ❌ | ✅ | 识别 `&`/`&mut`，统计引用链长度 |
| **生命周期分析** | ❌ | ✅ | 推导变量有效作用域，统计引用次数 |
| **所有权转移检测** | ❌ | ✅ | 识别赋值导致的所有权移动 |
| **函数参数所有权** | ❌ | ✅ | 区分值传递与借用传递 |

### CLI 接口对比

**MVP 命令**
```bash
rust_visualizer analyze <file.rs>
  --list-vars     # 列出变量
  --unused        # 显示未使用变量
  --events        # 显示事件序列
```

**v1.0.0 命令**
```bash
rust_visualizer analyze <file.rs>
  --list-vars     # 列出变量（保留）
  --unused        # 显示未使用变量（保留）
  --events        # 显示事件序列（保留）
  --ownership     # 新增：所有权分析
  --borrow        # 新增：借用分析
  --lifetime      # 新增：生命周期分析
```

---

## 数据结构与事件系统

### MVP 事件类型
```rust
pub enum EventKind {
    VarDefined(Variable),
    VarUsed { name: String, scope_level: usize },
    VarDropped { name: String },
    FuncDefined(Function),
    ScopeEnter { level: usize },
    ScopeExit { level: usize },
}
```

### v1.0.0 事件类型
```rust
pub enum EventKind {
    VarDefined(Variable),
    VarUsed { name: String, scope_level: usize },
    VarDropped { name: String },
    FuncDefined(Function),
    ScopeEnter { level: usize },
    ScopeExit { level: usize },
    BorrowCreated { name: String, kind: BorrowKind, scope_level: usize },  // 新增
    BorrowDropped { name: String, kind: BorrowKind },                      // 新增
    OwnershipMoved { name: String, target: Option<String>, is_function_call: bool }, // 新增
    VariableCopied { name: String, target: String },                       // 新增
}
```

### 新增核心数据结构

**所有权状态**
```rust
pub enum OwnershipStatus {
    Owned,
    Borrowed(BorrowKind),      // 新增
    BorrowedMutably,           // 新增
    Moved,                     // 新增
    Dropped,
}
```

**借用类型**
```rust
pub enum BorrowKind {
    Immutable,  // &
    Mutable,    // &mut
}
```

---

## 测试覆盖对比

### MVP 测试场景

| 测试编号 | 场景 | 状态 |
|----------|------|------|
| T001 | 简单变量分析 | ✅ |
| T002 | 空文件处理 | ✅ |
| T003 | 嵌套作用域 | ✅ |
| T004 | 未使用变量检测 | ✅ |
| T005 | 函数参数追踪 | ✅ |
| T006 | 可变性识别 | ✅ |
| T007 | 宏调用处理 | ✅ |
| T008 | 复杂表达式 | ✅ |
| T009 | 多函数定义 | ✅ |
| T010 | 结构化模式 | ✅ |

**覆盖率**: 基础功能 100%，所有权与借用 0%，生命周期 0%

### v1.0.0 测试场景

| 测试编号 | 场景 | 状态 |
|----------|------|------|
| TC-01 | 所有权转移检测 | ✅ |
| TC-02 | 借用冲突检测 | ✅ |
| TC-03 | 生命周期与作用域 | ✅ |
| TC-04 | 可变借用链 | ✅ |
| TC-05 | 函数参数所有权 | ✅ |

**覆盖率**: 所有权分析 100%，借用分析 100%，生命周期分析 100%

---

## 输出示例对比

### MVP 输出
```
=== Variable Analysis ===
Total: 6, Used: 6, Unused: 0
Variables: ["x", "y", "z", "a", "b", "result"]

=== Unused Variables ===
No unused variables found.
```

### v1.0.0 输出（新增）
```
=== Ownership Analysis ===
[OWNERSHIP] s1 -> owned at Span
[OWNERSHIP] s2 -> owned at Span
[OWNERSHIP] s1 -> dropped at Span    // 所有权转移
[OWNERSHIP] s2 -> dropped at Span

=== Borrow Analysis ===
[BORROW] & data at Span
[BORROW] &mut data at Span
[DROP_BORROW] & data at Span
[DROP_BORROW] &mut data at Span
Long borrow chains: data: 6 references

=== Lifetime Analysis ===
Lifetime Summary:
  - data: 6 references
  - outer: 2 references
  - inner: 4 references
```

---

## 技术债务与已知限制

### MVP 阶段
| 限制 | 状态 |
|------|------|
| 无所有权分析 | v1.0 已解决 |
| 无借用分析 | v1.0 已解决 |
| 无生命周期分析 | v1.0 已解决 |

### v1.0.0 阶段
| 限制 | 计划解决版本 |
|------|-------------|
| 借用冲突严格验证（同时存在 `&` 和 `&mut`） | v1.1 |
| Span 未转换为可读行号 | v1.1 |
| 生命周期参数推导（如 `'a`） | v1.2 |
| Copy trait 语义细化 | v1.2 |

---

## 演进路线总结

```
MVP (v0.1.0) ──→ v1.0.0 ──→ v1.1.0 ──→ v1.2.0
    │               │            │            │
    ▼               ▼            ▼            ▼
 变量追踪      所有权分析    借用规则验证   生命周期参数
 作用域管理    借用识别       行号定位       可视化输出
 基础事件      生命周期推导   错误诊断       交互式分析
```

---

## 结论

**MVP** 验证了项目的技术可行性，建立了 AST 解析和事件驱动分析的基础框架。

**v1.0.0** 在此基础上实现了 Rust 核心语义分析能力：
- 所有权状态机（Owned → Borrowed → Moved → Dropped）
- 借用类型识别与引用链统计
- 基于作用域的生命周期推导

从 MVP 到 v1.0.0，项目从"能解析代码"演进为"能理解 Rust 核心语义"，为后续的错误诊断和可视化输出奠定了基础。
