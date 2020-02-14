extern crate yaml_rust;
use yaml_rust::Yaml;

use std::fs;

use crate::errors::{Error, ErrorKind, ResultExt};

pub struct Action
{
  from: String,
  to: String,
  dir: bool,
  owner: String
}

pub fn process(node: &Yaml, block_name: &str) -> Result<Action, Error>
{
  let new_action = Action { from: String::new(), to: String::new(), dir: false, owner: block_name.to_string() };
  Ok(new_action)
}

pub fn execute(action: Action) -> Result<(), Error>
{
  println!("Executing the copy!");
  Ok(())
}
