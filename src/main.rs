#![feature(async_closure)]
use std::env;
use std::fs;
pub mod genes;
pub mod keyboard;
use genes::*;
use keyboard::*;

const POPULATION_SIZE: usize = 100;
const GENERATIONS: usize = 1000;
const NGRAM_MULTIPLIER: f32 = 1.0;
const ROLL_MULTIPLIER: f32 = 1.5;

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
    content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].parse().ok()?))
            } else {
                None
            }
        })
        .collect()
}

fn create_sample_ngrams() -> Ngrams {
    vec![
        (vec![Keycode::KC(["a".to_string(), "A".to_string()])], 100),
        (
            vec![
                Keycode::KC(["t".to_string(), "T".to_string()]),
                Keycode::KC(["h".to_string(), "H".to_string()]),
            ],
            50,
        ),
    ]
}
