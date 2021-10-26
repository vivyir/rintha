use crate::common::{FullConfig, RinthaError};
use bunt::{eprintln, print, println};

pub fn list(
    program_config: &mut FullConfig,
    full_list: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !full_list {
        return list_normal(&program_config);
    } else {
        return list_full(&program_config);
    }
}

fn list_normal(fc: &FullConfig) -> Result<(), Box<dyn std::error::Error>> {
    let profname = fc.current_profile.as_str();
    let profile = &fc.profiles[profname];

    println!("{$bold}Profile:{/$} {[bold+yellow]}", profile.name);
    if profile.mods.is_none() {
        println!("{$bold+red}Error:{/$} {$bold}No mods have been installed yet!{/$}");
        std::process::exit(-1);
    }

    for chungus in profile.mods.as_ref().unwrap() {
        // safe to unwrap, already checked
        println!(
            "{$bold+cyan}Mod ID: [{[blue]}]{/$} {[bold+yellow]} (by {[bold+blue]}): {[italic+cyan]}",
            chungus.id.0, chungus.title, chungus.author_username, chungus.small_description
        );
    }

    Ok(())
}

fn list_full(fc: &FullConfig) -> Result<(), Box<dyn std::error::Error>> {
    let profname = fc.current_profile.as_str();
    let profile = &fc.profiles[profname];

    println!("{$bold}Profile:{/$} {[bold+yellow]}", profile.name);
    if profile.mods.is_none() {
        println!("{$bold+red}Error:{/$} {$bold}No mods have been installed yet!{/$}");
        std::process::exit(-1);
    }

    Ok(())
}
