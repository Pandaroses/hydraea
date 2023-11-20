use layout::{
    format_json_kle,
    genes::{mate, Individual},
    Key, Keyboard, Keycode,
};
use std::collections::HashMap;
//TODO main is not currently functional as intended, everything is hard coded
fn main() {
    let meow: Keyboard = format_json_kle("/home/gsh/proj/hydraea/files/corne.json".to_string());
    let words: Vec<String> = init();
    let mut test = Individual {
        chromosomes: meow.clone(),
        fitness: 0,
        lookup_table: HashMap::new(),
    };
    let mut test2 = Individual {
        chromosomes: meow.clone(),
        fitness: 0,
        lookup_table: HashMap::new(),
    };
    test.init_table();
    test2.init_table();
    let home = vec![
        Keycode::KC(["a".to_string(), "a".to_string()]),
        Keycode::KC(["s".to_string(), "s".to_string()]),
        Keycode::KC(["d".to_string(), "d".to_string()]),
        Keycode::KC(["f".to_string(), "f".to_string()]),
        Keycode::KC(["j".to_string(), "j".to_string()]),
        Keycode::KC(["k".to_string(), "k".to_string()]),
        Keycode::KC(["l".to_string(), "l".to_string()]),
        Keycode::KC([";".to_string(), ":".to_string()]),
    ];
    let homerow: Vec<String> = test.chromosomes.layers[0]
        .clone()
        .into_iter()
        .filter_map(|i| {
            if home.contains(&i.value.clone().unwrap()) {
                Some(i.id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .to_owned();
    test.fitness(homerow.clone(), words.clone());
    println!("{:?}", test.fitness);
    test2.mutate();
    test2.init_table();
    let mut test3 = mate(test, test2);
    test3.fitness(homerow, words);
    println!("{:?}", test3.fitness);
}

fn init() -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    words.push("fart".to_string());
    let mut reader =
        csv::Reader::from_path("/home/gsh/proj/hydraea/files/unigram_freq.csv").unwrap();
    for result in reader.records() {
        words.push(result.unwrap()[0].to_string());
    }
    return words;
}
