// Standard input output package
use std::io;
use std::io::Write;
fn main() {
    let mut name = String::new();
    let mut nickname = String::new();


    print!("Input your name: ");
    io::stdout().flush().unwrap();
 

    io::stdin().read_line(&mut name)
    .expect("Failed to read user name");
    name = name.trim().to_string();

    println!("Hello {}, nice to see you today!", name);

    if name.chars().count() > 3 {
        println!("Your name is {} characters long. Try making it less than 3", name.chars().count());
        print!("Input your nickname: ");
        io::stdout().flush().unwrap(); 


        
        io::stdin().read_line(&mut nickname)
        .expect("Failed to read nickname");
        nickname = nickname.trim().to_string();

        println!("Hello {}!", nickname);
    }

}
