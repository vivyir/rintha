use clap::{load_yaml, App};

mod common;
mod subcommands;

pub enum Subcommand {
    Get(String, Option<usize>),
    Unknown,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* cli handling code begin */
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
        Subcommand::Get(query, limit) => subcommands::get(query, limit)?,
        Subcommand::Unknown => println!("No such subcommand."),
    }
    /* cli handling code end */

    Ok(())
}
