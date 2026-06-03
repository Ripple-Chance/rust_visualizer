# RustVisualizer 测试用例索引

## 📁 测试文件目录

所有测试文件位于 `examples/` 目录下。

---

## 📝 测试用例列表

| 编号 | 文件名 | 功能测试 | 描述 |
|------|--------|---------|------|
| T001 | `test_simple.rs` | 简单变量分析 | 最基本的变量定义和使用测试 |
| T002 | `test_empty.rs` | 空文件处理 | 测试空函数体处理 |
| T003 | `test_nested_scope.rs` | 嵌套作用域 | 3层嵌套作用域的变量隔离测试 |
| T004 | `test_unused.rs` | 未使用变量检测 | 测试未使用变量识别功能 |
| T005 | `test_function.rs` | 函数参数追踪 | 测试函数参数作为变量 |
| T006 | `test_mutable.rs` | 可变性识别 | 测试 `let mut` 识别 |
| T007 | `test_simple.rs` | 宏调用处理 | 测试 `println!` 中变量识别 |
| T008 | `test_function.rs` | 复杂表达式 | 测试 `vec!`, `iter()` 等 |
| T009 | `test_function.rs` | 多函数定义 | 测试主函数+辅助函数 |
| T010 | `test_struct_pattern.rs` | 结构化模式 | 测试 `vec!` 模式匹配 |
| - | `demo.rs` | 综合演示 | 演示文件，包含主要功能展示 |

---

## 🎯 使用方法

### 运行所有测试

```bash
# 运行单个测试
cargo run -- analyze examples/test_simple.rs --list-vars

# 运行多个测试（PowerShell）
cargo run -- analyze examples/test_simple.rs --list-vars
cargo run -- analyze examples/test_nested_scope.rs --list-vars
cargo run -- analyze examples/test_unused.rs --unused
```

### 添加新测试

1. 在 `examples/` 目录下创建新的测试文件（如 `test_new_feature.rs`）
2. 编写测试代码
3. 运行测试并验证功能
4. 更新本文档，添加新测试条目

---

## 📋 测试清单

### MVP 阶段测试清单

- [x] T001 - 简单变量分析
- [x] T002 - 空文件处理
- [x] T003 - 嵌套作用域
- [x] T004 - 未使用变量检测
- [x] T005 - 函数参数追踪
- [x] T006 - 可变性识别
- [x] T007 - 宏调用处理
- [x] T008 - 复杂表达式
- [x] T009 - 多函数定义
- [x] T010 - 结构化模式

### v1.0 阶段测试清单（待实现）

- [ ] T011 - 所有权转移（赋值）
- [ ] T012 - 所有权转移（函数传参）
- [ ] T013 - 不可变借用（`&T`）
- [ ] T014 - 可变借用（`&mut T`）
- [ ] T015 - 借用冲突检测
- [ ] T016 - Copy 类型识别
- [ ] T017 - 生命周期推导

### v2.0 阶段测试清单（待实现）

- [ ] T018 - DOT 格式导出
- [ ] T019 - SVG 渲染
- [ ] T020 - 可视化样式

### v3.0 阶段测试清单（待实现）

- [ ] T021 - 长借用链检测
- [ ] T022 - 嵌套借用检测
- [ ] T023 - Arc/Mutex 模式扫描

---

## 🔍 测试数据说明

### 测试文件命名规范

```
test_<功能模块>_<具体场景>.rs
```

示例：
- `test_simple.rs` - 简单功能测试
- `test_nested_scope.rs` - 嵌套作用域测试
- `test_unused.rs` - 未使用变量测试

### 测试文件内容规范

1. **文件头部注释**：简要说明测试目的
2. **代码简洁**：只包含测试相关的代码
3. **命名规范**：使用有意义的变量名
4. **可独立运行**：每个测试文件都应该是完整的 Rust 程序

---

## 📊 测试覆盖率趋势

```
版本      已测试用例数    覆盖率
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
MVP v0.1    10           100%
v1.0         7           (待实现)
v2.0         3           (待实现)
v3.0         3           (待实现)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
总计        23           (最终目标)
```

---

**最后更新**：2026-05-31  
**负责人**：RustVisualizer Team  
**版本**：v0.1.0
