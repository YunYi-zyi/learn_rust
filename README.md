# 🦀 Learn Rust - Rust 语言学习项目

这是一个交互式的 Rust 语言学习项目，包含了结构化的练习和示例代码，帮助你逐步掌握 Rust 编程语言的核心概念。

## 📋 项目特色

- 🎯 **交互式学习**：通过菜单系统选择和运行特定练习
- 📚 **结构化内容**：每个概念都有独立的模块和练习
- 🌏 **双语支持**：中英文双语界面和注释
- 🔧 **易于扩展**：模块化设计，方便添加新的练习
- ✅ **渐进式学习**：从基础到高级，循序渐进

## 🏗️ 项目结构

```
learn_rust/
├── src/
│   ├── main.rs                 # 主程序入口，交互式菜单系统
│   └── exercises/              # 练习模块目录
│       ├── mod.rs              # 模块声明和练习管理
│       ├── variables.rs        # 变量和常量练习
│       ├── strings.rs          # 字符串操作练习
│       ├── loops.rs            # 循环结构练习
│       ├── functions.rs        # 函数定义练习
│       └── conditionals.rs     # 条件语句练习
├── Cargo.toml                  # 项目配置文件
├── Cargo.lock                  # 依赖锁定文件
└── README.md                   # 项目说明文档
```

## 🚀 快速开始

### 前置要求

确保你已经安装了 Rust 工具链：

```bash
# 安装 Rust（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version
```

### 运行项目

```bash
# 克隆项目（如果从 GitHub 获取）
git clone https://github.com/YunYi-zyi/learn_rust.git
cd learn_rust

# 编译并运行交互式学习系统
cargo run

# 仅编译项目
cargo build

# 运行测试（如果有）
cargo test
```

## 📖 当前练习内容

| 编号 | 练习主题                           | 文件位置                        | 状态 |
| ---- | ---------------------------------- | ------------------------------- | ---- |
| 1    | 变量和常量 (Variables & Constants) | `src/exercises/variables.rs`    | ✅   |
| 2    | 字符串操作 (String Operations)     | `src/exercises/strings.rs`      | ✅   |
| 3    | 循环结构 (Loops)                   | `src/exercises/loops.rs`        | ✅   |
| 4    | 函数定义 (Functions)               | `src/exercises/functions.rs`    | ✅   |
| 5    | 条件语句 (Conditionals)            | `src/exercises/conditionals.rs` | ✅   |
| 6    | 所有权（Ownership）                | `src/exercises/ownership.rs`    | ✅   |
| 7    | 结构体 (Structs)                   | `src/exercises/structlearn.rs`  | ✅   |
| 8    | 枚举（Enums）                      | `src/exercises/enums.rs`        | ✅   |
| 9    | 模式匹配（Pattern Matching）       | `src/exercises/matchln.rs`      | ✅   |
| 10   | 向量（Vector）                     | `src/exercises/vect.rs`         | ✅   |

## ➕ 如何添加新练习

按照以下步骤可以轻松添加新的练习模块：

### 第一步：创建练习文件

在 `src/exercises/` 目录下创建新的练习文件，例如 `structs.rs`：

```rust
// src/exercises/structs.rs

/// 结构体和枚举练习
pub fn run() {
    println!("🏗️  结构体和枚举练习");
    println!("Structs and Enums Exercise");
    println!("{}", "=".repeat(40));

    // 定义结构体
    struct Person {
        name: String,
        age: u32,
        email: String,
    }

    // 创建结构体实例
    let person = Person {
        name: String::from("张三"),
        age: 25,
        email: String::from("zhangsan@example.com"),
    };

    println!("👤 Person Info:");
    println!("   Name: {}", person.name);
    println!("   Age: {}", person.age);
    println!("   Email: {}", person.email);

    // 定义枚举
    enum Color {
        Red,
        Green,
        Blue,
        Rgb(u8, u8, u8),
    }

    let colors = vec![
        Color::Red,
        Color::Green,
        Color::Rgb(255, 165, 0),
    ];

    println!("\n🎨 Colors:");
    for color in colors {
        match color {
            Color::Red => println!("   红色 (Red)"),
            Color::Green => println!("   绿色 (Green)"),
            Color::Blue => println!("   蓝色 (Blue)"),
            Color::Rgb(r, g, b) => println!("   RGB({}, {}, {})", r, g, b),
        }
    }
}
```

### 第二步：更新模块声明

在 `src/exercises/mod.rs` 文件中添加新模块：

```rust
// 添加模块声明
pub mod structs;  // 新增这一行

// 添加导出
pub use structs::run as run_structs;  // 新增这一行

// 更新练习列表
pub const EXERCISES: &[(&str, &str)] = &[
    ("1", "变量和常量 (Variables and Constants)"),
    ("2", "字符串操作 (String Operations)"),
    ("3", "循环结构 (Loops)"),
    ("4", "函数定义 (Functions)"),
    ("5", "条件语句 (Conditionals)"),
    ("6", "结构体和枚举 (Structs and Enums)"),  // 新增这一行
];

// 更新运行函数
pub fn run_exercise(choice: &str) -> bool {
    match choice {
        "1" => { run_variables(); true }
        "2" => { run_strings(); true }
        "3" => { run_loops(); true }
        "4" => { run_functions(); true }
        "5" => { run_conditionals(); true }
        "6" => { run_structs(); true }  // 新增这一行
        _ => false,
    }
}
```

### 第三步：编译和测试

```bash
# 编译检查语法
cargo check

# 运行项目测试新练习
cargo run
```

## 🎯 练习编写规范

为了保持项目的一致性，请遵循以下编写规范：

### 1. 文件结构

- 每个练习文件都应该有一个 `pub fn run()` 函数作为入口点
- 使用双语注释（中文和英文）
- 添加适当的输出格式化

### 2. 函数模板

```rust
/// 练习描述（中文）
/// Exercise Description (English)
pub fn run() {
    println!("🎯 练习标题");
    println!("Exercise Title");
    println!("{}", "=".repeat(40));

    // 练习代码

    println!("\n✅ 练习完成！");
}
```

### 3. 输出格式

- 使用 emoji 和分隔线增强可读性
- 提供中英文对照的输出信息
- 使用适当的缩进和空行

## 🔮 计划中的练习

以下是计划添加的练习主题：

- [ ] 结构体和枚举 (Structs & Enums)
- [ ] 所有权系统 (Ownership System)
- [ ] 借用和引用 (Borrowing & References)
- [ ] 模式匹配 (Pattern Matching)
- [ ] 错误处理 (Error Handling)
- [ ] 泛型编程 (Generics)
- [ ] Trait 系统 (Traits)
- [ ] 生命周期 (Lifetimes)
- [ ] 闭包和迭代器 (Closures & Iterators)
- [ ] 智能指针 (Smart Pointers)
- [ ] 并发编程 (Concurrency)
- [ ] 异步编程 (Async Programming)
- [ ] 模块系统 (Module System)
- [ ] 测试 (Testing)
- [ ] 文件I/O (File I/O)

## 🛠️ 开发环境

- **Rust Edition**: 2024
- **包管理器**: Cargo

## 📚 学习资源

### 官方文档

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)
- [Cargo 手册](https://doc.rust-lang.org/cargo/)

### 中文学习资源

- [Rust 程序设计语言（中文版）](https://kaisery.github.io/trpl-zh-cn/)
- [通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/)
- [Rust 语言圣经](https://course.rs/)
- [Rust 死灵书](https://nomicon.purewhite.io/)

### 在线工具

- [Rust Playground](https://play.rust-lang.org/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust by Practice](https://practice.rs/)

## 🤝 贡献指南

欢迎贡献新的练习或改进现有内容！

1. Fork 这个项目
2. 创建你的特性分支 (`git checkout -b feature/new-exercise`)
3. 按照上述规范添加新练习
4. 提交你的更改 (`git commit -am 'Add new exercise: topic'`)
5. 推送到分支 (`git push origin feature/new-exercise`)
6. 创建一个 Pull Request

## 📝 更新日志

### v0.1.0 (2024-12-XX)

- ✨ 初始版本发布
- ✅ 实现交互式学习系统
- ✅ 添加基础练习：变量、字符串、循环、函数、条件语句
- 📚 完善项目文档

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

感谢所有为 Rust 语言发展做出贡献的开发者们，以及提供优质学习资源的社区。

---

**Happy Coding! 🦀**
_这是一个个人学习项目，用于记录和分享 Rust 语言的学习历程。如果这个项目对你有帮助，请给个 ⭐ Star！_
