# Zonary

A minimalist binary-based esoteric programming language.  

---

## English

### Introduction

Zonary is a minimalist binary-based esoteric programming language created by Kacefier.  
Its design goal is to explore the limits of minimalism in language design.  
Written in Rust, distributed as a single-file binary with no external dependencies.  

### Features

- **Binary Registers**: Registers are named using binary numbers.  
- **Configurable Bit Width**: The program bit width can be set via preprocessor directives.  
- **Fixed-Length Instructions**: Each instruction has a fixed length.  
- **Pure Binary Code**: Code consists entirely of 0s and 1s.  
- **Preprocessor Directives**: `/00` and `/01` are used for custom characters and bit width.  
- **Memory Optimization**: Registers that have never been assigned or have been set to zero do not occupy memory.  
- **Esolang Fun**: A language designed to challenge the difficulty of writing and provide fun.  

### Instruction Set

| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| 000    | SET      | Assign a value to a register |
| 001    | ADD      | Add a value to a register |
| 010    | SUB      | Subtract a value from a register |
| 011    | JMP      | Unconditional jump or define a label |
| 100    | IFZ      | Jump if the register is zero |
| 101    | OUT      | Output a value (binary/decimal/hex/ASCII) |
| 110    | INP      | Read a value from input into a register |
| 111    | SYS      | System call (exit or delay) |

### Quick Start

First, download the appropriate executable file and installation script from the [Releases](https://github.com/Kacefier/Zonary/releases) page.  
Then run the installation script to install.  
During installation, the script will prompt you to enter the path to the executable file—simply provide the path to the downloaded executable.  

If you cannot find a suitable executable in Releases, try cloning the repository and building manually.  

### Usage Examples

| Command | Description |
|---------|-------------|
| `zonary example.zonary` | Run an example program |
| `zonary -h` | Show help information |
| `zonary -v` | Show version information |

### Precompiled Platforms

Precompiled executables are provided for the following platforms:  

- Windows (amd64)
- Linux (amd64)

If your operating system is not listed, please try compiling manually.  
Additionally, the repository provides installation and uninstallation scripts for Windows and macOS/Linux platforms.  

### Open Source License

This program is free software, licensed under the GNU General Public License v3.0.  

### Author

Kacefier  

GitHub: https://github.com/Kacefier  
Email: kacefier@zohomail.com  

---

## 中文

### 简介

Zonary 是一个基于二进制的极简整活编程语言，由 Kacefier 制作。  
它的设计目的是探索语言设计中极简主义的边界。  
使用 Rust 语言编写，以单文件二进制形式分发，无需外部依赖。  

### 特点

- **二进制寄存器**：寄存器使用二进制数字命名。  
- **可配置位宽**：可通过预处理语句设置程序位数。  
- **定长指令**：每条指令长度固定。  
- **纯二进制代码**：代码全部由 0 和 1 组成。  
- **预处理语句**：`/00` 和 `/01` 用于自定义字符和位宽。  
- **内存优化**：从未赋值或被清零的寄存器不占用内存。  
- **整活乐趣**：一门旨在挑战书写难度和带来乐趣的语言。  

### 指令集

| 操作码 | 助记符 | 说明 |
|--------|--------|------|
| 000    | SET    | 给寄存器赋值 |
| 001    | ADD    | 寄存器加一个值 |
| 010    | SUB    | 寄存器减一个值 |
| 011    | JMP    | 无条件跳转或定义标记 |
| 100    | IFZ    | 寄存器为零时跳转 |
| 101    | OUT    | 输出值（二进制/十进制/十六进制/ASCII） |
| 110    | INP    | 从输入读取值到寄存器 |
| 111    | SYS    | 系统调用（退出或延迟） |

### 快速开始

请先到 [Releases](https://github.com/Kacefier/Zonary/releases) 页面下载合适的可执行文件和安装脚本。  
然后运行安装脚本进行安装。  
安装时，安装脚本会提示输入可执行文件的路径，输入下载的可执行文件的路径即可。  

如果你没有在 Releases 里找到合适的可执行文件，请尝试克隆仓库后手动编译。  

### 使用示例

| 命令 | 说明 |
|------|------|
| `zonary example.zonary` | 运行示例程序 |
| `zonary -h` | 显示帮助信息 |
| `zonary -v` | 显示版本信息 |

### 预编译平台

发布页提供以下平台的预编译可执行文件：  

- Windows（amd64）
- Linux（amd64）

如果没有你使用的操作系统，请尝试手动编译。  
同时，仓库也提供 Windows 和 macOS/Linux 平台的安装与卸载脚本。  

### 开源许可证

本程序是自由软件，采用 GNU 通用公共许可证 v3.0 授权。  

### 作者

Kacefier  

GitHub：https://github.com/Kacefier  
邮箱：kacefier@zohomail.com  
