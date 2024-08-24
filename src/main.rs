#![feature(async_closure)]
use std::env;
use std::fs;
pub mod genes;
pub mod keyboard;
use genes::*;
use keyboard::*;

const POPULATION_SIZE: usize = 100;
const GENERATIONS: usize = 1000;
const NGRAM_MULTIPLIER: f32 = 10.0;
const ROLL_MULTIPLIER: f32 = 50.0;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <bigrams_file> <keyboard_json>", args[0]);
        std::process::exit(1);
    }

    let bigrams_file = &args[1];
    let keyboard_json = &args[2];

    let bigrams_content = fs::read_to_string(bigrams_file)?;
    let bigram_set: Vec<(String, usize)> = parse_bigrams(&bigrams_content);

    let keyboard_content = fs::read_to_string(keyboard_json)?;
    let initial_keyboard = Keyboard::from_json(&keyboard_content)?;

    let ngrams: Ngrams = create_sample_ngrams();

    let fitness_config = FitnessConfig {
        ngram_multiplier: NGRAM_MULTIPLIER,
        roll_type: Roll::Inwards, // Adjust as needed
        roll_multiplier: ROLL_MULTIPLIER,
    };

    let mut population = Population {
        keyboards: vec![initial_keyboard.clone(); POPULATION_SIZE],
        best_fitness: 0,
        generation: 0,
        average_fitness: 0,
        ngrams: ngrams.clone(),
        config: fitness_config,
        bigram_set: bigram_set.clone(),
    };

    for _ in 0..GENERATIONS {
        population.next().await;
    }

    population.output().await;
    Ok(())
}

fn parse_bigrams(content: &str) -> Vec<(String, usize)> {
    let meow: Vec<(String, usize)> = serde_json::from_str(content).unwrap();
    meow
}

// waiting for primary research
fn create_sample_ngrams() -> Ngrams {
    vec![
        (
            vec![
                Keycode::KC(["1".to_string(), "1".to_string()]),
                Keycode::KC(["leftalt".to_string(), "LEFTALT".to_string()]),
            ],
            12612,
        ),
        (
            vec![
                Keycode::KC(["e".to_string(), "E".to_string()]),
                Keycode::KC(["space".to_string(), "SPACE".to_string()]),
            ],
            10165,
        ),
        (
            vec![
                Keycode::KC(["2".to_string(), "2".to_string()]),
                Keycode::KC(["leftalt".to_string(), "LEFTALT".to_string()]),
            ],
            9118,
        ),
        (
            vec![
                Keycode::KC(["t".to_string(), "T".to_string()]),
                Keycode::KC(["space".to_string(), "SPACE".to_string()]),
            ],
            8368,
        ),
        (
            vec![
                Keycode::KC(["space".to_string(), "SPACE".to_string()]),
                Keycode::KC(["a".to_string(), "A".to_string()]),
            ],
            8300,
        ),
        (
            vec![
                Keycode::KC(["w".to_string(), "W".to_string()]),
                Keycode::KC(["a".to_string(), "A".to_string()]),
            ],
            8207,
        ),
        (
            vec![
                Keycode::KC(["s".to_string(), "S".to_string()]),
                Keycode::KC(["space".to_string(), "SPACE".to_string()]),
            ],
            7343,
        ),
        (
            vec![
                Keycode::KC(["d".to_string(), "D".to_string()]),
                Keycode::KC(["space".to_string(), "SPACE".to_string()]),
            ],
            7222,
        ),
        (
            vec![
                Keycode::KC(["2".to_string(), "2".to_string()]),
                Keycode::KC(["1".to_string(), "1".to_string()]),
                Keycode::KC(["leftalt".to_string(), "LEFTALT".to_string()]),
            ],
            3748,
        ),
        (
            vec![
                Keycode::KC(["space".to_string(), "SPACE".to_string()]),
                Keycode::KC(["t".to_string(), "T".to_string()]),
                Keycode::KC(["h".to_string(), "H".to_string()]),
            ],
            3440,
        ),
        (
            vec![
                Keycode::KC(["enter".to_string(), "ENTER".to_string()]),
                Keycode::KC(["1".to_string(), "1".to_string()]),
                Keycode::KC(["leftalt".to_string(), "LEFTALT".to_string()]),
            ],
            3398,
        ),
        (
            vec![
                Keycode::KC(["1".to_string(), "1".to_string()]),
                Keycode::KC(["leftalt".to_string(), "LEFTALT".to_string()]),
                Keycode::KC(["leftctrl".to_string(), "LEFTCTRL".to_string()]),
            ],
            3181,
        ),
        (
            vec![
                Keycode::KC(["t".to_string(), "T".to_string()]),
                Keycode::KC(["h".to_string(), "H".to_string()]),
                Keycode::KC(["e".to_string(), "E".to_string()]),
            ],
            2774,
        ),
        (
            vec![
                Keycode::KC(["semicolon".to_string(), "SEMICOLON".to_string()]),
                Keycode::KC(["w".to_string(), "W".to_string()]),
                Keycode::KC(["enter".to_string(), "ENTER".to_string()]),
            ],
            2751,
        ),
        (
            vec![
                Keycode::KC(["leftctrl".to_string(), "LEFTCTRL".to_string()]),
                Keycode::KC(["a".to_string(), "A".to_string()]),
                Keycode::KC(["backspace".to_string(), "BACKSPACE".to_string()]),
            ],
            2735,
        ),
        (
            vec![
                Keycode::KC(["esc".to_string(), "ESC".to_string()]),
                Keycode::KC(["reserved".to_string(), "RESERVED".to_string()]),
                Keycode::KC(["semicolon".to_string(), "SEMICOLON".to_string()]),
                Keycode::KC(["w".to_string(), "W".to_string()]),
            ],
            2466,
        ),
        (
            vec![
                Keycode::KC(["reserved".to_string(), "RESERVED".to_string()]),
                Keycode::KC(["semicolon".to_string(), "SEMICOLON".to_string()]),
                Keycode::KC(["w".to_string(), "W".to_string()]),
                Keycode::KC(["enter".to_string(), "ENTER".to_string()]),
            ],
            2407,
        ),
        (
            vec![
                Keycode::KC(["space".to_string(), "SPACE".to_string()]),
                Keycode::KC(["t".to_string(), "T".to_string()]),
                Keycode::KC(["h".to_string(), "H".to_string()]),
                Keycode::KC(["e".to_string(), "E".to_string()]),
            ],
            1826,
        ),
        (
            vec![
                Keycode::KC(["t".to_string(), "T".to_string()]),
                Keycode::KC(["h".to_string(), "H".to_string()]),
                Keycode::KC(["e".to_string(), "E".to_string()]),
                Keycode::KC(["space".to_string(), "SPACE".to_string()]),
            ],
            1679,
        ),
        (
            vec![
                Keycode::KC(["1".to_string(), "1".to_string()]),
                Keycode::KC(["leftalt".to_string(), "LEFTALT".to_string()]),
                Keycode::KC(["leftctrl".to_string(), "LEFTCTRL".to_string()]),
                Keycode::KC(["t".to_string(), "T".to_string()]),
            ],
            1592,
        ),
    ]
}
