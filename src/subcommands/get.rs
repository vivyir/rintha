use crate::common::{
    mod_dir, shallow_search, version_list, ConfigMod, FullConfig, ModID, ModReleaseType,
    RinthaError, VersionID,
};
use bunt::{eprintln, print, println};
use downloader::{Download, Downloader};
use sha1::Digest;
use std::{io::Write, mem};

pub fn get(
    program_config: &mut FullConfig,
    query: String,
    limit: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    /* config mod variables to commit if tx successful */
    let c_id: ModID;
    let c_title: String;
    let c_author_username: String;
    let c_small_description: String;
    let c_latest_mc_ver: String;
    let c_license: String;
    let c_sha1: String;
    /* -------------Version related data-------------- */
    let c_installed_version_id: VersionID;
    let c_installed_version_number: String;
    let c_installed_version_type: ModReleaseType;
    let c_supported_game_versions: Vec<String>;
    /* -----------VersionFile related data------------ */
    let c_current_filename: String;
    /* ----------------------------------------------- */

    let results = shallow_search(query, limit)?;
    for (n, i) in results.iter().enumerate() {
        println!(
            "{$bold+cyan}[{[blue]}]{/$} {[bold+yellow]} (by {[bold+blue]}): {[italic+cyan]}",
            n, i.title, i.author_username, i.small_description
        );
        println!(
            "Downloaded {[bold+cyan]} times, licensed under \"{[bold+cyan]}\" and the latest supported mc version is {[bold+cyan]}",
            i.downloads, i.license, i.latest_mc_ver
        );
        println!();
    }

    // FIXME: get a proper line reader
    print!("{$bold}Enter your choice: {/$}");
    std::io::stdout().flush()?;
    let mut string = String::new();
    std::io::stdin().read_line(&mut string)?;
    let choice1: usize = string.trim().parse()?;

    if choice1 >= results.len() {
        eprintln!("{$bold}Choice was over the limit, exiting...{/$}");
        std::process::exit(-1);
    }

    println!(
        "{$bold}Getting info for \"{[yellow]}\"...{/$}\n",
        results[choice1].title
    );

    c_id = results[choice1].id.clone();
    c_title = results[choice1].title.clone();
    c_author_username = results[choice1].author_username.clone();
    c_small_description = results[choice1].small_description.clone();
    c_latest_mc_ver = results[choice1].latest_mc_ver.clone();
    c_license = results[choice1].license.clone();

    let versions = version_list(results[choice1].id.clone())?;
    if versions.len() >= 10 {
        println!("{$bold}10 or more results were returned, press enter after each 5 versions are shown to continue...{/$}");
    }

    let mut tmp = String::with_capacity(32); // micro-optimization, not a magic number dw, String::new() will work fine aswell
    for (i, ver) in versions.iter().enumerate() {
        println!(
            "{$bold+cyan}[{[blue]}]{/$} {[bold+yellow]} ({[italic+bold+magenta]}) ({[bold+green] :?})",
            i, ver.name, ver.version_number, ver.version_type,
        );
        print!("{$bold}Supported loader(s): {/$}");
        for loader in &ver.loaders {
            print!("{[green]} ", loader);
        }
        println!();
        print!("{$bold}Supported minecraft version(s): {/$}");
        for version in &ver.game_versions {
            print!("{[green]} ", version);
        }
        println!();
        println!(
            "{$bold}Dependency count: {[blue]}{/$}",
            ver.dependencies.len()
        );
        println!();

        if (versions.len() >= 10) && (i % 5 == 0) {
            std::io::stdin().read_line(&mut tmp)?;
        }
    }
    mem::drop(tmp);

    // FIXME: get a proper line reader
    print!("{$bold}Enter your choice: {/$}");
    std::io::stdout().flush()?;
    let mut string = String::new();
    std::io::stdin().read_line(&mut string)?;
    let choice2: usize = string.trim().parse()?;

    if choice2 >= versions.len() {
        eprintln!("{$bold}Choice was over the limit, exiting...{/$}");
        std::process::exit(-1);
    }

    c_installed_version_id = versions[choice2].id.clone();
    c_installed_version_number = versions[choice2].version_number.clone();
    c_installed_version_type = versions[choice2].version_type;
    c_supported_game_versions = versions[choice2].game_versions.clone();

    let final_choice = versions[choice2].clone();

    println!("Downloading {[bold+yellow]}...", final_choice.name);
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("."))
        .parallel_requests(1)
        .build()?;

    let modfile = Download::new(final_choice.files[0].url.as_str()).file_name(
        std::path::Path::new(final_choice.files[0].filename.clone().as_str()),
    );

    let result = downloader.download(&[modfile])?;
    for r in result {
        match r {
            Err(e) => println!("{$bold}Error occured!{/$} {[bold+red]}", e.to_string()),
            Ok(s) => println!("{$bold}Success:{/$} {[bold+green]}", &s),
        };
    }

    if let Some(hash) = final_choice.files[0].hashes.get("sha1") {
        let modfile = std::fs::read(final_choice.files[0].filename.clone())?;
        let sha1_hash = format!("{:x}", sha1::Sha1::digest(&modfile));
        c_sha1 = sha1_hash.clone();
        mem::drop(modfile);

        if *hash == sha1_hash {
            println!("{$bold}Verification:{/$} {$bold+green}Checked sha1 hash of downloaded mod, it matches!{/$}");
        } else {
            println!("{$bold}Verification:{/$} {$bold+red}Checked sha1 hash of downloaded mod, it doesn't match! cancelling transaction...{/$}");
            std::fs::remove_file(final_choice.files[0].filename.clone())?;
            return Err(Box::new(RinthaError::BadFileHash));
        }
    } else {
        println!("{$bold+intense+red}NOTE{/$}: {$bold}No \"sha1\" hash exists for this mod, this mod is UNVERIFIED but a sha1 hash will be calculated for local integrity checks...{/$}");

        let modfile = std::fs::read(final_choice.files[0].filename.clone())?;
        let sha1_hash = format!("{:x}", sha1::Sha1::digest(&modfile));
        c_sha1 = sha1_hash;
    }

    c_current_filename = final_choice.files[0].filename.clone();

    let mod_manifestation = ConfigMod {
        id: c_id,
        title: c_title,
        author_username: c_author_username,
        small_description: c_small_description,
        latest_mc_ver: c_latest_mc_ver,
        license: c_license,
        sha1: c_sha1,
        installed_version_id: c_installed_version_id,
        installed_version_number: c_installed_version_number,
        installed_version_type: c_installed_version_type,
        supported_game_versions: c_supported_game_versions,
        current_filename: c_current_filename,
    };

    // FIXME: make this respect custom paths set in config
    let mod_directory = match mod_dir() {
        Ok(dir) => dir,
        Err(err) => match err {
            RinthaError::UnsupportedPlatform => {
                println!("{$bold+red}Unsupported platform, no mod directory found...{/$}");
                std::process::exit(-1);
            }
            _ => unreachable!(), // mod_dir only returns unsupported platform on failure.
        },
    };

    {
        // yes i know this is stupid and it does a lot of allocation
        // but i don't know any other way to do it, please help (FIXME)
        let current_prof = program_config.current_profile.as_str();
        let mut edited_prof = program_config.profiles[current_prof].clone();

        match edited_prof.add_mod(mod_manifestation) {
            Ok(_) => (),
            Err(err) => {
                match err {
                    RinthaError::ModAlreadyInstalled => {
                        println!("{$bold+intense+red}Another version of this mod is already installed!{/$}");
                        println!("{$bold}Please check `rintha list`.{/$}");
                        std::process::exit(-1);
                    }
                    _ => unreachable!(), // add_mod only returns ModAlreadyInstalled on error
                }
            }
        };

        program_config
            .profiles
            .insert(current_prof.into(), edited_prof);
    }

    Ok(())
}
