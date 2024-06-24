# 使用结构体组织关联数据
[TOC]
## 定义并实例化结构体
定义结构体，需要使用`struct`关键字并为整个结构体提供一个名字。在大括号中，定义每一部分数据的名字和类型，我们称为**字段**
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
要在定义结构体后使用它，我们可以通过为每个字段指定具体值的方式来创建该结构体的**实例**,为了从结构体中获取某个特定的值，可以使用点号
```rust
fn main(){
    let mut user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```
注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变。
我们可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例。
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}
```