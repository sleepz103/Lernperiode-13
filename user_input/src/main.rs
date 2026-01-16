// Standard input output package
use std::io;
use std::io::Write;
fn main() {
    // print line "macro"
    print!("Input your name: ");

    // some "flush" measure to allow writing on same line?
    io::stdout().flush().unwrap();

    // declare mutable "input" variable which is a new string
    let mut input = String::new(); 

    // input.standardinput read line
    // mutate input variable
    // (without &mut Rust just puts reference/copy of variable)
    // if fail to read return message
    io::stdin().read_line(&mut input)
        .expect("Failed to read user name");

    // interesting - rust captures \n (newline)
    // when user is entering input
    // use on input to combat this bug
    println!("Hello {}, nice to see you today!", input.trim());
}
