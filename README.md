# project_builder

这是一个使用 Rust 的过程宏生成 Builder 模式代码的示例项目。该项目包括两个库：
- `builder_macro`：定义过程宏以生成 Builder 模式代码。
- `my_project`：使用 `builder_macro` 库定义结构体并进行测试。

## 安装

首先，克隆这个项目到你的本地机器：

```bash
git clone https://github.com/yourusername/project_builder.git
cd project_builder
```
然后，进入 builder_macro 目录并构建它：

```bash
cd builder_macro
cargo build
```
接下来，进入 my_project 目录并构建它：

```bash
cd ../my_project
cargo build
```
用法
在 my_project 中，我们定义了两个结构体 Person 和 Company，并使用 builder_macro 中的 Builder 过程宏生成它们的 Builder 模式代码。
你可以在你的 Rust 项目中通过以下方式使用 builder_macro 过程宏：

```rust
use builder_macro::Builder;

#[derive(Debug, PartialEq, Builder)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

#[derive(Debug, PartialEq, Builder)]
pub struct Company {
    pub name: String,
    pub founded: u32,
    pub ceo: String,
    pub employees: usize,
    pub founder: Person,
}

impl Company {
    pub fn new(name: String, founded: u32, ceo: String, employees: usize, founder: Person) -> Self {
        Company { name, founded, ceo, employees, founder }
    }
}
```
示例
下面是如何使用生成的 Builder 模式代码创建 Person 和 Company 对象的示例：
```rust
let person = Person::builder()
    .name("Alice".to_string())
    .age(30)
    .build()
    .unwrap();

let company = Company::builder()
    .name("Tech Corp".to_string())
    .founded(2000)
    .ceo("Bob".to_string())
    .employees(100)
    .founder(person)
    .build()
    .unwrap();
```
测试
你可以使用以下命令运行测试：

```bash
cargo test
```
测试包括验证使用 new 方法和 Builder 模式创建的对象是否相等。
