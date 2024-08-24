//! Pandaroses Keyboard Layout Generator (Hydraea)
//! uses a genetic algorithm with many options

use rand::{random, Rng};

pub struct Keyboard {
    keys: Vec<Key>,
    /// specifies default finger positions on the matrix
    default_positions: Vec<(u8, u8)>,
    fitness: usize,
}

#[derive(Eq, PartialEq)]
enum Roll {
    Inwards,
    Outwards,
}

pub struct FitnessConfig {
    ngram_multiplier: f32,
    roll_type: Roll,
    // non_optimal_column_multiplier: f32,
    roll_multiplier: f32,
}

impl Keyboard {
    /// calculates fitness,
    pub async fn calculate_fitness(mut self, ngrams: Ngrams, config: FitnessConfig) {
        let mut ngram_fitness = 0.0;
        // ngram+roll fitness
        for n in ngrams.iter() {
            let mut i = 0;
            let mut keys: Vec<Key> = Vec::new();
            while i < n.0.len() {
                match self.find_key(n.0[i].clone()).await {
                    Some(x) => {
                        keys.push(x);
                        if keys.len() >= 2 {
                            let y_check = keys[i - 1].matrix.1 == keys[i].matrix.1;
                            let same_hand = keys[i - 1].hand == keys[i - 1].hand;
                            let x_diff = keys[i - 1].matrix.0.abs_diff(keys[i].matrix.0) == 1;
                            if x_diff && same_hand && y_check {
                                continue;
                            } else {
                                i = n.0.len() + 1;
                            }
                        }
                    }
                    None => {
                        i = n.0.len() + 1;
                    }
                };
            }
            if keys.len() == n.0.len() {
                let roll = match (keys[0].matrix.0 as i8 - keys[1].matrix.0 as i8) {
                    -1 if keys[0].hand == Hand::Left => Roll::Inwards,
                    -1 if keys[0].hand == Hand::Right => Roll::Outwards,
                    1 if keys[0].hand == Hand::Left => Roll::Outwards,
                    1 if keys[0].hand == Hand::Right => Roll::Inwards,

                    _ => todo!(),
                };
                let roll_mul = if config.roll_type == roll {
                    config.roll_multiplier
                        + if self
                            .default_positions
                            .contains(&(keys[0].matrix.0, keys[0].matrix.1))
                        {
                            config.roll_multiplier
                        } else {
                            0.0
                        }
                } else {
                    1.0
                };
                ngram_fitness += n.1 as f32 * n.0.len() as f32 * roll_mul;
            }
        }
    }

    pub async fn find_key(&self, kc: Keycode) -> Option<Key> {
        for key in &self.keys {
            for (keycode, _) in &key.values {
                if *keycode == kc {
                    return Some(key.clone());
                }
            }
        }
        None
    }
    pub async fn mutate(mut self) {
        let mut rander = rand::thread_rng();
        let amount = rander.gen_range(1..10);
        let mut key_1 = (0, 0);
        let mut key_2 = (0, 0);
        for i in 0..amount {
            let mut x = true;
            while x == true {
                let layer_1 = rander.gen_range(0..self.keys[0].values.len());
                let layer_2 = rander.gen_range(0..self.keys[0].values.len());
                if !(layer_1 == 1 || layer_2 == 1) {
                    let size = self.keys.len();
                    let index_1 = rander.gen_range(0..size);
                    let index_2 = rander.gen_range(0..size);
                    let temp_key_1 = self.keys[index_1].values[layer_1].clone();
                    let temp_key_2 = self.keys[index_2].values[layer_2].clone();
                    match (temp_key_1.0, temp_key_2.0, (temp_key_1.1 || temp_key_2.1)) {
                        (Keycode::KC(_), Keycode::KC(_), false) => {
                            x = false;
                            key_1 = (index_1, layer_1);
                            key_2 = (index_2, layer_2)
                        }
                        _ => continue,
                    };
                }
            }
            let mut values_1 = self.keys[key_1.0].values.clone();
            let mut values_2 = self.keys[key_2.0].values.clone();
            let buf_value = values_2[key_2.1].clone();
            values_2[key_2.1] = values_1[key_1.1].clone();
            values_1[key_1.1] = buf_value;
            self.keys[key_1.0].values = values_1;
            self.keys[key_2.0].values = values_2;
        }
    }

    pub async fn flip(mut self) {
        let mut columns: Vec<Vec<Key>> = Vec::new();
        for i in 0..self.keys.len() {
            columns[self.keys[i].matrix.1 as usize].push(self.keys[i].clone());
        }
        // TODO
    }
    // whar?

    pub async fn mate(mut self, other: Keyboard) {
        let mut buf_keyboard: Keyboard = Keyboard {
            keys: Vec::new(),
            default_positions: self.default_positions,
            fitness: 0,
        };
        let mut missing_indexes: Vec<(usize,usize)> = Vec::new();
        for i in 0..self.keys.len() / 2 {
            buf_keyboard.keys[i] = self.keys[i].clone();
        }
        for i in 0..self.keys[0].values.len() {
            for (idx, x) in self.keys.iter().enumerate() {
                if x.values[i].1 == true {
                    buf_keyboard.keys[idx].values[i] = x.values[i].clone();
                }
            }
            // terrible code
            for idx in self.keys.len() / 2..self.keys.len() {
                let kc = other.keys[idx].values[i].0.clone();
                match buf_keyboard.find_key(kc).await {
                    Some(_) => missing_indexes.push((idx,i)),
                    None => buf_keyboard.keys[idx].values[i] = other.keys[idx].values[i].clone(),
                }
            }
            let missing_keys = self.keys.iter().filter_map(|f| match buf_keyboard.find_key() {
                D
                None => {},
                _ => {None}
            }
            )
        }
    }
}

#[derive(Clone)]
pub struct Key {
    // coords: (f32, f32),
    matrix: (u8, u8),
    values: Vec<(Keycode, bool)>,
    finger: Finger,
    hand: Hand,
    /// how hard it is to get to the position
    cost: usize,
    //TODO cost to reset to position
    // bweight: usize,
}

#[derive(Clone, Copy)]
enum Finger {
    Pinky = 4,
    Ring = 3,
    Middle = 2,
    Index = 1,
    Thumb = 0,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Hand {
    Left,
    Right,
}

/// to make things simpler, MO(1) is shift, MO(0) is base layer(where the alphabet should be theoretically)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keycode {
    KC([String; 2]),
    MO(u8),
    OSL(u8),
    TRANS,
}

pub type Ngrams = Vec<(Vec<Keycode>, u32)>;
