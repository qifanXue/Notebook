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
### 变量与字段同名时的字段初始化简写语法
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
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
### 使用结构体更新语法从其他实例创建实例
使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例。
1. 不使用结构体更新语法
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```
2. 使用结构体更新语法
`..` 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```
在这个例子中，我们在创建`user2`后不能再使用`user1`，因为`user1`的`username`字段中的`String`被移到`user2`中；若我们只使用`user1`的`active`和`sign_in_count`值，那么`user1`在创建`user2`后仍然有效，因为`active`和`sign_in_count`的类型是实现`Copy`trait的类型。
### 使用没有命名字段的元组结构体来创建不同的类型
元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
注意`black`和`origin`值的类型不同，因为它们是不同的元组结构体的实例。一个获取`Color`类型参数的函数不能接受`Point`作为参数。在其他方面，元组结构体实例类似于元组：可以将其解构为单独的部分，也可以使用`.`后跟索引来访问单独的值。
### 没有任何字段的类单元结构体
类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```