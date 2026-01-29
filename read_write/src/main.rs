use std::fs::File;
use std::io::{Write, Read}; // Write has write_all()


fn main() -> std::io::Result<()> {

    let mut file = File::open("stats.txt")?;
    let mut statistics = String::new();
    file.read_to_string(&mut statistics)?;

    // wrong: What if bigger that one char, for ex. 10?
    let mut win: i32 = statistics.chars().nth(0).unwrap() as i32;  // First character (win)
    let mut lose: i32 = statistics.chars().nth(2).unwrap() as i32; // Third character (lose, after comma)

    // Displays correctly
    println!("File content: {}", statistics);

    // Error displays: 0,0 in file turns to 52 and 44
    println!("Wins: {}", win);
    println!("Losses: {}", lose);

    
    win = win + 1;
    lose = lose + 1;

    let new_stats: String = String::from(win.to_string() + "," + &lose.to_string());

    let mut file: File = File::create("stats.txt")?;
    file.write_all(new_stats.as_bytes())?;
    println!("Message written to file.");

    
    
    Ok(())
}