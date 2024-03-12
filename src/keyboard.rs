//! Pandaroses Keyboard Layout Generator (Hydraea)
//! uses a genetic algorithm with many options

pub struct Keyboard {
    keys: Vec<Key>,
    /// specifies default finger positions on the matrix
    default_positions: Vec<(u8, u8)>,
    fitness: usize,
}

enum Roll {
    Inwards,
    Outwards,
}

pub struct FitnessConfig {
    ngram_multiplier: f32,
    roll_type: Roll,
    non_optimal_column_multiplier: f32,
    roll_multiplier: f32,
}

impl Keyboard {
    /// calculates fitness,
    pub async fn calculate_fitness(mut self, ngrams: Ngrams) {
        let fitness = 0;
        let mut ngram_fitness = 0;
        for i in 0..9 {
            for n in ngrams.iter() {
                ngram_fitness = 0;
                for key in &n.0 {
                    self.find_key(key.clone()).await;
                }
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

#[derive(Clone, Copy)]
enum Hand {
    Left,
    Right,
}

/// to make things simpler, MO(1) is shift, MO(0) is base layer(where the alphabet should be theoretically)
#[derive(Clone, PartialEq, Eq)]
pub enum Keycode {
    KC([String; 2]),
    MO(u8),
    OSL(u8),
    TRANS,
}

pub type Ngrams = Vec<(Vec<Keycode>, u32)>;
