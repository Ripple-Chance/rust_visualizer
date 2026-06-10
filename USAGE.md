# Rust Visualizer 使用说明

**版本**: 3.0  
**更新日期**: 2026-06-10  

## 概述

Rust Visualizer 是一个用于分析和可视化 Rust 代码变量所有权关系的命令行工具。

## 命令行参数

### 基本命令

```bash
rust_visualizer <command> [options]
```

### 命令列表

| 命令 | 说明 |
|------|------|
| `analyze` | 分析单个 Rust 文件 |
| `server` | 启动 Web 服务 |
| `batch` | 批量分析目录中的 Rust 文件 |

### analyze 命令

```bash
rust_visualizer analyze <input_file> [options]
```

**选项**:

| 选项 | 说明 | 示例 |
|------|------|------|
| `--dot <path>` | 生成 DOT 格式文件 | `--dot output.dot` |
| `--svg <path>` | 生成 SVG 图像 | `--svg output.svg` |
| `--interactive <path>` | 生成交互式 SVG | `--interactive output.svg` |
| `--animation <path>` | 生成时间线动画 SVG | `--animation output.svg` |
| `--html <path>` | 生成 HTML 动画文件 | `--html output.html` |
| `--json <path>` | 生成 JSON 格式输出 | `--json output.json` |

**示例**:

```bash
# 生成基本 SVG
rust_visualizer analyze src/main.rs --svg output.svg

# 生成多种格式
rust_visualizer analyze src/main.rs --dot output.dot --svg output.svg --json output.json

# 生成交互式和动画 SVG
rust_visualizer analyze src/main.rs --interactive interactive.svg --animation animation.svg
```

### server 命令

```bash
rust_visualizer server [--port <port>]
```

**选项**:

| 选项 | 说明 | 默认值 |
|------|------|--------|
| `--port <port>` | 指定服务端口 | 8080 |

**示例**:

```bash
# 使用默认端口
rust_visualizer server

# 指定端口
rust_visualizer server --port 3000
```

启动后访问 http://localhost:8080 查看 Web 界面。

### batch 命令

```bash
rust_visualizer batch --input-dir <dir> --output-dir <dir>
```

**选项**:

| 选项 | 说明 |
|------|------|
| `--input-dir <dir>` | 输入目录（包含 Rust 文件） |
| `--output-dir <dir>` | 输出目录（存放 SVG 文件） |

**示例**:

```bash
rust_visualizer batch --input-dir src --output-dir output
```

## 输出格式说明

### SVG 输出

生成的 SVG 文件包含：
- 作用域分组（浅蓝色背景框）
- 变量节点（彩色矩形）
- 变量状态标签
- 图例说明

### 交互式 SVG

交互式 SVG 支持：
- 鼠标悬停显示变量详细信息
- 点击节点高亮显示
- CSS 动画效果

### HTML 动画

HTML 动画文件包含：
- 完整的网页界面
- 前进/后退控制按钮
- 进度条显示
- 事件描述面板
- 颜色图例说明
- 美观的渐变背景设计

**生成示例**:
```bash
rust_visualizer analyze input.rs --html animation.html
```

### 动画 SVG

动画 SVG 支持：
- 播放/暂停控制（已移除，简化为前进/后退）
- 前进/后退按钮
- 进度条显示
- 事件描述

### JSON 输出

JSON 输出包含：
- 文件路径
- 变量总数
- 使用/未使用变量数
- 每个变量的详细信息（名称、类型、使用状态、作用域）

## 颜色编码说明

| 颜色 | 状态 | 含义 |
|------|------|------|
| #4CAF50 | Owned | 变量拥有所有权 |
| #FF9800 | Moved | 所有权已移动 |
| #2196F3 | Borrowed(Immutable) | 不可变借用 |
| #F44336 | Borrowed(Mutable) | 可变借用 |
| #EEEEEE | Unused | 未使用变量 |

## API 使用说明

### POST /api/analyze

分析 Rust 代码。

**请求**:
```json
{
  "code": "fn main() { let x = 42; println!(\"{}\", x); }"
}
```

**响应**:
```json
{
  "success": true,
  "message": "Analysis completed successfully",
  "data": {
    "total_variables": 1,
    "used_variables": 1,
    "unused_variables": 0,
    "dot_output": "digraph {...}",
    "svg_output": "<svg>...</svg>"
  }
}
```

## 常见问题

### Q: 如何安装依赖？

```bash
cargo build --release
```

### Q: 如何使用 Graphviz 生成 PNG？

```bash
dot -Tpng input.dot -o output.png
```

### Q: Web 服务无法启动？

确保端口未被占用，或使用 `--port` 指定其他端口。

### Q: 批量分析没有生成文件？

确保输入目录包含 `.rs` 文件，输出目录有写入权限。

## 技术支持

如有问题，请提交 Issue 到 GitHub 仓库。