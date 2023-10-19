use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::read_to_string;

pub struct Key {
    x: f32,
    y: f32,
}
impl Default for Key {
    fn default() -> Key {
        todo!();
    }
}
pub struct Keyboard {
    layers: Vec<Layer>,
}
#[derive(Debug)]
struct Jkey {
    x: f64,
    y: f64,
    h: f64,
    w: f64,
}
impl Default for Jkey {
    fn default() -> Self {
        Jkey {
            x: 0.0,
            y: 0.0,
            h: 1.0,
            w: 1.0,
        }
    }
}
pub type Layer = Vec<Key>;

//RULES
// x,y,w,h units keyboard units u
// x,y reset each row, in row apply to every proceeding key until new overwrite
// x2, y2 big ass enter moment
// w,h only applies to next key
// r stays until overwritten, even over rows, if rx,ry is specified rotate around that point, r is angle

//TODO
// probably rewrite most of it
// i think keep Jkey for now but change it
// fuck the kle dude
pub fn format_json_kle(path: String) -> Keyboard {
    let keyboard: Layer = Layer::new();
    let meow: Vec<Value> =
        serde_json::from_str(&read_to_string(path.to_string()).unwrap()).unwrap();
    let (mut x, mut y) = (0.0, 0.0);
    for i in 0..meow.len() {
        let mrrow = meow[i].as_array().unwrap();
        for r in 0..mrrow.len() {
            let (mut mx, mut my) = (x, y);
            let mut kar: Jkey = Jkey::default();
            match mrrow[r].is_object() {
                true => {
                    kar = Jkey {
                        x: mrrow[r]["x"].as_f64().unwrap_or(0.0),
                        y: mrrow[r]["y"].as_f64().unwrap_or(0.0),
                        h: mrrow[r]["h"].as_f64().unwrap_or(1.0),
                        w: mrrow[r]["w"].as_f64().unwrap_or(1.0),
                    };
                }
                false => {}
            }
            if kar.x != 0.0 {
                mx = kar.x;
            } else {
            }
            if kar.y != 0.0 {
                my = kar.y;
            }
        }
        y = i as f64;
    }

    //
    //
    //

    let mut r: Vec<Layer> = Vec::new();
    r.push(keyboard);
    return Keyboard { layers: r };
}
