extern crate rand;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;


fn get_words(filename: &str) -> Vec<String> {
    let mut f = match File::open(&filename) {
        Err(why) => panic!("couldn't open file {}: {}", filename, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read file {}: {}", filename, why),
        Ok(size) => println!("read {} bytes", size),
    }
    return s.split('\n').map(String::from).filter(|s| !s.is_empty()).collect();
}

macro_rules! word_vec {
    ($x: expr) => (vec_$x)
}

fn main() {
    let word_vec = vec![get_words("list_1.txt"),
                        get_words("list_2.txt"),
                        get_words("list_3.txt"),
                        get_words("list_4.txt")];
    let line_sleep = Duration::new(1, 0);
    let word_sleep = Duration::new(0, 50000000);
    loop {
        println!("\nО-о-о-о! Моя оборона!");
        sleep(line_sleep);
        for line in 0..4 {
            for word in 0..4 {
                let word = &word_vec[word][rand::thread_rng().gen_range(0, word_vec[word].len())];
                let chars = word.chars();
                for ch in chars {
                    print!("{}", ch);
                    io::stdout().flush();
                    sleep(word_sleep);
                }
                print!(" ");
            }
            println!("");
        }
    }
}
