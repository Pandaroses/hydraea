use layout::format_json_kle;
use layout::Keyboard;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    return words;
}
// i have to change this function to make it work with layers aswell but idk hojw to do it
fn distance(keys: layout::Layer) -> HashMap<String, f32> {
    let mut fart = HashMap::new();
    for i in 0..keys.len() {
        for x in 0..keys.len() {
            let label: String = (keys[i].id.to_owned() + &keys[x].id.to_owned()).into();
            let dist: f32 =
                ((keys[x].x - keys[i].x).powi(2) + (keys[x].y - keys[i].y).powi(2)).sqrt();
            fart.insert(label, dist);
        }
    }

    return fart;
}

// how the fuck do layer keys must work
