# Rust 学习笔记与练习

这个项目是我学习 Rust 编程语言的练习集。记录了从环境配置到基础语法的探索过程。

## 目录结构

- `hello.rs`: 我的第一个 Rust 程序（Hello World）。
- `demo1.rs`: 基础语法练习文件（变量声明、循环等）。
- `greeting/`: 使用 Cargo 创建的标准项目模板。

##  环境配置记录

在 Windows 上配置 Rust 时遇到的关键问题及解决方法：

1. **链接器冲突**：
   - 报错：`error: linking with link.exe failed`。
   - 原因：系统误用了 MSYS2 或 Git 的 `link.exe`。
   - 解决：确保 Visual Studio 的 MSVC 工具链路径在环境变量中处于优先位置，或使用 `Developer PowerShell for VS 2022`。

2. **必需组件**：
   - 安装了 Visual Studio Build Tools，并勾选了“使用 C++ 的桌面开发”以及 Windows SDK。

##  如何运行

### 1. 编译单个文件
使用 Rust 编译器直接编译：
```powershell
rustc hello.rs
.\hello.exe
```

### 2. 使用 Cargo 管理项目
进入项目目录并运行：
```powershell
cd greeting
cargo run
```
