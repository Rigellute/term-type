extern crate clap;
extern crate rand;
use clap::{App, Arg};
use rand::prelude::*;
use std::io::{stdin, stdout, Write};
use std::time::SystemTime;

const EASY_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

const MEDIUM_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const HARD_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789()[]{}*&^%$#@!~";

const ARG_MODE: &str = "mode";
const ARG_TEXT: &str = "text";

enum Mode {
    Easy,
    Medium,
    Hard,
}

fn generate_random_word(mode: &Mode) -> String {
    let charset = match mode {
        Mode::Easy => EASY_CHARSET,
        Mode::Medium => MEDIUM_CHARSET,
        Mode::Hard => HARD_CHARSET,
    };

    let mut rng = thread_rng();
    let rand_string: Option<String> = (0..rng.gen_range(1, 8))
        .map(|_| Some(*rng.choose(charset)? as char))
        .collect();

    match rand_string {
        Some(s) => s,
        None => "String not created".to_string(),
    }
}

fn generate_sentence(mode: &Mode) -> String {
    (0..10)
        .map(|_| generate_random_word(&mode))
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    // Setup basic cli with help options and arg parsing
    let matches = App::new("TermType")
        .version("1.0")
        .author("Alexander Keliris")
        .about("Practice typing within the terminal 🚀")
        .arg(
            Arg::with_name(ARG_MODE)
                .help("Specify the level of difficulty: <easy | medium | hard>")
                .takes_value(true)
                .short("m")
                .long(ARG_MODE),
        )
        .arg(
            Arg::with_name(ARG_TEXT)
                .help("Type with user provided text")
                .takes_value(true)
                .short("t")
                .long(ARG_TEXT),
        )
        .get_matches();

    let mode = match matches.value_of(ARG_MODE) {
        Some(m) => match m {
            "easy" => Mode::Easy,
            "medium" => Mode::Medium,
            "hard" => Mode::Hard,
            _ => Mode::Medium,
        },
        None => Mode::Medium,
    };

    loop {
        let rand_string = generate_sentence(&mode);
        let text = match matches.value_of(ARG_TEXT) {
            Some(t) => t.to_string(),
            None => rand_string,
        };

        println!("{}", text);
        stdout().flush().unwrap();

        let now = SystemTime::now();
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        match now.elapsed() {
            Ok(elapsed) => {
                let words_per_min = ((60.0 / elapsed.as_secs() as f64) * 10.0).floor();
                println!("{} wpm", words_per_min);
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {:?}", e);
            }
        }
    }
}
