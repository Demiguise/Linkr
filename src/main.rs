//Setup file handling
use std::fs::File;
use std::io::prelude::*;

//YAML modules
extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

//Error chain setup
#[macro_use]
extern crate error_chain;
mod errors;
use crate::errors::{Error, ErrorKind, ResultExt};

mod symlink;
mod copy;

enum ActionTypes
{
    Symlink(symlink::Action),
    Copy(copy::Action),
}

fn load_config(filename: &str) -> Result<Yaml, Error>
{
    let mut file = File::open(filename)
        .chain_err(|| "Unable to open file")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .chain_err(|| "Unable to read contents of file")?;

    let data = YamlLoader::load_from_str(&contents)
        .chain_err(|| "Failed to parse contents as YAML")?;

    match data.into_iter().nth(0)
    {
        Some(i) => { Ok(i) }
        None => { Err("Contents contained no YAML".into()) }
    }
}

fn main()
{
    if let Err(ref e) = run()
    {
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1)
        {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace()
        {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn process(node: &Yaml, block_name: &str, actions: &mut Vec<ActionTypes>) -> Result<(), Error>
{
    match *node
    {
        Yaml::Hash(ref hash) =>
        {
            for (k, v) in hash
            {
                let module_name = k.as_str()
                    .chain_err(|| "Unable to parse module name as a string")?;

                match module_name
                {
                    "symlink" =>
                    {
                        println!("Processing a symlink");
                        actions.push(ActionTypes::Symlink(symlink::process(v, block_name)?));
                    }
                    "copy" =>
                    {
                        println!("Processing a copy");
                        actions.push(ActionTypes::Copy(copy::process(v, block_name)?));
                    }
                    _ =>
                    {
                        return Err(ErrorKind::Msg(format!("Unknown module [{}] for [{}]", module_name, block_name).to_string()).into());
                    }
                }
            }
        }
        _ =>
        {
            return Err(ErrorKind::Msg(format!("Expected module block for [{}]", block_name).to_string()).into());
        }
    }
    Ok(())
}

fn run() -> Result<(), Error>
{
    let cfg = load_config(".linkr")?;
    let mut actions = Vec::<ActionTypes>::new();

    match &cfg
    {
        Yaml::Hash(ref hash) =>
        {
            for (k, v) in hash
            {
                let block_name = k.as_str()
                    .chain_err(|| "Unable to parse top level block name as a string")?;

                process(v, block_name, &mut actions)?;
            }
        }
        _ =>
        {
            return Err(ErrorKind::Msg("Expected YAML hash map".to_string()).into());
        }
    }

    for action in actions
    {
        match action
        {
            ActionTypes::Symlink(item) =>
            {
                symlink::execute(item)?;
            }
            ActionTypes::Copy(item) =>
            {
                copy::execute(item)?;
            }
        }
    }

    Ok(())
}
