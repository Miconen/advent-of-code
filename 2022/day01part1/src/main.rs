use std::fs;
use std::cmp;

fn main() {
    let file = "./input.txt";
    println!("Reading file {}", file);

    let lines = match fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Failed to read file: {}", error);
            return;
        }
    };

    let mut max: u32 = 0;
    let mut sum: u32 = 0;
    for line in lines.lines() {
        sum += line.parse::<u32>().unwrap_or_else(|_| {
            println!("Sum: {} | Max: {}", sum, max);
            max = cmp::max(sum, max);
            sum = 0;
            0
        });
    }

    // Information about how many valid lines there are
    let oks = lines.lines().filter(|line| { line.parse::<u32>().is_ok() }).count();
    let errors = lines.lines().filter(|line| { line.parse::<u32>().is_err() }).count();
    println!("Oks: {} | Errors: {}", oks, errors);
    println!("Biggest: {}", max);
}
