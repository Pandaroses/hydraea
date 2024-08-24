//! Pandaroses Keyboard Layout Generator (Hydraea)
//! uses a genetic algorithm with many options

use rand::Rng;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
struct JsonKey {
    matrix: Vec<u8>,
    values: Vec<Vec<String>>,
    finger: String,
    hand: String,
    cost: usize,
}

#[derive(Clone, Debug)]
pub struct Keyboard {
    pub keys: Vec<Key>,
    /// specifies default finger positions on the matrix
    pub default_positions: Vec<(u8, u8)>,
    pub fitness: usize,
}

#[derive(Eq, PartialEq, Clone)]
pub enum Roll {
    Inwards,
    Outwards,
}

#[derive(Clone)]
pub struct FitnessConfig {
    pub ngram_multiplier: f32,
    pub roll_type: Roll,
    // non_optimal_column_multiplier: f32,
    pub roll_multiplier: f32,
}

impl Keyboard {
    /// calculates fitness,
    pub async fn calculate_fitness(
        &mut self,
        ngrams: Ngrams,
        config: FitnessConfig,
        bigrams: Vec<(String, usize)>,
    ) {
        let mut ngram_fitness = 0.0;
        // ngram+roll fitness
        for n in ngrams.iter() {
            let mut i = 0;
            let mut keys: Vec<Key> = Vec::new();
            while i < n.0.len() {
                match self.find_key(n.0[i].clone()) {
                    Some(x) => {
                        keys.push(x.clone());
                        if i >= 1 {
                            let y_check = keys[i - 1].matrix.1 == keys[i].matrix.1;
                            let same_hand = keys[i - 1].hand == keys[i].hand;
                            let x_diff = keys[i - 1].matrix.0.abs_diff(keys[i].matrix.0) == 1;
                            let layer_1: usize = x
                                .clone()
                                .values
                                .iter()
                                .enumerate()
                                .filter_map(|m| if m.1 .0 == n.0[i] { Some(m.0) } else { None })
                                .nth(0)
                                .unwrap();
                            let layer_2: usize = self
                                .find_key(n.0[i - 1].clone())
                                .unwrap()
                                .clone()
                                .values
                                .iter()
                                .enumerate()
                                .filter_map(|m| {
                                    if m.1 .0 == n.0[i - 1] {
                                        Some(m.0)
                                    } else {
                                        None
                                    }
                                })
                                .nth(0)
                                .unwrap();
                            if x_diff && same_hand && y_check && layer_1 == layer_2 {
                                i += 1;
                            } else {
                                i = n.0.len() + 100;
                            }
                        } else {
                            i += 1
                        }
                    }
                    None => {
                        i = n.0.len() + 100;
                    }
                };
            }
            if keys.len() == n.0.len()
                && keys.len() == i
                && (keys[0].matrix.0 as i8).abs_diff(keys[1].matrix.0 as i8) == 1
            {
                let roll = match keys[0].matrix.0 as i8 - keys[1].matrix.0 as i8 {
                    -1 if keys[0].hand == Hand::Left => Roll::Inwards,
                    -1 if keys[0].hand == Hand::Right => Roll::Outwards,
                    1 if keys[0].hand == Hand::Left => Roll::Outwards,
                    1 if keys[0].hand == Hand::Right => Roll::Inwards,
                    _ => todo!(),
                };
                let roll_mul = if config.roll_type == roll {
                    config.roll_multiplier
                } else {
                    1.0
                };
                ngram_fitness += n.1 as f32 * n.0.len() as f32 * roll_mul;
            }
            let mut distance_fitness = 0;
            for i in bigrams.iter() {
                let k1 = &i.0[0..1];
                let k2 = &i.0[1..2];
                let first_key = self
                    .find_key(Keycode::KC([k1.to_string().clone(), k1.to_uppercase()]))
                    .unwrap();
                let second_key = self
                    .find_key(Keycode::KC([k2.to_string().clone(), k2.to_uppercase()]))
                    .unwrap();
                let l1: usize = self
                    .find_key(Keycode::KC([k1.to_string().clone(), k1.to_uppercase()]))
                    .unwrap()
                    .clone()
                    .values
                    .iter()
                    .enumerate()
                    .filter_map(|m| {
                        if m.1 .0 == Keycode::KC([k1.to_string().clone(), k1.to_uppercase()]) {
                            Some(m.0)
                        } else {
                            None
                        }
                    })
                    .nth(0)
                    .unwrap();
                let l2: usize = self
                    .find_key(Keycode::KC([k1.to_string().clone(), k1.to_uppercase()]))
                    .unwrap()
                    .clone()
                    .values
                    .iter()
                    .enumerate()
                    .filter_map(|m| {
                        if m.1 .0 == Keycode::KC([k1.to_string().clone(), k1.to_uppercase()]) {
                            Some(m.0)
                        } else {
                            None
                        }
                    })
                    .nth(0)
                    .unwrap();

                if l1 != l2 {
                    distance_fitness += u64::MAX as usize;
                }

                if first_key.hand == second_key.hand {
                    if first_key.finger == second_key.finger {
                        distance_fitness += first_key.cost * second_key.cost * i.1;
                    } else {
                        distance_fitness += (first_key.cost + second_key.cost) * i.1;
                    }
                } else {
                    distance_fitness += (first_key.cost + second_key.cost) * i.1;
                }
            }

            self.fitness = (ngram_fitness * config.ngram_multiplier) as usize
                + ((1.0 / (distance_fitness as f32 / 1000000000000.0)).powf(2.0) * 2000000.0)
                    as usize
        }
    }

    pub fn find_key(&self, kc: Keycode) -> Option<Key> {
        for key in &self.keys {
            for (keycode, _) in &key.values {
                if *keycode == kc {
                    return Some(key.clone());
                }
            }
        }
        None
    }
    pub async fn mutate(&mut self) {
        let mut rander = rand::thread_rng();
        let amount = rander.gen_range(1..self.keys.len());
        for _ in 0..amount {
            let size = self.keys.len();
            let index_1 = rander.gen_range(0..size);
            let index_2 = rander.gen_range(0..size);
            let layer_1 = rander.gen_range(0..self.keys[index_1].values.len());
            let layer_2 = rander.gen_range(0..self.keys[index_2].values.len());

            // Only swap if both keys are KC and not modified
            if let (Keycode::KC([_, _]), false) = (
                self.keys[index_1].values[layer_1].0.clone(),
                self.keys[index_1].values[layer_1].1,
            ) {
                if let (Keycode::KC([_, _]), false) = (
                    self.keys[index_2].values[layer_2].0.clone(),
                    self.keys[index_2].values[layer_2].1,
                ) {
                    // Swap the keycodes
                    let temp = self.keys[index_1].values[layer_1].clone();
                    self.keys[index_1].values[layer_1] = self.keys[index_2].values[layer_2].clone();
                    self.keys[index_2].values[layer_2] = temp;
                }
            }
        }
    }
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let json_keys: Vec<JsonKey> = serde_json::from_str(json)?;

        let keys = json_keys
            .into_iter()
            .map(|jk| Key {
                matrix: (jk.matrix[0], jk.matrix[1]),
                values: jk
                    .values
                    .into_iter()
                    .map(|v| (parse_keycode(&v[0]), v[1] == "true"))
                    .collect(),
                finger: Finger::from_str(&jk.finger).unwrap_or(Finger::Thumb),
                hand: Hand::from_str(&jk.hand).unwrap_or(Hand::Left),
                cost: jk.cost,
            })
            .collect();

        Ok(Keyboard {
            keys,
            default_positions: Vec::new(),
            fitness: 0,
        })
    }

    pub async fn mate(&mut self, other: Keyboard) {
        let mut buf_keyboard: Keyboard = Keyboard {
            keys: vec![
                Key {
                    matrix: (0, 0),
                    values: self.keys[0].values.clone(),
                    finger: Finger::Thumb,
                    hand: Hand::Left,
                    cost: 1,
                };
                self.keys.len()
            ],
            default_positions: self.default_positions.clone(),
            fitness: 0,
        };
        let mut missing_indexes: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.keys.len() / 2 {
            buf_keyboard.keys[i] = self.keys[i].clone();
        }
        for i in 0..self.keys[0].values.len() {
            for (idx, x) in self.keys.iter().enumerate() {
                if x.values[i].1 {
                    buf_keyboard.keys[idx].values[i] = x.values[i].clone();
                }
            }
            // terrible code
            for idx in self.keys.len() / 2..self.keys.len() {
                let kc = other.keys[idx].values[i].0.clone();
                match buf_keyboard.find_key(kc) {
                    Some(_) => missing_indexes.push((idx, i)),
                    None => buf_keyboard.keys[idx].values[i] = other.keys[idx].values[i].clone(),
                }
            }
        }
        let missing_keys: Vec<(usize, usize)> = self
            .keys
            .iter()
            .enumerate()
            .filter_map(|f| {
                let x: Vec<(usize, usize)> =
                    f.1.values
                        .iter()
                        .enumerate()
                        .filter_map(|k| match self.find_key(k.1 .0.clone()) {
                            Some(_) => None,
                            None => Some((f.0, k.0)),
                        })
                        .collect();
                Some(x)
            })
            .flatten()
            .collect();
        if missing_keys.len() > 0 {
            for i in 0..missing_indexes.len() {
                buf_keyboard.keys[missing_indexes[i].0].values[missing_indexes[i].1] =
                    self.keys[missing_keys[i].0].values[missing_keys[i].1].clone()
            }
        }
    }
}

fn parse_keycode(s: &str) -> Keycode {
    if s.starts_with("KC(") && s.ends_with(")") {
        let inner = &s[3..s.len() - 1];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 2 {
            return Keycode::KC([parts[0].to_string(), parts[1].to_string()]);
        }
    } else if s.starts_with("MO(") && s.ends_with(")") {
        if let Ok(num) = s[3..s.len() - 1].parse::<u8>() {
            return Keycode::MO(num);
        }
    } else if s.starts_with("OSL(") && s.ends_with(")") {
        if let Ok(num) = s[3..s.len() - 1].parse::<u8>() {
            return Keycode::OSL(num);
        }
    } else if s == "TRANS" {
        return Keycode::TRANS;
    }
    // If none of the above conditions are met, treat it as a KC key
    Keycode::KC([s.to_string(), s.to_uppercase()])
}

impl FromStr for Finger {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pinky" => Ok(Finger::Pinky),
            "Ring" => Ok(Finger::Ring),
            "Middle" => Ok(Finger::Middle),
            "Index" => Ok(Finger::Index),
            "Thumb" => Ok(Finger::Thumb),
            _ => Err(()),
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Left" => Ok(Hand::Left),
            "Right" => Ok(Hand::Right),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Key {
    // coords: (f32, f32),
    pub matrix: (u8, u8),
    pub values: Vec<(Keycode, bool)>,
    finger: Finger,
    hand: Hand,
    /// how hard it is to get to the position
    cost: usize,
    //TODO cost to reset to position
    // bweight: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Finger {
    Pinky = 4,
    Ring = 3,
    Middle = 2,
    Index = 1,
    Thumb = 0,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
