use nanoid::nanoid;
use serde_json::Value;
use std::{f64::consts::PI, fs::read_to_string};
pub mod genes;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Key {
    pub x: f32,
    pub y: f32,
    pub id: String,
    pub value: Option<Keycode>,
    pub fixed: bool,
}
#[derive(Debug, Clone)]
pub struct Keyboard {
    pub layers: Vec<Layer>,
}
#[derive(Debug)]
struct Index {
    x: f64,
    y: f64,
    h: f64,
    w: f64,
    r: f64,
    rx: f64,
    ry: f64,
    x2: Option<f64>,
    y2: Option<f64>,
    w2: Option<f64>,
    h2: Option<f64>,
}
impl Default for Index {
    fn default() -> Self {
        Index {
            x: 0.0,
            y: 0.0,
            h: 1.0,
            w: 1.0,
            r: 0.0,
            rx: 0.0,
            ry: 0.0,
            x2: None,
            y2: None,
            w2: None,
            h2: None,
        }
    }
}
pub type Layer = Vec<Key>;

//RULES
// x,y,w,h units keyboard units u
// x,y reset each row, in row apply to every proceeding key until new overwrite
// w,h only applies to next key
// r stays until overwritten, even over rows, if rx,ry is specified rotate around that point r in deg rx,ry stays until overwriten aswell
// else rotate around 0,0 for no apparent reason
// apparently the fucking rotation is applied after the coordinate rows what the fuck
//  e.g. you calculate the absolute coordinate of where the key is in the "row" by adding specified y or w value,
// then you do a rotation around point rx,ry or 0,0 using
// x' = xcos(r) - ysin(r) and y' = ycos(r) + xsin(r)
// additionally if rx,ry are not 0,0 then you have to transform the position until origin is 0,0, then commit rotation and untransform
// implementation, finds middle position of keycap, then rotate around point

//TODO ALL THE THINGS ARE RELATIVE FROM 0,0 REMEMBER IT GOES NEGATIVE NOT POSITIVE YOU SILLY BILLYx

/// this function takes the path to your KLE json file and tries to deserialize it into a keyboard struct
/// **Note** currently does not support rotation as KLE rotation is outside of my skill range
pub fn format_json_kle(path: String) -> Keyboard {
    let mut keyboard: Layer = Layer::new();
    let json: Vec<Value> = serde_json::from_str(&read_to_string(&path).unwrap()).unwrap();
    let mut current: Index = Index::default();
    for i in 0..json.len() {
        let next = json[i].as_array().unwrap();
        for r in 0..next.len() {
            match next[r].is_object() {
                true => {
                    current = Index {
                        x: current.x + next[r]["x"].as_f64().unwrap_or(0.0),
                        y: current.y + next[r]["y"].as_f64().unwrap_or(0.0),
                        h: next[r]["h"].as_f64().unwrap_or(1.0),
                        w: next[r]["w"].as_f64().unwrap_or(1.0),
                        r: next[r]["r"].as_f64().unwrap_or(current.r),
                        rx: next[r]["rx"].as_f64().unwrap_or(current.rx),
                        ry: next[r]["ry"].as_f64().unwrap_or(current.ry),
                        x2: next[r]["x2"].as_f64(),
                        y2: next[r]["y2"].as_f64(),
                        w2: next[r]["w2"].as_f64(),
                        h2: next[r]["h2"].as_f64(),
                    };
                }
                false => {
                    let (mut ex, mut ey) = (
                        (current.x + (current.w / 2.0)),
                        (current.y + (current.h / 2.0)),
                    );
                    match next[r].as_str().unwrap() {
                        "Shift" => {
                            println!(
                                "x,y: {:?},{:?} |r: {:?} |rx,ry: {:?},{:?}",
                                ex, ey, r, current.rx, current.ry
                            )
                        }
                        _ => {}
                    };
                    let (dx, dy) = (current.rx, current.ry);
                    (ex, ey) = (ex - dx, -(ey - dy));
                    ex = ex * f64::cos(current.r * 180.0 / PI)
                        - ey * f64::sin(current.r * 180.0 / PI);
                    ey = ey * f64::cos(current.r) + ex * f64::sin(current.r);
                    (ex, ey) = (ex + dx, ey + dy);
                    current.x += current.w;
                    let (ex2, ey2) = (
                        (current.x + current.x2.unwrap_or(1.0) + (current.w2.unwrap_or(1.0) / 2.0)),
                        (current.y + current.y2.unwrap_or(1.0) + (current.h2.unwrap_or(1.0) / 2.0)),
                    );
                    let i = nanoid!(5);
                    if current.x2.is_some()
                        | current.y2.is_some()
                        | current.w2.is_some()
                        | current.h2.is_some()
                    {
                        keyboard.push(Key {
                            x: ex2 as f32,
                            y: ey2 as f32,
                            id: i.to_string(),
                            value: Some(Keycode::KC([
                                next[r].as_str().unwrap().to_string().to_lowercase(),
                                next[r].as_str().unwrap().to_string().to_lowercase(),
                            ])),
                            fixed: false,
                        });
                    }
                    keyboard.push(Key {
                        x: ex as f32,
                        y: ((ey * -1.0) + 20.0) as f32,
                        id: i,
                        value: Some(Keycode::KC([
                            next[r].as_str().unwrap().to_string().to_lowercase(),
                            next[r].as_str().unwrap().to_string().to_lowercase(),
                        ])),
                        fixed: false,
                    });
                    current.w = 1.0;
                    current.h = 1.0;
                }
            }
        }
        current = Index {
            x: 0.0,
            y: current.y + 1.0,
            r: current.r,
            rx: current.rx,
            ry: current.ry,
            ..Default::default()
        };
    }

    let mut r: Vec<Layer> = Vec::new();
    r.push(keyboard);
    Keyboard { layers: r }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum Keycode {
    KC([String; 2]),
    DF(i32),
    MO(i32),
    OSL(i32),
    TG(i32),
}

pub fn default_keycodes() -> Vec<Keycode> {
    let meow: String =
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ1!2@3#4$5%6^7&8*9(0)".to_string();
    let mut res = Vec::new();
    for i in 0..(meow.len() / 2) {
        res.push(Keycode::KC([
            String::from(meow.chars().nth(i).unwrap()),
            String::from(meow.chars().nth(i + 1).unwrap()),
        ]));
    }
    return res;
}
