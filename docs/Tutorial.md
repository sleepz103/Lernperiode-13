---
title: My Rust Tutorial
---
(This guide is based on Windows 11 System.)
# Goal

In this tutorial, you will learn how to create a number guesser game in Rust. 
For those who want to take a deeper dive, there's an extra extion to learn how to perform read/write with an external file. 

# Previous Knowledge

Rust is a system language. It is lower-level than, say, C# for example. 
Still, this tutorial welcomes everyone who:
- is familiar with procedural languages (C#/Javascript/Python/where-code-goes-one-by-one)
- has program flow understandment (like if-else cases, loops and functions)
- wants to learn new language (and their patience) with ambigous syntax and lower-level code.  

Spoiler: This language will be familiar to C# users for it has similar syntax. It also will deny compiling if you got an error, which isn't the case in Javascript.
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

# Tutorial

## Environment Setup

First I'll show you necessaries for your environment. You begin by installing the language from the website.

https://rust-lang.org/learn/get-started/

<img src="/assets/welcome_rust.png" alt="welcome_rust">

<img src="/assets/installation.png" alt="installation">

Here, choose `1` and wait. The basic installation comes with language itself and `cargo`. `cargo` is your project manager and game changer.

**After that, feel free to go to your prefered directory (folder where you want to create your project) and open a command line. You will work with terminal to run your code.**


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
`cargo new projectName` initalies new project  
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

### Hello World!

In terminal, run `cargo new my_firt_rust_project`.
Then move to that directory with `cd my_firt_rust_project`.
In `src` folder you can find main.rs with
```rust
fn main() {
    println!("Hello, world!");
}
```
Running `cargo run` here (where you currently are, without going to src folder) will output classic "Hello, world!".

### .gitignore
For those who work with git, I mentioned that `main.rs` and `cargo.lock` are only important. Here's where to place your `.gitignore`
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

<img src="/assets/vsc_extensions.png" alt="vsc_extensions">

Find these on marketplace. When you got rust, cargo and IDE + extensions setup, you're ready for next steps.

## Data and Types

### Trivia
First of all, what kind of variable is this?  
`let win_count :i32 = 0;`
If your thought process was "well `let` must be declaring a variable and `i32` reminds me of integers, then it must be a variable integer assigned to value 0" then you're not wrong. But you're close.  
**It's a constant, so called "unmutable", integer**
This is a very important concept of Rust. Rust likes variables to be constant at all times possible. 

### Constant Variables
Let's begin with creating (constant) variables. 
```rust
let win :i32 = 0; // this is a constant signed integer 32
let lose :u32 = 0; // this is a constant unsigned integer 32
let win_flag :bool = false; // this is a constant bool
let user_input :String = String::new(); // this is a constant empty string
// notice 1: Rust likes variables and function names in snake case. Else, you will receive a warning
// notice 2: Rust is case-sensitive. pay attention when writing :String 
```
### Signed/Unsigned Integers
You know how when you do math addition, like 1 + 2 + 4 + 60, you write the pluses, but don't write each numbers sign (+/-) that defines if it's positive or negative? In such addition you assume every number is positive.  
That's the exact case with Rust integers:
u32 = unsigned integer, assuming that the value will be 0 + positive only
Math with i32 would look like 2 + 3 - 1 = 4
i32 = signed integer, acounting the value can be negative
Math with i32 would look like (-2) + (-3) - (-1) + (+10) = (+6)

### Mutable Variables
"Stop yapping and tell me how to change variables"

```rust
let mut win :i32 = 0; // this is a mutable signed integer 32
let mut lose :u32 = 0; // this is a mutable unsigned integer 32
let mut win_flag :bool = false; // this is a mutable bool
let mut user_input :String = String::new(); // this is a mutable string
```

In Rust you need to define that your variable is expected to change. A good explanation why you can find here: [Rust Tutorial #9 - Memory Management, Heap & Stack](https://www.youtube.com/watch?v=-6cnnNlAvNk&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=10)

## While and if-else

These two are similar to C#

while value1 (logic check here) value2 { }
```rust
while win_flag != true {
    // cool code here
}
```
if value1 (logic check here) value 2 { }
```rust
if user_guess > rng {
    println!("Your guess was higher than the hidden number.")
} else {
    println!("Your guess was lower than the hidden number.")
}
```

## Functions

# Result

# What could go wrong?
