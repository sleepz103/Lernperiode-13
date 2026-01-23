use std::fs::File;
use std::io::{self, Write, Read}; // Write has write_all()


fn main() -> std::io::Result<()> {

    let mut file = File::open("stats.txt")?;
    let mut statistics = String::new();
    file.read_to_string(&mut statistics)?;
    println!("File content:\n{}", statistics);

    let win = statistics.chars().nth(0).unwrap();  // First character (win)
    let lose = statistics.chars().nth(2).unwrap(); // Third character (lose, after comma)

    println!("Wins: {}", win);
    println!("Losses: {}", lose);

    
    win = win + 1;
    lose = lose + 1;

    let new_stats = win.push_str(lose)

    let mut file = File::create("stats.txt")?;
    file.write_all(stats.as_bytes())?;
    println!("Message written to file.");

    
    
    Ok(())
}