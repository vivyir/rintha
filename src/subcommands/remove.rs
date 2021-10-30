use crate::{
    common::{mod_dir, FullConfig},
    RemoveArg,
};
use bunt::{eprintln, print, println};
use femtorinth::data_structures::ModID;
use std::fs;

// rem? rem??? REM????
// rem is literally best girl

pub fn remove(
    program_config: &mut FullConfig,
    op: RemoveArg,
) -> Result<(), Box<dyn std::error::Error>> {
    match op {
        RemoveArg::Guided => rem_guided(program_config)?,
        RemoveArg::ModID(mod_id) => rem_mod_id(program_config, mod_id)?,
        RemoveArg::Unknown => unreachable!(),
    }

    Ok(())
}

fn rem_mod_id(
    program_config: &mut FullConfig,
    mod_id: ModID,
) -> Result<(), Box<dyn std::error::Error>> {
    let profdir = program_config.get_current_prof_path()?;
    let profname = program_config.current_profile.as_str();
    let profile = &program_config.profiles[profname];

    println!("{$bold}Profile:{/$} {[bold+yellow]}", profile.name);

    if program_config.profiles[program_config.current_profile.as_str()]
        .mods
        .is_none()
    {
        eprintln!("{$bold+red}Error:{/$} {$bold}No mods installed.{/$}");
        std::process::exit(-1);
    }

    // FIXME: probably unneeded allocation, help?
    let idx = match program_config.profiles[program_config.current_profile.as_str()]
        .mods
        .clone()
        .unwrap()
        .iter()
        .position(|val| *val.id.0 == mod_id.0)
    {
        Some(idx) => idx,
        None => {
            eprintln!(
                "{$bold+red}Error:{/$} {$bold}No mod with ID '{}' found.{/$}",
                mod_id.0
            );
            std::process::exit(-1);
        }
    };

    let mut newmods = program_config.profiles[program_config.current_profile.as_str()].clone();
    let rmod = newmods.mods.as_mut().unwrap().remove(idx);

    program_config
        .profiles
        .insert(program_config.current_profile.clone(), newmods);

    fs::remove_file(profdir.join(rmod.current_filename.as_str()))?;
    fs::remove_file(mod_dir()?.join(rmod.current_filename.as_str()))?;

    println!("{$bold+green}Success:{/$} {$bold}Removed from current profile, the mods directory and the manifest!{/$}");

    Ok(())
}

fn rem_guided(program_config: &mut FullConfig) -> Result<(), Box<dyn std::error::Error>> {
    let profdir = program_config.get_current_prof_path()?;
    let profname = program_config.current_profile.as_str();
    let profile = &program_config.profiles[profname];

    println!("{$bold}Profile:{/$} {[bold+yellow]}", profile.name);

    if program_config.profiles[program_config.current_profile.as_str()]
        .mods
        .is_none()
    {
        eprintln!("{$bold+red}Error:{/$} {$bold}No mods installed.{/$}");
        std::process::exit(-1);
    }

    Ok(())
}
