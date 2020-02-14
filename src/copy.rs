extern crate yaml_rust;
use yaml_rust::Yaml;

use std::fs;

pub struct Action
{
  from: String,
  to: String,
  dir: bool,
}

pub fn process(node: &Yaml) -> Action
{
  Action { from: String::new(), to: String::new(), dir: false }
}
