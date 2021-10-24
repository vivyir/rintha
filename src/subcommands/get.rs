use crate::common::{shallow_search, version_list};
use bunt::{eprintln, print, println};
use downloader::{Download, Downloader};
use sha1::Digest;
use std::{io::Write, mem};

pub fn get(query: String, limit: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
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
    let choice: usize = string.trim().parse()?;

    if choice >= results.len() {
        eprintln!("{$bold}Choice was over the limit, exiting...{/$}");
        std::process::exit(-1);
    }

    println!(
        "{$bold}Getting info for \"{[yellow]}\"...{/$}\n",
        results[choice].title
    );

    let versions = version_list(results[choice].id.clone())?;
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
    let choice: usize = string.trim().parse()?;

    if choice >= versions.len() {
        eprintln!("{$bold}Choice was over the limit, exiting...{/$}");
        std::process::exit(-1);
    }

    let final_choice = versions[choice].clone();

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
        mem::drop(modfile);

        if hash.to_owned() == sha1_hash {
            println!("{$bold}Verification:{/$} {$bold+green}Checked sha1 hash of downloaded mod, it matches!{/$}");
        } else {
            println!("{$bold}Verification:{/$} {$bold+red}Checked sha1 hash of downloaded mod, it doesn't match! cancelling transaction...{/$}");
            std::fs::remove_file(final_choice.files[0].filename.clone())?;
        }
    } else {
        println!("{$bold+intense+red}NOTE{/$}: {$bold}No \"sha1\" hash exists for this mod, skipping verification...{/$}");
    }

    Ok(())
}
