use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./hashtags.txt") {
        for line in lines {
            if let Ok(hashtag) = line {
                println!("{}", hashtag);
            }
        }
    }
    else {
        println!("CHICKEN");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
