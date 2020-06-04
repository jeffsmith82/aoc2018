use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    part1(&buffer)?;
    part2(&buffer)?;
    Ok(())
}

fn part1(s: &str) -> Result<()> {
    let mut freq = 0;
    for line in s.lines() {
        let change: i32 = line.parse()?;
        freq += change;
    }
    println!("Count: {}", freq);
    Ok(())
}

fn part2(s: &str) -> Result<()> {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    loop {
        for line in s.lines() {
            let change: i32 = line.parse()?;
            freq += change;
            if seen.contains(&freq) {
                println!("repeated: {}", freq);
                return Ok(());
            }
            seen.insert(freq);
        }
    }
}
