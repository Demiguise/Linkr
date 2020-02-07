extern crate yaml_rust;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::prelude::*;

fn load_config(filename: &str) -> Result<Vec<yaml_rust::Yaml>, yaml_rust::ScanError>
{
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    YamlLoader::load_from_str(&contents)
}

fn main()
{
    let cfg = load_config(".linkr").unwrap();
    println!("{:?}", cfg);
}
