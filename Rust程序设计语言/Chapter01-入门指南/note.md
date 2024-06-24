# 入门指南
## 本章重点
1. 如何使用cargo创建rust项目
2. 如何编译运行rust项目

[TOC]
## Hello，Cargo！
Cargo是Rust的构建系统和包管理器。Cargo可以帮助处理很多任务，如构建代码、下载依赖库以及编译这些库
### 使用Cargo创建项目
1. 进入存放代码的目录,创建项目hello_cargo
```rust
$ cargo new hello_cargo //cargo new + 项目名称
```
进入*hello_cargo*目录，看到cargo生成了文件和目录，并初始化了一个Git仓库：
* Git仓库：带有一个 *.gitignore* 文件
* *Cargo.toml*文件
```rust
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]

```
* *src/main.rs*
```
fn main() {
    println!("Hello, world!");
}

```
### 构建并运行Cargo项目
1. ```cargo bulid``` 构建项目
```rust
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```
这个命令会在*target/debug/hello_cargo*下创建一个可执行文件
使用以下命令来运行它：
```rust
$ ./target/debug/hello_cargo //或者在 Windows 下为 .\target\debug\hello_cargo.exe
Hello, world!
```
首次运行 ```cargo build``` 时，Cargo会在项目根目录创建一个新文件：*Cargo.lock*，这个文件记录项目依赖的实际版本。
2. ```cargo run``` 一次性完成代码编译和运行的操作
```rust
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```
3. ```cargo check``` 快速检查代码确保其可以编译，但并不产生可执行文件
```rust
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```
### 发布构建
当项目最终准备好发布时，可以使用 ```cargo build --release```,这会在*target/debug*下生成可执行文件，消耗更长的编译时间，但Rust代码运行的更快