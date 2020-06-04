use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    part1(&buffer)?;
    Ok(())
}

fn part1(s: &str) -> Result<()> {
    let mut twos = 0;
    let mut threes = 0;
    let mut counts = HashMap::new();

    for line in s.lines() {
        for c in line.chars() {
            //*counts.entry(c).default(0) += 1;
            match counts.get_mut(&c) {
                Some(v) => *v += 1,
                None => {
                    counts.insert(c, 1);
                    ()
                }
            }
        }
        let mut found2 = false;
        let mut found3 = false;
        for (_, v) in counts.iter() {
            if *v == 2 && found2 == false {
                twos += 1;
                found2 = true;
            } else if *v == 3 && found3 == false {
                threes += 1;
                found3 = true;
            }
        }
        counts.clear();
    }
    println!("{} * {}: result: {}", twos, threes, twos * threes);

    Ok(())
}
