use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    // total of all the modules
    let mut total: i32 = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("modules.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(mass) = line {
                // cast the String mass to an int and perform the calculation
                let val: i32 = (mass.parse::<i32>().unwrap() / 3) - 2;
                total += val; // increment the total value with newly calculated val
                println!("old value = {} ->  new value = {}", mass, val);
            }
        }
    }

    println!("The total is: {}", total);
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
