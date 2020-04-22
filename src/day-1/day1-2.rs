use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    // subtotal of each individual mass
    let mut subtotal: i64 = 0;

    // total of all the modules
    let mut total: i64 = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("modules.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(mass) = line {
                let mut val: i64 = mass.parse::<i64>().unwrap();
                loop {
                    val = (val / 3) - 2;
                    if val < 0 {
                        break;
                    }
                    subtotal += val;
                }
                total += subtotal; // increment the total value with newly calculated val
                println!("mass = {} ->  subtotal = {}", mass, subtotal);
                subtotal = 0;
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
