use rand::Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::env;
use std::iter::FromIterator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut read_hashtags = HashSet::new();
    let mut rng = rand::thread_rng();
    let mut hashtags_to_use = HashSet::new();
    let mut hashtags_to_print = String::new();

    let filename = &args[1];

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(hashtag) = line {
                read_hashtags.insert(hashtag.to_string());
            }
        }

        let hashtag_vec = Vec::from_iter(read_hashtags.iter().cloned());

        while hashtags_to_use.len() < 30 {
            let index = rng.gen_range(0, read_hashtags.len());
            hashtags_to_use.insert(hashtag_vec[index].to_string());
        }

        for hashtag in hashtags_to_use {
            hashtags_to_print.push_str(" #");
            hashtags_to_print.push_str(&hashtag);
        }

        println!("Final hashtag list is:");
        println!("{}", hashtags_to_print.trim_start());

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
