# Rust Visualizer v3.0 版本说明

**版本**: 3.0  
**发布日期**: 2026-06-10  

## 概述

Rust Visualizer v3.0 是一个用于可视化 Rust 代码变量所有权关系的工具，在 v2.0 的基础上新增了交互式可视化、时间线动画和 Web 服务功能。

## 新增功能

### 1. 交互式可视化
- 支持点击节点高亮显示
- 悬停显示变量详细信息（名称、类型、使用状态、作用域）
- CSS 过渡动画效果
- 图例说明

### 2. 时间线动画
- 展示变量所有权状态随时间变化
- 动画播放控制（前进/后退）
- 时间轴进度条
- 事件描述显示
- 美观的渐变背景设计
- 颜色图例说明

### 3. HTML 动画输出
- 完整的网页界面
- 现代化 UI 设计
- 渐变紫色背景
- 卡片式布局
- 响应式设计

### 4. 命令行界面增强
- 彩色输出（使用 colored crate）
- 进度条显示（使用 indicatif crate）
- 支持 JSON 格式输出
- 批量分析模式

### 5. Web 服务模式
- REST API 接口（POST /api/analyze）
- 内置 Web UI 界面
- JSON 格式响应
- 支持远程分析

### 6. 批量分析
- 项目级分析
- 自动遍历目录中的 Rust 文件
- 批量生成 SVG 输出

### 7. 界面优化
- SVG 图片尺寸优化（800x500）
- 作用域框定位修正
- 变量节点居中显示
- 未使用变量灰色显示
- 图例说明增强

## 核心模块

### graph/interactive_svg.rs
- `InteractiveSvgRenderer`: 交互式 SVG 渲染器
- 支持鼠标悬停提示、点击高亮

### graph/timeline_animator.rs
- `TimelineAnimator`: 时间线动画引擎
- 支持逐帧生成和动画 SVG 输出

### web/service.rs
- `start_server()`: 启动 Web 服务
- REST API 处理

## 使用方法

### 基础分析
```bash
cargo run -- analyze input.rs --svg output.svg
```

### 交互式 SVG
```bash
cargo run -- analyze input.rs --interactive interactive.svg
```

### 时间线动画（HTML）
```bash
cargo run -- analyze input.rs --html animation.html
```

### 时间线动画（SVG）
```bash
cargo run -- analyze input.rs --animation animation.svg
```

### JSON 输出
```bash
cargo run -- analyze input.rs --json output.json
```

### 启动 Web 服务
```bash
cargo run -- server --port 8080
```

### 批量分析
```bash
cargo run -- batch --input-dir src --output-dir output
```

### 生成多种格式
```bash
cargo run -- analyze input.rs --svg output.svg --html animation.html --json output.json
```

## 技术栈

- Rust 2021
- clap 4.0（CLI）
- hyper 1.0（Web API）
- tokio 1.0（异步运行时）
- serde 1.0（JSON 序列化）
- indicatif（进度条）
- colored（彩色输出）
- walkdir（目录遍历）

## 版本对比

| 功能 | v1.0 | v2.0 | v3.0 |
|------|------|------|------|
| 所有权分析 | ✅ | ✅ | ✅ |
| DOT 导出 | ❌ | ✅ | ✅ |
| SVG 渲染 | ❌ | ✅ | ✅ |
| 交互式 SVG | ❌ | ❌ | ✅ |
| 时间线动画 | ❌ | ❌ | ✅ |
| Web 服务 | ❌ | ❌ | ✅ |
| 批量分析 | ❌ | ❌ | ✅ |
| JSON 输出 | ❌ | ❌ | ✅ |

## 许可证

MIT License