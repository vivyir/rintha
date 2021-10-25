use clap::{load_yaml, App};

use crate::common::FullConfig;

mod common;
mod subcommands;

pub enum Subcommand {
    Get(String, Option<usize>),
    Unknown,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* configuration handling code begin */
    let mut program_config: FullConfig = confy::load("rintha")?;
    // TODO: integrity checks for the full config on load
    /* configuration handling code end */

    /* cli interface handling code begin */
    let yaml = load_yaml!("cli.yaml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    let mut command: Subcommand = Subcommand::Unknown;
    if let Some(submatches) = matches.subcommand_matches("get") {
        if submatches.value_of("query").unwrap().chars().count() >= 3 {
            command = Subcommand::Get(
                submatches.value_of("query").unwrap().to_string(), // value is required
                match submatches.value_of("limit") {
                    Some(limit) => Some(limit.parse::<usize>()?),
                    None => None,
                },
            );
        } else {
            println!("Query must be longer than or equal to 3 characters.");
            std::process::exit(-1);
        }
    }
    let command = command;

    match command {
        Subcommand::Get(query, limit) => subcommands::get(&mut program_config, query, limit)?,
        Subcommand::Unknown => println!("No such subcommand."),
    }
    /* cli interface handling code end */

    confy::store("rintha", program_config)?;
    Ok(())
}
