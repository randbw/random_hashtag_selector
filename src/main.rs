use rand::Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::iter::FromIterator;

fn main() {
    let mut read_hashtags = HashSet::new();
    let mut rng = rand::thread_rng();
    let mut hashtags_to_use = HashSet::new();

    if let Ok(lines) = read_lines("./hashtags.txt") {
        for line in lines {
            if let Ok(hashtag) = line {
                read_hashtags.insert(hashtag.to_string());
            }
        }

        let hashtags_veccaz = Vec::from_iter(read_hashtags.iter().cloned());

        while hashtags_to_use.len() < 30 {
            let index = rng.gen_range(0, read_hashtags.len());
            hashtags_to_use.insert(hashtags_veccaz[index].to_string());
            println!("hash at ind {} is {}", index, hashtags_veccaz[index].to_string());
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
