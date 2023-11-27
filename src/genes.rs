//! simple keyboard layout genetic algorithms
//!
//! # Examples
//! TODO

//TODO in genetic algorithms
// refactor code
// document code
// finish code
// make fitness better
// find cool graphing
// potentially have a really looking tui before writing a frontend in web https://docs.rs/tui/latest/tui/index.html
use crate::{Key, Keyboard, Keycode};
use rand::Rng;
use std::collections::HashMap;

pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (((x1 - x2).powi(2) + (y1 - y2).powi(2)) as f32).sqrt()
}

pub struct Individual {
    pub chromosomes: Keyboard,
    pub fitness: usize,
    pub lookup_table: HashMap<Keycode, usize>,
}

impl Individual {
    /// potentially useless, or maybe not
    pub fn render(self) {
        todo!()
    }
    /// switches around between 0 and 10 keys around on random layers and random position, needs error/validity handling
    pub fn mutate(&mut self) {
        let layers = self.chromosomes.layers.len();
        let mut random = rand::thread_rng();
        let mutation_amount = random.gen_range(0..10);
        (0..mutation_amount).for_each(|_| {
            let key1_layer = random.gen_range(0..layers);
            let key2_layer = random.gen_range(0..layers);
            let key1_pos = random.gen_range(0..self.chromosomes.layers[key1_layer].len());
            let key2_pos = random.gen_range(0..self.chromosomes.layers[key2_layer].len());
            let temp_key1 = self.chromosomes.layers[key1_layer][key1_pos].clone();
            self.chromosomes.layers[key1_layer][key1_pos].value =
                self.chromosomes.layers[key2_layer][key2_pos].clone().value;
            self.chromosomes.layers[key2_layer][key2_pos].value = temp_key1.value;
            self.init_table();
        });
    }
    /// calculates the fitness of this Individual, is not fully-fledged as it should have many more parameters, but those are in the TODO
    pub fn fitness(&mut self, homerow: Vec<String>, wordset: Vec<String>) {
        //this is the function that calculates how far fingers have to travel to write the wordset
        let mut total_distance = 0;
        let mut fingers: Vec<Key> = self.chromosomes.layers[0]
            .clone()
            .into_iter()
            .filter_map(|i| {
                if homerow.contains(&i.id) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        let layer: usize = 0;
        for word in wordset {
            let space = self.shortest_distance(
                layer,
                vec![&Keycode::KC(["space".to_string(), "space".to_string()])],
                fingers.clone(),
            );
            total_distance += space.1;
            fingers[space.2] = space.0;
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
                    // for some reason this returns the exact same values even if self is mutated
                    let meow = self.shortest_distance(layer, current_char, fingers.clone());
                    fingers[meow.2] = meow.0;
                    total_distance += meow.1;
                } else {
                    todo!()
                    // implement looking for layer switching
                }
            });
            self.fitness = total_distance as usize;
        }
    }
    /// finds the shortest distance from a list of keys to a key
    pub fn shortest_distance(
        &self,
        layer: usize,
        current_char: Vec<&Keycode>,
        fingers: Vec<Key>,
    ) -> (Key, i32, usize) {
        let mut shortest: (f32, Key, usize) = (f32::MAX, Key::default(), 0);
        for key in self.chromosomes.layers[layer].iter() {
            if key.value.as_ref().unwrap() == current_char[0] {
                for (i, finger) in (&fingers).into_iter().enumerate() {
                    let dist = distance(key.x, key.y, finger.x, finger.y);
                    if dist < shortest.0 {
                        shortest = (dist, key.clone(), i);
                    }
                }
            }
        }
        return (shortest.1, shortest.0 as i32, shortest.2);
    }
    ///initializes table of this Individual, should only be run by the Individual inside other functions
    pub fn init_table(&mut self) {
        let mut num = 0;
        let mut table: HashMap<Keycode, usize> = HashMap::new();
        self.chromosomes.layers.iter().for_each(|layer| {
            for key in layer {
                table.insert(key.value.clone().unwrap(), num);
            }
            num += 1;
        });
        self.lookup_table = table;
    }
}

pub fn mate(a: Individual, b: Individual) -> Individual {
    // replace this with actual all_keycodes because lookup table can be potentially issued
    let all_keycodes = a.lookup_table;
    let mut missing_keycodes: Vec<Option<Keycode>> = Vec::new();
    let mut res: Keyboard = Keyboard { layers: vec![] };

    for layer in 0..a.chromosomes.layers.len() {
        let middle: usize = a.chromosomes.layers[layer].len() / 2;
        let res_layer: &mut Vec<Key> = &mut a.chromosomes.clone().layers[layer];
        res_layer.splice(
            middle..,
            b.chromosomes.layers[layer][middle..].iter().cloned(),
        );

        for key in middle..res_layer.len() {
            let key_value = res_layer[key].value.clone();
            let duplicates: Vec<Key> = res_layer
                .iter()
                .filter_map(|i| {
                    if i.value == key_value {
                        Some(i.to_owned())
                    } else {
                        None
                    }
                })
                .collect();

            if duplicates.len() > 1 {
                res_layer[key].value = None;
            }
        }

        for (keycode, keycode_layer) in &all_keycodes {
            if *keycode_layer == layer
                && !res_layer
                    .iter()
                    .any(|key| key.value.as_ref() == Some(keycode))
            {
                missing_keycodes.push(Some(keycode.clone()));
            }
        }

        println!("{:?}", missing_keycodes);

        for key in middle..res_layer.len() {
            let key_value = res_layer[key].value.clone();
            if key_value == None {
                let mut rand = rand::thread_rng();
                let pos = rand.gen_range(0..missing_keycodes.len());
                res_layer[key].value = missing_keycodes[pos].clone();
                missing_keycodes.remove(pos);
            }
        }
        res.layers.push(res_layer.to_owned());
    }
    let mut meow = Individual {
        chromosomes: res,
        fitness: 0,
        lookup_table: b.lookup_table.clone(),
    };
    meow.init_table();
    meow
}

pub struct Population {
    pub individuals: Vec<Individual>,
    pub average_fitness: usize,
    pub best_fitness: usize,
    pub generation: usize,
}

impl Population {
    /// given all remaining individuals, pick out random individuals that get to survive the pruning,will contain configurable settings such as how much of the population should remain and others
    pub fn lottery(self) -> Vec<Individual> {
        todo!()
    }
    /// sorts all individuals by fitness, then sets average fitness and best fitness of current population, returns the sorted list
    pub fn eval(&mut self) -> Vec<Individual> {
        todo!()
    }
    /// initialize a population, should only be run once, will contain configs such as size of population, whether or not each individual should be randomized or based on a predeterimened keyboard layout
    pub fn init(&mut self) {}
    /// wrapper for all functions needed to ascend a generation, includes logging and potential rendering
    pub fn next(&mut self) {}
}
