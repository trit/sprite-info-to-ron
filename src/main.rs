use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::Serialize;

#[derive(Serialize)]
struct Sprite {
    name: String,
    x: i16,
    y: i16,
    width: i16,
    height: i16,
    frames: i16
}

#[derive(Serialize)]
struct Config {
    texture_width: i32,
    texture_height: i32,
    sprites: Vec<Sprite>
}

fn main() {
    let mut sprites = Vec::new();
    let file = File::open("tiles_list_v1.1").unwrap();
    for line in BufReader::new(file).lines().map(|l| l.unwrap()) {
        let settings: Vec<&str> = line.split(" ").collect();

        //println!("{:?}", settings);

        sprites.push(Sprite {
            name: settings[0].to_string(),
            x: settings[1].parse::<i16>().unwrap(),
            y: settings[2].parse::<i16>().unwrap(),
            width: settings[3].parse::<i16>().unwrap(),
            height: settings[4].parse::<i16>().unwrap(),
            frames: if settings.len() == 6 { settings[5].parse::<i16>().unwrap() } else { 1 }
        });
    }

    let conf = Config {
        texture_width: 512,
        texture_height: 512,
        sprites: sprites
    };

    let pretty = PrettyConfig {
        depth_limit: 4,
        separate_tuple_members: true,
        enumerate_arrays: true,
        ..PrettyConfig::default()
    };

    let s = to_string_pretty(&conf, pretty).expect("Serialization failed");

    fs::write("tiles.ron", s).unwrap();
}
