use std::io;
use rand::Rng;
use std::fs::File;
use std::io::{Write, Read};

fn main() -> std::io::Result<()> {

    let stats = read_file()?;
    let win: i32 = stats.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
    let lose: i32 = stats.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);

    println!("Statistics (Hard Mode only) \nWins: {win}\nLosses: {lose}");

    let mut user_menu_choice: String = String::new();


    println!("Choose difficulty: \n1: Normal \n2: Hard (6 shots only)");

    io::stdin().read_line(&mut user_menu_choice)
    .expect("Failed to read user_menu_choice.");
    let user_menu_choice_num = user_menu_choice.trim().parse()
    .expect("Please enter a valid number!");

    match user_menu_choice_num {
        1=>Ok(game()),
        2=>Ok(game_hard()),
        _=>Ok(println!("Failed to choose."))
    }    
}

// game function
// idea 1: if user chooses hard mode, run same function but hard
// aspects are blocked behind if statements
// idea 2: store stats globally for easier access

fn game() {
    let rng = rand::rng().random_range(0..100);
    let mut username = String::new();
    let mut user_guess :i32 = 0;
    
    print!("Enter nickname: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut username)
    .expect("Failed to read username.");
    username = username.trim().to_string();

    println!("Hello {}", username);
    println!("Welcome to number guesser.");
    println!("You must guess the hidden number between 0 and 100.");
    
    while user_guess != rng {
        println!("Your turn.");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
        .expect("Failed to read user_input.");

        user_guess = user_input.trim().parse()
        .expect("Please enter a valid number!");

        if user_guess == rng {
            println!("Congratulations. The hidden number was: {}", rng);
            break;
        }

        if user_guess > rng {
            println!("Your guess was higher than the hidden number.")
        } else {
            println!("Your guess was lower than the hidden number.")
        }
    } 
}


fn game_hard() {
    let rng = rand::rng().random_range(0..100);
    let mut username = String::new();
    let mut user_guess :i32 = 0;
    let mut user_guess_count :u32 = 0;
    
    print!("Enter nickname: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut username)
    .expect("Failed to read username.");
    username = username.trim().to_string();

    println!("Hello {}", username);
    println!("Welcome to number guesser.");
    println!("You must guess the hidden number between 0 and 100.");
    println!("You only have 6 tries");
    
    while user_guess != rng {
        println!("Your turn.");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
        .expect("Failed to read user_input.");

        user_guess = user_input.trim().parse()
        .expect("Please enter a valid number!");

        if user_guess == rng {
            println!("Congratulations. The hidden number was: {}", rng);
            add_win_to_file()
            .expect("Failed to add win to file!");
            break;
        }

        if user_guess > rng {
            println!("Your guess was higher than the hidden number.")
        } else {
            println!("Your guess was lower than the hidden number.")
        }

        user_guess_count = user_guess_count + 1;

        if user_guess_count >= 6 {
            println!("After six tries you didn't guess the hidden number {}. You lose.", rng);
            add_loss_to_file()
            .expect("Failed to add loss to file!");
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

// win & loss funcitons
// idea 1: have one function handle each case
// idea 2: values are inserted with parameter

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
