extern crate yaml_rust;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::prelude::*;

fn main()
{
    let mut file = File::open(".linkr").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let cfg = YamlLoader::load_from_str(&contents).unwrap();

    println!("{:?}", cfg);
}
