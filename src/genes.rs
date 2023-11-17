use crate::{Key, Keyboard, Keycode};
use std::collections::HashMap;

pub struct Population {
    pub individuals: Vec<Individual>,
    pub average_fitness: usize,
    pub best_fitness: usize,
    pub generation: usize,
}

pub struct Individual {
    pub chromosomes: Keyboard,
    pub fitness: usize,
    pub lookup_table: HashMap<Keycode, usize>,
}

pub fn mate(a: Individual, b: Individual) -> Individual {
    todo!();
}

pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (((x1 - x2).powi(2) + (y1 - y2).powi(2)) as f32).sqrt() as f32 as f32 as f32 as f32
}

impl Individual {
    pub fn mutate(mut self) {}
    pub fn fitness(&mut self, homerow: Vec<Key>, wordset: Vec<String>) {
        let mut total_distance = 0;
        let mut fingers: Vec<Key> = homerow;
        let layer: usize = 0;
        for word in wordset {
            word.chars().for_each(|char| {
                let current_char: Vec<&Keycode> = self
                    .lookup_table
                    .keys()
                    .filter_map(|i| match &i {
                        Keycode::KC([x, y]) => {
                            if &char.to_string() == x || &char.to_string() == y {
                                Some(i)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .collect();
                let fart = current_char[0];
                let neededlayer = self.lookup_table.get(fart).unwrap();
                if &layer == neededlayer {
                    let mut shortest: (f32, usize) = (f32::MAX, 0);
                    self.chromosomes.layers[layer].iter().for_each(|key| {
                        if key.value.as_ref().unwrap() == current_char[0] {
                            (&fingers).into_iter().enumerate().for_each(|(i, finger)| {
                                let dist = distance(key.x, key.y, finger.x, finger.y);
                                if dist < shortest.0 {
                                    shortest = (dist, i);
                                }
                            });
                            fingers[shortest.1] = key.clone();
                            total_distance += shortest.0 as i32;
                        }
                    });
                }
            });
            self.fitness = total_distance as usize;
        }
    }

    pub fn init_table(&mut self) {
        let mut num = 0;
        let mut table: HashMap<Keycode, usize> = HashMap::new();
        for layer in &self.chromosomes.layers {
            for key in layer {
                table.insert(key.value.clone().unwrap(), num);
            }
            num += 1;
        }
        self.lookup_table = table;
    }
}

impl Population {
    fn prune(&mut self) {}
}
