---
title: My Rust Tutorial
---

# Goal

In this tutorial, you will learn how to create a number guesser in Rust. For those who want to take a deeper dive, you will learn how to perform read/write with an external file. 

# Previous Knowledge

Rust is a system language. It is lower-level than, say, C# for example. Still, this tutorial welcomes everyone who's familiar with one procedural language (C#/Javascript/Python/where-code-goes-one-by-one) and program flow understandment (like if-else cases, loops and functions) and who wants to (and their patience) with ambigous syntax and lower-level code.  

This language will be familiar to C# users for it has similar syntax. It also will deny compiling if you got an error, which isn't the case in Javascript.
Though, if you wrote a program before, you're good to go. 

# What you'll learn

Your Rust adventure goes in following order:
- Necessaries for working with Rust/Environment Setup 
    - cargo
    - Rust Syntax
    - rust-analyzer
    - rust debugger: CodeLLDB
- Data and Types
- While, if-else
- Functions


(
First I'll show you necessaries for your environment. I recommend VSC.

This way you will safely compile a "Hello World" program.
Then we will move focus to data types.
Then a short trip to while, if-else
Next there will be explanation on functions
)

# Tutorial

## Environment Setup

First I'll show you necessaries for your environment. You begin by installing the language from the website.

https://rust-lang.org/learn/get-started/

![welcome_rust](/assets/welcome_rust.png)

![installation](/assets/installation.png)

Here choose one and wait. The basic installation comes with language itself and `cargo`. `Cargo` is your project manager and game changer.

Generally, Rust code has to be compiled into machine-readable  code which can be executed via terminal. You type:
```
rustc your_code.rs
```
to make .exe file from .rs file
then run
```
your_code
```
to see your work in action
```
Hello, world!
```
That's a lot of work though.

### Cargo
Instead, you use cargo to do the work.  
`cargo new projectName` init new project  
It creates not only `main.rs`, but also manages packages in a structure like
```
projectName
|   Cargo.lock
│   Cargo.toml
│
├───src
│       main.rs
│
└───target
```
`main.rs` is where you write your code, `cargo.lock` stores your packages (imports). Rest is to be safely ignored.

Then, inside your project directory, you can run `cargo build` to compile and build .exe from .rs. However, while easier, what you really want to do is `cargo run` to compile and run from project folder directly. No need for changing directories.

### .gitignore
For those who work with git, I mentioned that `main.rs` and `cargo.lock` are only important. Here's where to place `your .gitignore`
```
mainFolder
│    .gitignore
├─── projectName1
├─── projectName2
└─── helloWorld
```
Here's how to ignore Rust byproducts:
```
*.exe
*.pdb

**/*.rs.bk
*.toml
*/target
```

### IDE

For your IDE (environment to develop your code) I recommend Visual Studio Code. I myself used it and it's life saving when you have:
    - Rust Syntax
    - rust-analyzer
    - debugger: CodeLLDB

![vsc_extensions](/assets/vsc_extensions.png)

Find these on marketplace. When you got rust, cargo and IDE + extensions setup, you're ready for next steps.

## Data and Types

## While and if-else

## Functions


# Result

# What could go wrong?
