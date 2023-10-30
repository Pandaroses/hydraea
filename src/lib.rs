use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{default, fs::read_to_string};
#[derive(Debug)]
pub struct Key {
    pub x: f32,
    pub y: f32,
    pub id: String,
}
impl Default for Key {
    fn default() -> Key {
        todo!();
    }
}
#[derive(Debug)]
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
// TODO x2, y2 i think treat as two separate keys, each having the same id
// w,h only applies to next key
//r stays until overwritten, even over rows, if rx,ry is specified rotate around that point r in deg rx,ry stays until overwriten aswell
// else rotate around 0,0 for no apparent reason
// apparently the fucking rotation is applied after the coordinate rows what the fuck
//  e.g. you calculate the absolute coordinate of where the key is in the "row" by adding specified y or w value,
// then you do a rotation around point rx,ry or 0,0 using
// x' = xcos(r) - ysin(r) and y' = ycos(r) + xsin(r)
// additionally if rx,ry are not 0,0 then you have to transform the position until origin is 0,0, then commit rotation and untransform
// implementation, finds middle position of keycap, then rotate around point

pub fn format_json_kle(path: String) -> Keyboard {
    let mut keyboard: Layer = Layer::new();
    let meow: Vec<Value> =
        serde_json::from_str(&read_to_string(path.to_string()).unwrap()).unwrap();
    let mut current: Index = Index::default();
    for i in 0..meow.len() {
        let mrrow = meow[i].as_array().unwrap();
        for r in 0..mrrow.len() {
            match mrrow[r].is_object() {
                true => {
                    current = Index {
                        x: current.x + mrrow[r]["x"].as_f64().unwrap_or(0.0),
                        y: current.y + mrrow[r]["y"].as_f64().unwrap_or(0.0),
                        h: mrrow[r]["h"].as_f64().unwrap_or(1.0),
                        w: mrrow[r]["w"].as_f64().unwrap_or(1.0),
                        r: mrrow[r]["r"].as_f64().unwrap_or(current.r),
                        rx: mrrow[r]["rx"].as_f64().unwrap_or(current.rx),
                        ry: mrrow[r]["ry"].as_f64().unwrap_or(current.ry),
                        x2: mrrow[r]["x2"].as_f64(),
                        y2: mrrow[r]["y2"].as_f64(),
                        w2: mrrow[r]["w2"].as_f64(),
                        h2: mrrow[r]["h2"].as_f64(),
                    };
                }
                false => {
                    let (mut ex, mut ey) = (
                        (current.x + (current.w / 2.0)),
                        (current.y + (current.h / 2.0)),
                    );
                    println!("{:?} , {:?}", mrrow[r].as_str().unwrap(), (ex, ey));
                    let (mut dx, mut dy) = (current.rx, current.ry);
                    (ex, ey) = (ex - dx, -(ey - dy));
                    ex = ex * f64::cos(current.r) - ey * f64::sin(current.r);
                    ey = ey * f64::cos(current.r) + ex * f64::sin(current.r);
                    (ex, ey) = (ex + dx, ex + dy);
                    current.x += current.w;
                    let (mut ex2, mut ey2) = (
                        (current.x + current.x2.unwrap_or(1.0) + (current.w2.unwrap_or(1.0) / 2.0)),
                        (current.y + current.y2.unwrap_or(1.0) + (current.h2.unwrap_or(1.0) / 2.0)),
                    );
                    let i = nanoid!(10);
                    if (current.x2.is_some()
                        | current.y2.is_some()
                        | current.w2.is_some()
                        | current.h2.is_some())
                    {
                        keyboard.push(Key {
                            x: ex2 as f32,
                            y: ey2 as f32,
                            id: i.to_string(),
                        });
                    }
                    keyboard.push(Key {
                        x: ex as f32,
                        y: ey as f32,
                        id: i,
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

    //
    //
    //

    let mut r: Vec<Layer> = Vec::new();
    r.push(keyboard);
    return Keyboard { layers: r };
}
