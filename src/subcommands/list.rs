use crate::common::FullConfig;
use bunt::{eprintln, print, println};

pub fn list(program_config: &mut FullConfig, full_list: bool) {
    if full_list {
        list_full(program_config);
    } else {
        list_normal(program_config);
    }
}

fn list_normal(fc: &FullConfig) {
    let profname = fc.current_profile.as_str();
    let profile = &fc.profiles[profname];

    println!("{$bold}Profile:{/$} {[bold+yellow]}", profile.name);
    if profile.mods.is_none() {
        eprintln!("{$bold+red}Error:{/$} {$bold}No mods have been installed yet!{/$}");
        std::process::exit(-1);
    }

    for i in profile.mods.as_ref().unwrap() {
        // safe to unwrap, already checked
        println!(
            "{$bold+cyan}Mod ID: [{[blue]}]{/$} {[bold+yellow]} (by {[bold+blue]}) ({[bold+intense+green]:?}): {[italic+cyan]}",
            i.id.0, i.title, i.author_username, i.installed_version_type, i.small_description
        );
        println!(
            "Version ID: {[bold+cyan]}, licensed under \"{[bold+cyan]}\" and the latest supported mc version is {[bold+cyan]}",
            i.installed_version_id.0, i.license, i.latest_mc_ver
        );
        println!();
    }
}

fn list_full(fc: &FullConfig) {
    let profname = fc.current_profile.as_str();
    let profile = &fc.profiles[profname];

    println!("{$bold}Profile:{/$} {[bold+yellow]}", profile.name);
    if profile.mods.is_none() {
        eprintln!("{$bold+red}Error:{/$} {$bold}No mods have been installed yet!{/$}");
        std::process::exit(-1);
    }

    for i in profile.mods.as_ref().unwrap() {
        println!(
            "{[bold+yellow]} by {[bold+intense+blue]}, {[bold+intense+green]:?}: {[italic+cyan]}",
            i.title, i.author_username, i.installed_version_type, i.small_description
        );

        println!("{$bold+cyan}Mod ID: [{[green]}]{/$}", i.id.0);
        println!(
            "{$bold+cyan}Version ID: [{[green]}]{/$}",
            i.installed_version_id.0
        );
        println!("{$bold+cyan}Mod license: [{[green]}]{/$}", i.license);
        println!(
            "{$bold+cyan}Mod version number: [{[green]}]{/$}",
            i.installed_version_number
        );
        println!("{$bold+cyan}SHA-1: [{[green]}]{/$}", i.sha1);
        println!("{$bold+cyan}Filename: [{[green]}]{/$}", i.current_filename);

        print!("{$bold+cyan}Supported minecraft versions: {/$}");
        for j in &i.supported_game_versions {
            print!("{[intense+green]} ", j);
        }

        println!("\n");
    }
}
