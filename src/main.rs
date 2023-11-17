use layout::{default_keycodes, format_json_kle, genes::Individual, Key, Keyboard, Keycode, Layer};
use std::collections::HashMap;
//TODO main is not currently functional as intended, everything is hard coded
fn main() {
    let meow: Keyboard = format_json_kle("/home/gsh/proj/ml/layout/files/split.json".to_string());
    let mut words: Vec<String> = init();
    let mut test = Individual {
        chromosomes: meow,
        fitness: 0,
        lookup_table: HashMap::new(),
    };
    test.init_table();
    let home = vec![
        Keycode::KC(["a".to_string(), "a".to_string()]),
        Keycode::KC(["b".to_string(), "b".to_string()]),
        Keycode::KC(["c".to_string(), "c".to_string()]),
    ];
    let homerow: Vec<Key> = test.chromosomes.layers[0]
        .clone()
        .into_iter()
        .filter_map(|i| {
            if home.contains(&i.value.clone().unwrap()) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .to_owned();
    test.fitness(homerow, words);
    println!("{:?}", test.fitness);
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
