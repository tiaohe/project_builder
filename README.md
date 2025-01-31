# project_builder [![CI Status](https://github.com/tiaohe/project_builder/actions/workflows/rust.yml/badge.svg)](https://github.com/tiaohe/project_builder/actions)

> Rust过程宏实现的构建者模式代码生成器 | Builder pattern code generator via Rust procedural macros

[English](./README.md) | 简体中文

## 特性 ✨

- 通过`#[derive(Builder)]`自动生成构建者模式代码
- 支持默认值、字段验证、链式调用
- 友好的编译错误提示
- 零运行时开销
- 支持Rust 1.65+

## 安装 ⚙️

在`Cargo.toml`中添加：

```toml
[dependencies]
builder_macro = { path = "./builder_macro" }
```

## 使用示例 🚀

### 基础用法

```rust
use builder_macro::Builder;

#[derive(Builder)]
struct User {
    id: u64,
    username: String,
    #[default = "active"]  // 支持默认值
    status: String,
    #[validate(length > 8)]  // 支持验证
    password: String
}

let user = User::builder()
    .id(1)
    .username("tiaohe".to_string())
    .password("strongpass123")
    .build()
    .unwrap();  // 验证失败时会返回Err
```

### 高级功能

```rust
#[derive(Builder)]
#[builder(entry_name = "new")]  // 自定义入口方法名
struct Task {
    #[default = "Untitled"]
    name: String,
    #[validate(range(1..=100))]
    priority: u8,
    #[builder(each = "tag")]  // 支持集合类型
    tags: Vec<String>
}

let task = Task::new()
    .priority(90)
    .tag("urgent")
    .tag("important")
    .build()
    .unwrap();
```

## 开发指南 🛠️

### 运行测试

```bash
cargo tests --all
```

## 贡献 🤝

欢迎通过Issue和PR参与贡献！请确保：
1. 通过`cargo fmt`和`cargo clippy`检查
2. 添加对应的测试用例
3. 更新相关文档

## 许可证 📜

MIT License © [tiaohe](https://github.com/tiaohe)