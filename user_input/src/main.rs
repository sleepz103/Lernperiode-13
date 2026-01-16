// Standard input output package
use std::io;
fn main() {
    // print line "macro"
    println!("Hello, world!");
    // declare mutable "input" variable which is a new string
    let mut input = String::new(); 

    // input.standardinput read line
    // mutate input variable
    // (without &mut Rust just puts reference/copy of variable)
    // if fail to read return message
    io::stdin().read_line(&mut input).expect("failed to read line");;
    println!("{}",input);
}
