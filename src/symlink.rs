extern crate yaml_rust;
use yaml_rust::Yaml;

use crate::errors::{Error, ErrorKind, ResultExt};

pub struct Action
{
  from: String,
  to: String,
  dir: bool,
  owner: String,
}

pub fn process(node: &Yaml, block_name: &str) -> Result<Action, Error>
{
  let mut new_action = Action { from: String::new(), to: String::new(), dir: false, owner: block_name.to_string() };

  match *node
  {
    Yaml::Hash(ref hash) =>
    {
      for (k, v) in hash
      {
        let param = k.as_str()
            .chain_err(|| "Unable to parse parameter name as a string")?;

        match param
        {
          "from" => { new_action.from = v.as_str().chain_err(|| "Unable to parse \'from\' as a string")?.to_string(); }
          "to" => { new_action.to = v.as_str().chain_err(|| "Unable to parse \'to\' as a string")?.to_string(); }
          "dir" => { new_action.dir = v.as_bool().chain_err(|| "Unable to parse \'dir\' as a boolean")?; }
          _ =>
          {
            return Err(ErrorKind::Msg(format!("Unknown parameter type [{}]", param).to_string()).into());
          }
        }
      }
    }
    _ =>
    {
      return Err(ErrorKind::Msg(format!("").to_string()).into());
    }
  }

  Ok(new_action)
}

pub fn execute(action: Action) -> Result<(), Error>
{
  println!("[Symlink] ({}) => ({})", action.from, action.to);
  Ok(())
}
