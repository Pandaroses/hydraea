use layout::{default_keycodes, Keyboard};
use layout::{format_json_kle, Layer};
use std::collections::HashMap;
//TODO main is not currently functional as intended, everything is hard coded
fn main() {
    let meow: Keyboard = format_json_kle("/home/gsh/proj/ml/layout/files/split.json".to_string());
    let mut words: Vec<String> = init();
}

fn init() -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    words.push("fart".to_string());
    let mut reader =
        csv::Reader::from_path("/home/gsh/proj/ml/testering/src/unigram_freq.csv").unwrap();
    for result in reader.records() {
        words.push(result.unwrap()[0].to_string());
    }
    let meow = default_keycodes();
    return words;
}
