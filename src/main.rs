//Setup file handling
use std::fs::File;
use std::io::prelude::*;

//YAML modules
extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

//Error chain setup
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain! { }
}
use errors::*;

mod symlink;
mod copy;

fn load_config(filename: &str) -> Result<Option<Yaml>>
{
    let mut file = File::open(filename)
        .chain_err(|| "Unable to open file")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .chain_err(|| "Unable to read contents of file")?;

    let data = YamlLoader::load_from_str(&contents)
        .chain_err(|| "Failed to parse contents as YAML")?;

    Ok(data.into_iter().nth(0))
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

fn process_module(module: &str, node: &Yaml) -> Result<()>
{
    match module
    {
        "symlink" =>
        {
            println!("Processing a symlink");
            symlink::process(node);
        }
        "copy" =>
        {
            println!("Processing a copy");
            copy::process(node);
        }
        _ =>
        {
        }
    }
    Ok(())
}

fn process_block(node: &Yaml) -> Result<()>
{
    match *node
    {
        Yaml::Hash(ref hash) =>
        {
            for (k, v) in hash
            {
                let module_name = k.as_str()
                    .chain_err(|| "Unable to parse module name as a string")?;

                process_module(module_name, v)?;
            }
        }
        _ =>
        {

        }
    }
    Ok(())
}

fn process(node: &Yaml) -> Result <()>
{
    match *node
    {
        Yaml::Hash(ref hash) =>
        {
            for (k, v) in hash
            {
                let block_name = k.as_str()
                    .chain_err(|| "Unable to parse top level block name as a string")?;

                println!("Top Level: {}", block_name);
                process_block(v)?;
            }
        }
        _ =>
        {

        }
    }

    Ok(())
}

fn run() -> Result<()>
{
    let cfg = load_config(".linkr")?;

    for block in cfg.iter()
    {
        process(block)?;
    }

    Ok(())
}
