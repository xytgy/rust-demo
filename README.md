# Rust 自动化学习环境 (Rust Auto-Builder Playground)

这是一个专为 Rust 初学者设计的高效学习与实验环境。本项目集成了一套**自动化构建系统**，能够智能管理你的代码编译流程，让你专注于编写代码，而无需操心繁琐的文件管理和编译命令。

##  核心特性

- ** 自动化构建**：只需一条 `cargo build` 命令，自动扫描并编译所有脚本。
- ** 智能归类**：
  - 源代码统一存放在 `scripts/`
  - 可执行文件自动输出到 `bin/`
  - 调试符号文件自动归档到 `debug/`
- **⚡ 增量编译**：内置智能检测机制，仅编译修改过的文件，极大提升构建速度。
- **防止污染**：保持项目根目录整洁，所有生成文件自动隔离。

##  技术栈

- **语言**：Rust (Edition 2021)
- **构建工具**：Cargo
- **自动化脚本**：自定义 `build.rs` (Rust编写)

##  快速上手 (30秒指南)

### 1. 环境准备
确保你已安装 Rust 环境 (Rustc & Cargo)。

### 2. 编写代码
在 `scripts/` 目录下新建一个 `.rs` 文件，例如 `hello_world.rs`：
```rust
fn main() {
    println!("Hello from the automated builder!");
}
```

### 3. 一键构建
在项目根目录下运行：
```powershell
cargo build
```
*系统会自动检测新文件，编译生成可执行程序，并整理相关文件。*

### 4. 运行程序
前往 `bin/` 目录运行生成的可执行文件：
```powershell
.\bin\hello_world.exe
```

##  目录结构说明

```
rust-demo/
├── scripts/       # [源文件] 存放所有的 .rs 源代码文件
├── bin/           # [输出] 存放编译好的 .exe 可执行文件
├── debug/         # [调试] 存放生成的 .pdb 调试符号文件
├── src/           # [入口] Cargo 项目占位入口
├── build.rs       # [核心] 自动化构建与文件管理脚本
├── Cargo.toml     # [配置] 项目配置文件
└── README.md      # 项目说明文档
```

##  核心配置 (build.rs)

本项目利用 `build.rs` 实现了以下自动化逻辑：
1.  **扫描** `scripts` 目录下的所有 Rust 源文件。
2.  **对比** 源文件与目标文件的修改时间（增量编译）。
3.  **调用** `rustc` 编译器生成二进制文件。
4.  **移动** `.pdb` 调试文件到独立目录，保持工作区整洁。

##  贡献指南

欢迎提交 Issue 或 Pull Request 来改进构建脚本！
1. Fork 本仓库
2. 创建你的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的修改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request

## 📄 许可证

本项目采用 MIT 许可证。详见 LICENSE 文件。
