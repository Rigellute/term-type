extern crate rand;
use rand::prelude::*;
use std::io::{stdin, stdout, Write};
use std::time::SystemTime;

fn main() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    loop {
        let mut rng = thread_rng();
        let rand_string: Option<String> = (0..30)
            .map(|_| Some(*rng.choose(CHARSET)? as char))
            .collect();

        match rand_string {
            Some(s) => {
                println!("{}", s);
            }
            None => {
                println!("String was not generated");
            }
        }

        stdout().flush().unwrap();

        let now = SystemTime::now();
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        match now.elapsed() {
            Ok(elapsed) => {
                // it prints '2'
                println!("{}", elapsed.as_secs());
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {:?}", e);
            }
        }
    }
}
