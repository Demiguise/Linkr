extern crate yaml_rust;
#[macro_use]
extern crate error_chain;
use yaml_rust::{Yaml, YamlLoader};
use std::fs::File;
use std::io::prelude::*;

mod errors {
    error_chain! { }
}

use errors::*;

fn load_config(filename: &str) -> Result<Option<Yaml>>
{
    let mut file = File::open(filename)
        .chain_err(|| "Unable to open file")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .chain_err(|| "Unable to read contents of file")?;

    let data = YamlLoader::load_from_str(&contents)
        .chain_err(|| "Failed to parse conents as YAML")?;

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

fn run() -> Result<()>
{
    let cfg = load_config(".linkr")?;

    for block in cfg
    {
        println!("{:?}\n---", block);
    }

    Ok(())
}
