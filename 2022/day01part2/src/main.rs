use std::fs;

// I really felt limited by my Rust knowledge on this one
// resulting in a lot more spaghetti code than i would normally
// be comfortable having someone else read or debug.

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

    let mut max: [u32; 3] = [0, 0, 0];
    let mut sum: u32 = 0;
    for line in lines.lines() {
        // Sum all lines until blank line
        sum += line.parse::<u32>().unwrap_or_else(|_| {
            // Encountered non-number value, blank line in this case
            println!("Sum: {} | Max: {:?}", sum, max);

            // This whole if statement thing feels very spaghetti
            // Line is only bigger than the 3rd entry
            // Replace 3rd with sum
            if max[2] < sum && max[1] > sum {
                max[2] = sum;
            }
            // Line is bigger than the 2nd & 3rd entry
            // Move 2nd to 3rd and replace 2nd with sum
            else if max[1] < sum && max[0] > sum {
                max[2] = max[1];
                max[1] = sum;
            }
            // Line is bigger than all entries
            // Rotate all entries right and replace index 0
            else if max[0] < sum {
                max.rotate_right(1);
                max[0] = sum;
            }

            // Reset sum
            sum = 0;
            0
        });
    }

    // Information about how many valid lines there are
    let oks = lines
        .lines()
        .filter(|line| line.parse::<u32>().is_ok())
        .count();
    let errors = lines
        .lines()
        .filter(|line| line.parse::<u32>().is_err())
        .count();
    println!("Oks: {} | Errors: {}", oks, errors);
    println!("Biggest: {:?}", max[0] + max[1] + max[2]);
}
