use crate::{Key, Keyboard, Keycode};
use rand::{random, seq::SliceRandom, Rng};
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

pub fn new_mate(a: Individual, b: Individual) -> Individual {
    // replace this with actual all_keycodes because lookup table can be potentially issued
    let all_keycodes = a.lookup_table;
    let mut missing_keycodes: Vec<Option<Keycode>> = Vec::new();
    let mut res: Keyboard;

    for layer in 0..a.chromosomes.layers.len() {
        let middle = a.chromosomes.layers[layer].len() / 2;
        let res_layer = a.chromosomes.clone().layers[layer].splice(
            ..middle,
            b.chromosomes.layers[layer][middle..].iter().cloned(),
        );
    }
    todo!();
}

pub fn mate(a: Individual, b: Individual) -> Individual {
    let all_keycodes = a.lookup_table;
    let mut missing_keycodes: Vec<Option<Keycode>> = Vec::new();
    let mut res_keyboard = a.chromosomes.clone();
    for layer in 0..a.chromosomes.layers.len() {
        for key in (a.chromosomes.layers[layer].len() / 2)..b.chromosomes.layers[layer].len() {
            res_keyboard.layers[layer][key].value = None;
        }
        for key in (a.chromosomes.layers[layer].len() / 2)..b.chromosomes.layers[layer].len() {
            if res_keyboard.layers[layer][key].value == None
                && !(res_keyboard.layers[layer]
                    .iter()
                    .filter(|i| i.value == b.chromosomes.layers[layer][key].value)
                    .count()
                    > 1)
            {
                res_keyboard.layers[layer][key].value =
                    b.chromosomes.layers[layer][key].value.clone();
            }
            missing_keycodes.push(b.chromosomes.layers[layer][key].value.clone());
        }
        for key in 0..res_keyboard.layers[layer].len() {
            res_keyboard.layers[layer].iter().for_each(|m| {
                if missing_keycodes.contains(&m.value) {
                    let remove = missing_keycodes.iter().position(|f| f == &m.value).unwrap();
                    missing_keycodes.remove(remove);
                }
            });
            let mut rand = rand::thread_rng();
            if res_keyboard.layers[layer][key].value == None {
                let pos = rand.gen_range(0..missing_keycodes.len());
                res_keyboard.layers[layer][key].value = missing_keycodes[pos].clone();
                missing_keycodes.remove(pos);
            } else if (res_keyboard.layers[layer]
                .clone()
                .into_iter()
                .filter(|i| i.value == res_keyboard.layers[layer][key].value.clone())
                .count())
                > 1
                && missing_keycodes.len() >= 1
            {
                let meow: Vec<Key> = res_keyboard.layers[layer]
                    .clone()
                    .into_iter()
                    .filter_map(|i| {
                        if i.value == res_keyboard.layers[layer][key].value.clone() {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();

                let mut positions: Vec<usize> = meow
                    .iter()
                    .filter_map(|i| res_keyboard.layers[layer].iter().position(|j| j.id == i.id))
                    .collect();
                let picked = rand.gen_range(0..positions.len());
                positions.remove(picked);
                positions.iter().for_each(|i| {
                    let removed_pos = rand.gen_range(0..missing_keycodes.len());
                    res_keyboard.layers[layer][*i].value = missing_keycodes[removed_pos].clone();
                    missing_keycodes.remove(removed_pos);
                })
            }
        }
    }
    let mut res = Individual {
        chromosomes: res_keyboard,
        fitness: 0,
        lookup_table: HashMap::new(),
    };
    res.init_table();
    res
}

pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (((x1 - x2).powi(2) + (y1 - y2).powi(2)) as f32).sqrt()
}

impl Individual {
    pub fn mutate(&mut self) {
        // this switches the keys around
        let layers = self.chromosomes.layers.len();
        let mut random = rand::thread_rng();
        let mutation_amount = random.gen_range(0..10);
        for _i in 0..mutation_amount {
            let key1_layer = random.gen_range(0..layers);
            let key2_layer = random.gen_range(0..layers);
            let key1_pos = random.gen_range(0..self.chromosomes.layers[key1_layer].len());
            let key2_pos = random.gen_range(0..self.chromosomes.layers[key2_layer].len());
            let temp_key1 = self.chromosomes.layers[key1_layer][key1_pos].clone();
            self.chromosomes.layers[key1_layer][key1_pos].value =
                self.chromosomes.layers[key2_layer][key2_pos].clone().value;
            self.chromosomes.layers[key2_layer][key2_pos].value = temp_key1.value;
            self.init_table();
        }
    }

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

impl Population {
    fn prune(&mut self) {}
}
