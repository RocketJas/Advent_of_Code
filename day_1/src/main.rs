use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {

    let mut total_cals_vec: Vec<i32> = Vec::new();
    let mut total_cals: i32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                
                if ip == "" {
                    total_cals_vec.push(total_cals);
                    total_cals = 0;
                }
                else {
                    total_cals += i32::from_str(&ip).unwrap();
                }
            }
        }
    }
    total_cals_vec.sort_by(|a,b| b.cmp(a));
    let last_3_elements= &total_cals_vec[0..3];
    let last_3_total: i32 = last_3_elements.iter().sum();
    print!("{:#?}", last_3_total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}