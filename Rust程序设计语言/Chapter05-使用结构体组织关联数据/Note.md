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
## 一个使用结构体的示例程序
编写一个计算长方形面积的程序：使用`cargo`新建一个叫做*rectangles*的二进制程序，它获取以像素为单位的长方形的宽度和高度，并计算出长方形的面积。

## 方法语法
方法与函数类似：它们使用`fn`关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。但是方法和函数不同，它们在结构体的上下文中被定义(或是枚举或`trait`对象的上下文)，并且它们第一个参数总是`self`,它代表调用该方法的结构体实例
### 定义方法
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
为了使函数定义于`Rectangle`的上下文，使用了一个`impl`块(implementation的缩写)，这个`impl`块中的所有内容都将与`Rectangle`类型相关联。
这里选择`&self`是因为我们不想获取所有权，只希望能够读取结构体中的数据，而不是写入。若想要在方法中改变调用方法的实例，需要将第一个参数改为`&mut self`。
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```
与字段同名的方法将被定义为只返回字段中的值，这样的方法被称为`getters`，`getters`可以把字段变成私有的，但方法是公共的，这样就可以把对字段的只读访问作为该类型公共API的一部分。
### 带有更多参数的方法
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```
### 关联函数
所有在`impl`块中定义的函数被称为**关联函数**，它们与`impl`后面命名的类型相关。我们可以定义不以 `self` 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例。我们已经使用了一个这样的函数，`String::from` 函数，它是在 `String` 类型上定义的。
关联函数经常被用作返回一个结构体新实例的构造函数。
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
}
```
使用结构体名和 `::` 语法来调用这个关联函数：比如 `let sq = Rectangle::square(3);`。这个方法位于结构体的命名空间中：`::` 语法用于关联函数和模块创建的命名空间。
### 多个impl块
每个结构体都允许拥有多个`impl`块，但每个方法有其自己的`impl`块
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

