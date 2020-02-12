extern crate yaml_rust;
use yaml_rust::Yaml;

pub fn process(node: &Yaml)
{
  match *node
  {
    Yaml::Hash(ref hash) =>
    {
      for (k, v) in hash
      {
        println!("[Symlink] {:?} -> {:?}", k, v);
      }
    }
    _ =>
    {

    }
  }
}
