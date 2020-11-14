use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut hashtags = HashSet::new();

    if let Ok(lines) = read_lines("./hashtags.txt") {
        for line in lines {
            if let Ok(hashtag) = line {
                hashtags.insert(hashtag.to_string());
            }
        }

        for hashtag in hashtags {
            println!("hashytag is {}", hashtag);
        }

    } else {
        println!("Failed to read ./hashtags.txt");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
