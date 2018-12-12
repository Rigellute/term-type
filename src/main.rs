extern crate rand;
use rand::prelude::*;
use std::io::{stdin, stdout, Write};
use std::time::SystemTime;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789()[]{}*&^%$#@!~";

fn generate_random_word() -> String {
    let mut rng = thread_rng();
    let rand_string: Option<String> = (0..rng.gen_range(1, 8))
        .map(|_| Some(*rng.choose(CHARSET)? as char))
        .collect();

    match rand_string {
        Some(s) => s,
        None => "String not created".to_string(),
    }
}

fn generate_sentence() -> String {
    (0..10)
        .map(|_| generate_random_word())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    loop {
        let rand_string = generate_sentence();

        println!("{}", rand_string);
        stdout().flush().unwrap();

        let now = SystemTime::now();
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        match now.elapsed() {
            Ok(elapsed) => {
                let words_per_min = (60.0 / elapsed.as_secs() as f64) * 10.0;
                println!("{} wpm", words_per_min);
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {:?}", e);
            }
        }
    }
}
