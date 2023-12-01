use layout::{
    format_json_kle,
    genes::{mate, Individual, Population},
    Key, Keyboard, Keycode,
    Keycode::KC,
};
use std::collections::HashMap;
//TODO main is not currently functional as intended, everything is hard coded
fn main() {
    let meow: Keyboard = format_json_kle("/home/gsh/proj/ml/layout/files/corne.json".to_string());
    println!("{:?}", meow);
    let meow = Keyboard {
        layers: vec![vec![
            Key {
                x: 3.5,
                y: 20.5,
                id: "oX6qJ".to_string(),
                value: Some(KC(["e".to_string(), "e".to_string()])),
                fixed: false,
            },
            Key {
                x: 11.5,
                y: 20.5,
                id: "9WFQm".to_string(),
                value: Some(KC(["i".to_string(), "i".to_string()])),
                fixed: false,
            },
            Key {
                x: 2.5,
                y: 20.6,
                id: "c7plJ".to_string(),
                value: Some(KC(["w".to_string(), "w".to_string()])),
                fixed: false,
            },
            Key {
                x: 4.5,
                y: 20.6,
                id: "FasnO".to_string(),
                value: Some(KC(["r".to_string(), "r".to_string()])),
                fixed: false,
            },
            Key {
                x: 10.5,
                y: 20.6,
                id: "movjq".to_string(),
                value: Some(KC(["u".to_string(), "u".to_string()])),
                fixed: false,
            },
            Key {
                x: 12.5,
                y: 20.6,
                id: "DUOON".to_string(),
                value: Some(KC(["o".to_string(), "o".to_string()])),
                fixed: false,
            },
            Key {
                x: 5.5,
                y: 20.7,
                id: "2rUpS".to_string(),
                value: Some(KC(["t".to_string(), "t".to_string()])),
                fixed: false,
            },
            Key {
                x: 9.5,
                y: 20.7,
                id: "blKcd".to_string(),
                value: Some(KC(["y".to_string(), "y".to_string()])),
                fixed: false,
            },
            Key {
                x: 0.5,
                y: 20.8,
                id: "q_YWu".to_string(),
                value: Some(KC(["tab".to_string(), "tab".to_string()])),
                fixed: false,
            },
            Key {
                x: 1.5,
                y: 20.8,
                id: "nSatc".to_string(),
                value: Some(KC(["q".to_string(), "q".to_string()])),
                fixed: false,
            },
            Key {
                x: 13.5,
                y: 20.8,
                id: "9Ox9g".to_string(),
                value: Some(KC(["p".to_string(), "p".to_string()])),
                fixed: false,
            },
            Key {
                x: 14.5,
                y: 20.8,
                id: "g9rTB".to_string(),
                value: Some(KC(["bksp".to_string(), "bksp".to_string()])),
                fixed: false,
            },
            Key {
                x: 3.5,
                y: 21.5,
                id: "RL4EM".to_string(),
                value: Some(KC(["d".to_string(), "d".to_string()])),
                fixed: false,
            },
            Key {
                x: 11.5,
                y: 21.5,
                id: "vOtVv".to_string(),
                value: Some(KC(["k".to_string(), "k".to_string()])),
                fixed: false,
            },
            Key {
                x: 2.5,
                y: 21.6,
                id: "DuWUg".to_string(),
                value: Some(KC(["s".to_string(), "s".to_string()])),
                fixed: false,
            },
            Key {
                x: 4.5,
                y: 21.6,
                id: "3zJuO".to_string(),
                value: Some(KC(["f".to_string(), "f".to_string()])),
                fixed: false,
            },
            Key {
                x: 10.5,
                y: 21.6,
                id: "4OhpV".to_string(),
                value: Some(KC(["j".to_string(), "j".to_string()])),
                fixed: false,
            },
            Key {
                x: 12.5,
                y: 21.6,
                id: "S_CY4".to_string(),
                value: Some(KC(["l".to_string(), "l".to_string()])),
                fixed: false,
            },
            Key {
                x: 5.5,
                y: 21.7,
                id: "sm6Bn".to_string(),
                value: Some(KC(["g".to_string(), "g".to_string()])),
                fixed: false,
            },
            Key {
                x: 9.5,
                y: 21.7,
                id: "_1iIu".to_string(),
                value: Some(KC(["h".to_string(), "h".to_string()])),
                fixed: false,
            },
            Key {
                x: 0.5,
                y: 21.8,
                id: "BO9hf".to_string(),
                value: Some(KC(["esc".to_string(), "esc".to_string()])),
                fixed: false,
            },
            Key {
                x: 1.5,
                y: 21.8,
                id: "VAnL2".to_string(),
                value: Some(KC(["a".to_string(), "a".to_string()])),
                fixed: false,
            },
            Key {
                x: 13.5,
                y: 21.8,
                id: "4SdoN".to_string(),
                value: Some(KC([":\n;".to_string(), ":\n;".to_string()])),
                fixed: false,
            },
            Key {
                x: 14.5,
                y: 21.8,
                id: "ZjpZJ".to_string(),
                value: Some(KC(["'".to_string(), "'".to_string()])),
                fixed: false,
            },
            Key {
                x: 3.5,
                y: 22.5,
                id: "thwDd".to_string(),
                value: Some(KC(["c".to_string(), "c".to_string()])),
                fixed: false,
            },
            Key {
                x: 11.5,
                y: 22.5,
                id: "RQ0Ze".to_string(),
                value: Some(KC(["<\n,".to_string(), "<\n,".to_string()])),
                fixed: false,
            },
            Key {
                x: 2.5,
                y: 22.6,
                id: "zGvqH".to_string(),
                value: Some(KC(["x".to_string(), "x".to_string()])),
                fixed: false,
            },
            Key {
                x: 4.5,
                y: 22.6,
                id: "5UprN".to_string(),
                value: Some(KC(["v".to_string(), "v".to_string()])),
                fixed: false,
            },
            Key {
                x: 10.5,
                y: 22.6,
                id: "LGkAi".to_string(),
                value: Some(KC(["m".to_string(), "m".to_string()])),
                fixed: false,
            },
            Key {
                x: 12.5,
                y: 22.6,
                id: "-om-c".to_string(),
                value: Some(KC([">\n.".to_string(), ">\n.".to_string()])),
                fixed: false,
            },
            Key {
                x: 5.5,
                y: 22.7,
                id: "3KUeM".to_string(),
                value: Some(KC(["b".to_string(), "b".to_string()])),
                fixed: false,
            },
            Key {
                x: 9.5,
                y: 22.7,
                id: "IxXw1".to_string(),
                value: Some(KC(["n".to_string(), "n".to_string()])),
                fixed: false,
            },
            Key {
                x: 0.5,
                y: 22.8,
                id: "2Qbm8".to_string(),
                value: Some(KC(["ctrl".to_string(), "ctrl".to_string()])),
                fixed: false,
            },
            Key {
                x: 1.5,
                y: 22.8,
                id: "xesdj".to_string(),
                value: Some(KC(["z".to_string(), "z".to_string()])),
                fixed: false,
            },
            Key {
                x: 13.5,
                y: 22.8,
                id: "KdTgV".to_string(),
                value: Some(KC(["?\n/".to_string(), "?\n/".to_string()])),
                fixed: false,
            },
            Key {
                x: 14.5,
                y: 22.8,
                id: "UykJQ".to_string(),
                value: Some(KC(["enter".to_string(), "enter".to_string()])),
                fixed: false,
            },
            Key {
                x: 4.0,
                y: 23.6,
                id: "GTwKs".to_string(),
                value: Some(KC(["super".to_string(), "super".to_string()])),
                fixed: false,
            },
            Key {
                x: 11.0,
                y: 23.6,
                id: "N2XNS".to_string(),
                value: Some(KC(["alt".to_string(), "alt".to_string()])),
                fixed: false,
            },
            Key {
                x: 5.0,
                y: 24.0,
                id: "FwI9g".to_string(),
                value: Some(KC(["lower".to_string(), "lower".to_string()])),
                fixed: false,
            },
            Key {
                x: 6.5,
                y: 24.0,
                id: "3EEk-".to_string(),
                value: Some(KC(["space".to_string(), "space".to_string()])),
                fixed: false,
            },
            Key {
                x: 9.0,
                y: 24.0,
                id: "unsy2".to_string(),
                value: Some(KC(["shift".to_string(), "shift".to_string()])),
                fixed: false,
            },
            Key {
                x: 6.7283864,
                y: 14.586898,
                id: "T7tRv".to_string(),
                value: Some(KC(["raise".to_string(), "raise".to_string()])),
                fixed: false,
            },
        ]],
    };

    let words: Vec<String> = init();
    // let words: Vec<String> = vec![
    //     "the".to_string(),
    //     "quick".to_string(),
    //     "brown".to_string(),
    //     "fox".to_string(),
    //     "jumps".to_string(),
    //     "over".to_string(),
    //     "the".to_string(),
    //     "lazy".to_string(),
    //     "dog".to_string(),
    // ];
    let mut test = Individual {
        chromosomes: meow.clone(),
        fitness: 0,
        lookup_table: HashMap::new(),
    };
    test.init_table();
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
    let mut pop = Population {
        individuals: vec![test; 500],
        average_fitness: 0,
        best_fitness: 0,
        generation: 0,
        homerow,
        wordset: words[0..500].iter().map(|i| i.to_string()).collect(),
        // wordset: words,
    };
    for _ in 0..1000 {
        pop.next();
    }
    let winner = pop.individuals[1].clone();
    let easier: Vec<(f32, f32, Keycode)> = winner.chromosomes.layers[0]
        .iter()
        .map(|s| (s.x, s.y, s.value.clone().unwrap()))
        .collect();
    println!("{:?}", easier);
}

fn init() -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    words.push("fart".to_string());
    let mut reader =
        csv::Reader::from_path("/home/gsh/proj/ml/layout/files/unigram_freq.csv").unwrap();
    for result in reader.records() {
        words.push(result.unwrap()[0].to_string());
    }
    return words;
}
