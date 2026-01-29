use std::fs::File;
use std::io::{Write, Read}; // Write has write_all()


fn main() -> std::io::Result<()> {

    let mut file = File::open("stats.txt")?;
    let mut statistics = String::new();
    file.read_to_string(&mut statistics)?;

    let v: Vec<&str> = statistics.split(',').collect();

    let mut win :i32  = v.get(0).unwrap_or(&"0").parse().unwrap_or(0);
    let mut lose :i32 = v.get(1).unwrap_or(&"0").parse().unwrap_or(0);
    println!("File content: {}", statistics);

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