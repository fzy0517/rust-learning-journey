# Rust 学习之旅

一个 Rust 学习仓库。根目录下的每个编号文件夹都是一个**独立的 Cargo 项目**，打开任意一个 `main.rs` 都能直接运行对应的代码。

## 环境要求

- **操作系统**: Windows / Linux / macOS
- **Rust**: 通过 [rustup](https://rustup.rs/) 安装
- **编辑器**: [Visual Studio Code](https://code.visualstudio.com/)

## 快速开始

### 1. 安装 Rust

访问 https://rustup.rs/ 下载安装器，或使用以下命令：

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# 下载 rustup-init.exe 并运行，选择默认安装
```

安装完成后重启终端，验证：

```bash
rustc --version
cargo --version
```

### 2. 安装 VS Code 插件

按 `Ctrl+Shift+X` 打开扩展面板，安装以下插件：

| 插件 | 用途 | 是否必须 |
|------|------|---------|
| **rust-analyzer** | Rust 语言支持（代码补全、跳转、错误提示、运行按钮） | ✅ 必须 |
| **CodeLLDB** | 调试器（用于 F5 一键调试） | ✅ 必须 |
| **Even Better TOML** | Cargo.toml 语法高亮 | ⭐ 推荐 |

> ⚠️ 注意：不要安装已弃用的 "Better TOML"，请使用 "Even Better TOML"。

### 3. 打开仓库

```bash
cd rust-learning-journey

# 关键：从终端启动 VS Code，确保继承 Rust 环境变量
code .
```

> 💡 **为什么必须从终端启动？** 直接点击桌面图标打开 VS Code 可能无法读取 `~/.cargo/env` 中的 PATH，导致找不到 `cargo` 或 `rustc`。

### 4. 运行代码

打开任意一个项目里的 `main.rs`，选择以下任意方式：

| 方式 | 操作 | 效果 |
|------|------|------|
| **▶ Run 按钮** | 点击 `fn main()` 上方的 `Run`（rust-analyzer 提供） | 直接运行当前文件所在的项目，最常用 |
| `Ctrl + Shift + B` | 默认构建任务 | 运行当前聚焦的 `main.rs` 所在项目 |
| `F5` | 调试 | 编译并启动调试器，支持断点、单步、查看变量 |

> 💡 快捷键方式是根据"当前聚焦的文件"定位它属于哪个项目的，所以使用 `Ctrl+Shift+B` / `F5` 前，确保光标停在某个项目的 `src/main.rs` 里。`Run` 按钮则没有这个限制。

## 新建学习项目

在仓库根目录执行：

```bash
cargo init --name topic_2 "2.变量"
```

两个要点：

1. **文件夹名随便起**——数字开头、带 `!` 等特殊字符都没关系；但 Cargo 包名必须是合法标识符，所以用 `--name` 单独指定。包名和文件夹名互不影响。
2. 命令执行完就是一个完整项目，`main.rs` 会自动出现 `Run` 按钮——**不需要复制 `.vscode`，也不需要重启 VS Code**。根目录的 `.vscode/settings.json` 里的 `*/Cargo.toml` 通配符会自动把它纳入 rust-analyzer 的管理。

也可以在终端手动运行某个项目：

```bash
cargo run --manifest-path "1.println!/Cargo.toml"
```

## .vscode 配置说明

根目录的 `.vscode` 文件夹让整个仓库的任意 `main.rs` 都能直接运行：

### settings.json — 项目发现

```json
{
    "rust-analyzer.linkedProjects": ["*/Cargo.toml"]
}
```

rust-analyzer 默认只识别工作区根目录的 `Cargo.toml`，这个配置让它自动加载所有一级子文件夹里的 Cargo 项目。没有它，`main.rs` 上方不会出现 `Run` 按钮。

### tasks.json — 编译任务

| 任务 | 快捷键 / 触发方式 | 效果 |
|------|------------------|------|
| `cargo run (当前 main.rs 所在项目)` | `Ctrl + Shift + B` | 编译并运行当前文件所在的项目 |
| `cargo check (当前 main.rs 所在项目)` | `Ctrl+Shift+P` → `Tasks: Run Task` | 只检查语法错误，不生成可执行文件（比 build 快） |

任务通过 `${fileDirname}/../Cargo.toml` 反推当前 `main.rs` 属于哪个项目；`$rustc` 错误捕获模式会把编译错误显示在左侧"问题"面板，点击直接跳转出错行。

### launch.json — 调试配置

| 配置 | 触发方式 | 效果 |
|------|---------|------|
| `Debug (当前 main.rs 所在项目)` | 按 `F5` | 编译当前文件所在项目 → 启动调试器 → 运行程序。遇到断点会暂停 |

调试时可用 `F10` 单步跳过、`F11` 单步进入，鼠标悬停变量可查看当前值，程序输出显示在 VS Code 内置终端中。

## 目录结构

```
rust-learning-journey/
├── .vscode/
│   ├── settings.json   # rust-analyzer 项目发现配置（管理所有子项目）
│   ├── tasks.json      # VS Code 编译任务配置
│   └── launch.json     # VS Code 调试配置
├── 1.println!/          # 一个独立的学习项目（ Cargo 项目 ）
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── 2.xxx/               # 以后新建的项目，同样结构
└── ...
```

## 常见问题

### Q: main.rs 上方没有 Run 按钮？

**解决**：确认根目录 `.vscode/settings.json` 存在且包含 `rust-analyzer.linkedProjects` 配置；然后执行 `Ctrl+Shift+P` → `rust-analyzer: Restart Server`。

### Q: 按 F5 或运行任务提示找不到 cargo？

**解决**：关闭 VS Code，从终端重新执行 `code .` 打开仓库。

### Q: CodeLLDB 安装时提示下载超时？

**解决**：手动下载对应平台的 `.vsix` 文件，通过 `Ctrl+Shift+P` → `Install from VSIX` 安装。

- Windows: `codelldb-win32-x64.vsix`
- Linux: `codelldb-linux-x64.vsix`
- macOS: `codelldb-darwin-x64.vsix` 或 `codelldb-darwin-arm64.vsix`

镜像下载（国内可用）：
```
https://ghproxy.com/https://github.com/vadimcn/codelldb/releases/download/v1.12.3/codelldb-<平台>.vsix
```

### Q: 项目很多时 rust-analyzer 变慢？

每个子项目都会被索引，但学习项目都很小，一般无感。如果只想专注某一个项目，可以单独打开那个子文件夹，项目内自带的 `.vscode` 配置同样可用。

## 学习资源

- [The Rust Programming Language（官方书）](https://doc.rust-lang.org/book/)
- [Rust 中文翻译版](https://kaisery.github.io/trpl-zh-cn/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## License

MIT
