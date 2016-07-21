extern crate rand;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use rand::thread_rng;


fn get_words(filename: &str) -> Result<Vec<String>, io::Error> {

    let mut f = try!(File::open(&filename));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    Ok(s.split('\n')
        .map(String::from)
        .filter(|s| !s.is_empty())
        .collect())
}

fn karaoke_print(line: &String, pause: &Duration) {
    for ch in line.chars() {
        print!("{}", ch);
        io::stdout().flush();
        sleep(*pause);
    }
}

fn main() {
    let word_vec = vec![get_words("list_1.txt").unwrap(),
                        get_words("list_2.txt").unwrap(),
                        get_words("list_3.txt").unwrap(),
                        get_words("list_4.txt").unwrap()];

    let line_sleep = Duration::new(1, 0);
    let word_sleep = Duration::new(0, 50000000);

    loop {
        println!("\nО-о-о-о! Моя оборона!");
        sleep(line_sleep);
        for _ in 0..4 {
            for word in 0..4 {
                let wordnum = thread_rng().gen_range(0, word_vec[word].len());
                karaoke_print(&word_vec[word][wordnum], &word_sleep);
                print!(" ");
            }
            println!("");
        }
    }
}
