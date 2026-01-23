use std::io;
use std::io::Write;
use rand::Rng;
fn main() {

    let mut userMenuChoice = String::new();
    let mut userMenuChoiceNum :i32 = 0;


    println!("Choose difficulty: \none: Normal \n2: Hard (not implemented)");

    io::stdin().read_line(&mut userMenuChoice)
    .expect("Failed to read userInput.");
    userMenuChoiceNum = userMenuChoice.trim().parse()
    .expect("Please enter a valid number!");

    match userMenuChoiceNum {
        1=>game(),
        _=>println!("Failed to choose.")
    }    
}

fn game() {
    let mut rng = rand::thread_rng().gen_range(0..100); // must be between 1 and 100
    let mut username = String::new();
    let mut userGuess :i32 = 0;
    
    print!("Enter nickname: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut username)
    .expect("Failed to read username.");
    username = username.trim().to_string();

    println!("Hello {}", username);
    println!("Welcome to number guesser.");
    println!("You must guess the hidden number between 0 and 100.");
    
    while userGuess != rng {
        println!("Your turn.");

        let mut userInput = String::new();
        io::stdin().read_line(&mut userInput)
        .expect("Failed to read userInput.");

        userGuess = userInput.trim().parse()
        .expect("Please enter a valid number!");

        if userGuess == rng {
            println!("Congratulations. The hidden number was: {}", rng);
            break;
        }

        if userGuess > rng {
            println!("Your guess was higher than the hidden number.")
        } else {
            println!("Your guess was lower than the hidden number.")
        }
    } 
}