use bunt::eprintln;
use clap::{load_yaml, App, ArgMatches};
use femtorinth::data_structures::ModID;

use crate::common::FullConfig;

mod common;
mod subcommands;

pub enum RemoveArg {
    Guided,
    ModID(ModID),
    Unknown,
}

pub enum Subcommand {
    Get(String, Option<usize>),
    Remove(RemoveArg),
    List { full: bool },
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
    parse_cli(matches, &mut command);
    let command = command;

    match command {
        Subcommand::Get(query, limit) => subcommands::get(&mut program_config, query, limit)?,
        Subcommand::Remove(op) => subcommands::remove(&mut program_config, op)?,
        Subcommand::List { full } => subcommands::list(&mut program_config, full),
        Subcommand::Unknown => println!("No such subcommand."),
    }
    /* cli interface handling code end */

    confy::store("rintha", program_config)?;
    Ok(())
}

fn parse_cli(matches: ArgMatches, command: &mut Subcommand) {
    if let Some(submatches) = matches.subcommand_matches("get") {
        if submatches.value_of("query").unwrap().chars().count() >= 3 {
            *command = Subcommand::Get(
                submatches.value_of("query").unwrap().to_string(), // value is required
                match submatches.value_of("limit") {
                    Some(limit) => Some(limit.parse::<usize>().unwrap()),
                    None => None,
                },
            );
        } else {
            eprintln!("{$bold+red}Error:{/$} {$bold}Query must be longer than or equal to 3 characters.{/$}");
            std::process::exit(-1);
        }
    } else if let Some(submatches) = matches.subcommand_matches("remove") {
        if submatches.is_present("guided") {
            *command = Subcommand::Remove(RemoveArg::Guided)
        } else if submatches.is_present("mod-id") {
            *command = Subcommand::Remove(RemoveArg::ModID(ModID(
                submatches.value_of("mod-id").unwrap().into(),
            )))
        }
    } else if let Some(submatches) = matches.subcommand_matches("list") {
        if submatches.is_present("full") {
            *command = Subcommand::List { full: true };
        } else {
            *command = Subcommand::List { full: false };
        }
    }
}
