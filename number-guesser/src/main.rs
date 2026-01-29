use std::io;
use rand::Rng;
use std::fs::File;
use std::io::{Write, Read};

fn main() -> std::io::Result<()> {

    let stats = read_file()?;
    let win: i32 = stats.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
    let lose: i32 = stats.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);

    println!("Statistics (Hard Mode only) \nWins: {win}\nLosses: {lose}");

    let mut userMenuChoice = String::new();
    let mut userMenuChoiceNum :i32 = 0;


    println!("Choose difficulty: \n1: Normal \n2: Hard (6 shots only)");

    io::stdin().read_line(&mut userMenuChoice)
    .expect("Failed to read userInput.");
    userMenuChoiceNum = userMenuChoice.trim().parse()
    .expect("Please enter a valid number!");

    match userMenuChoiceNum {
        1=>Ok(game()),
        2=>Ok(gameHard()),
        _=>Ok(println!("Failed to choose."))
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


fn gameHard() {
    let mut rng = rand::thread_rng().gen_range(0..100); // must be between 1 and 100
    let mut username = String::new();
    let mut userGuess :i32 = 0;
    let maxGuess :u32 = 6;
    let mut userGuessCount :u32 = 0;
    
    print!("Enter nickname: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut username)
    .expect("Failed to read username.");
    username = username.trim().to_string();

    println!("Hello {}", username);
    println!("Welcome to number guesser.");
    println!("You must guess the hidden number between 0 and 100.");
    println!("You only have 6 tries");
    
    while userGuess != rng {
        println!("Your turn.");

        let mut userInput = String::new();
        io::stdin().read_line(&mut userInput)
        .expect("Failed to read userInput.");

        userGuess = userInput.trim().parse()
        .expect("Please enter a valid number!");

        if userGuess == rng {
            println!("Congratulations. The hidden number was: {}", rng);
            add_win_to_file();
            break;
        }

        if userGuess > rng {
            println!("Your guess was higher than the hidden number.")
        } else {
            println!("Your guess was lower than the hidden number.")
        }

        userGuessCount = userGuessCount + 1;

        if userGuessCount >= 6 {
            println!("After six tries you didn't guess the hidden number {}. You lose.", rng);
            add_loss_to_file();
            break;
        }
    }
}

fn read_file () -> std::io::Result<Vec<String>>{

    let mut file = File::open("stats.txt")?;
    let mut statistics = String::new();
    file.read_to_string(&mut statistics)?;
    let v: Vec<String> = statistics.split(',').map(|s| s.to_string()).collect();
    Ok(v)

}

fn add_win_to_file () -> Result<(), Box<dyn std::error::Error>> {

    let stats = read_file()?;
    let win: i32 = stats.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
    let lose: i32 = stats.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
  
    let win = &win + 1;

    let new_stats: String = String::from(win.to_string() + "," + &lose.to_string());

    let mut file: File = File::create("stats.txt")?;
    file.write_all(new_stats.as_bytes())?;
    println!("Statistics written to file.");
    Ok(())
}

fn add_loss_to_file () -> Result<(), Box<dyn std::error::Error>> {

    let stats = read_file()?;
    let win: i32 = stats.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
    let lose: i32 = stats.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
  
    let lose = &lose + 1;

    let new_stats: String = String::from(win.to_string() + "," + &lose.to_string());

    let mut file: File = File::create("stats.txt")?;
    file.write_all(new_stats.as_bytes())?;
    println!("Statistics written to file.");
    Ok(())
}
