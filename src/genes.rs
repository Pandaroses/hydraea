use futures::future::join_all;
use keyboard::{Keycode, Ngrams};
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;

use crate::keyboard;

pub struct Population {
    pub keyboards: Vec<keyboard::Keyboard>,
    pub best_fitness: usize,
    pub generation: usize,
    pub average_fitness: usize,
    pub ngrams: Ngrams,
    pub config: keyboard::FitnessConfig,
    pub bigram_set: Vec<(String, usize)>,
}

impl Population {
    pub async fn eval(&mut self) {
        let futures = self
            .keyboards
            .par_iter_mut()
            .map(|k| {
                let ngrams = self.ngrams.clone();
                let config = self.config.clone();
                let bigram_set = self.bigram_set.clone();

                async move {
                    k.calculate_fitness(ngrams, config, bigram_set).await;
                }
            })
            .collect::<Vec<_>>();

        join_all(futures).await;

        self.keyboards.sort_by(|a, b| a.fitness.cmp(&b.fitness));
        self.keyboards.reverse();
        self.average_fitness =
            self.keyboards.par_iter().map(|k| k.fitness).sum::<usize>() / self.keyboards.len();
        self.best_fitness = self.keyboards[0].fitness;
    }

    pub async fn next(&mut self) {
        let mut rander = rand::thread_rng();
        println!("evaluating generation {}", { self.generation });
        self.eval().await;
        println!("evaluation finished");
        println!("best fitness: {:?}", self.best_fitness);
        println!("average fitness: {:?}", self.average_fitness);
        println!("mutating...");
        let len = self.keyboards.len();
        self.keyboards[len - 1] = self.keyboards[0].clone();
        for i in 1..self.keyboards.len() {
            let rander_keyboard = self.keyboards[rander.gen_range(0..len)].clone();
            match rander.gen_range(0..3) {
                0 => self.keyboards[i].mutate().await,
                1 => self.keyboards[i].mate(rander_keyboard).await,
                2 => {
                    self.keyboards[i].mate(rander_keyboard).await;
                    self.keyboards[i].mutate().await;
                }
                3 => {
                    if i / len * 100 > 10 {
                        self.keyboards[i] = self.keyboards[i].clone()
                    } else {
                        self.keyboards[i] = self.keyboards[0].clone();
                        self.keyboards[i].mutate().await;
                    }
                }
                _ => {}
            }
        }
        println!("mutations finished, carrying on");
        self.generation += 1;
    }

    pub async fn output(&mut self) {
        println!("GENERATION FINISHED");
        println!("best fitness: {}", self.best_fitness);
        //(u8,u8,u8) : x,y,layer
        let mut keys: HashMap<(u8, u8, u8), Keycode> = HashMap::new();
        let mut max_dimensions = (0, 0);
        let layer = self.keyboards[0].keys[0].values.len();
        for i in self.keyboards[0].keys.iter() {
            for (x, e) in i.values.iter().enumerate() {
                if i.matrix.0 > max_dimensions.0 {
                    max_dimensions.0 = i.matrix.0
                }
                if i.matrix.1 > max_dimensions.1 {
                    max_dimensions.1 = i.matrix.1
                }
                keys.insert((i.matrix.0, i.matrix.1, x as u8), e.0.clone());
            }
        }
        for i in 0..layer {
            println!("layer {}", i);
            for y in 0..max_dimensions.1 {
                let mut top = "".to_string();
                let mut middle = "".to_string();
                let mut bottom = "".to_string();
                for x in 0..max_dimensions.0 {
                    top += "-----";
                    let meow = match keys.get(&(x, y, i as u8)) {
                        Some(e) => {
                            let meow = match e {
                                Keycode::KC([a, _]) => a.to_string(),
                                _ => "   ".to_string(),
                            };
                            &format!("{:?}    ", meow)[0..2]
                        }
                        None => "   ",
                    };
                    bottom += "-----";
                    middle += format!("-{}-", meow).as_str();
                }
                println!("{}", top);
                println!("{}", middle);
                println!("{}", bottom);
            }
        }
    }
}
