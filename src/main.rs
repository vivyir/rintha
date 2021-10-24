pub mod rintha {
    pub use femtorinth::data_structures::{ModID, Version};
    pub use femtorinth::version_list;

    #[derive(Debug, Clone)]
    pub struct ShallowSearchResult {
        pub id: ModID,
        pub title: String,
        pub author_username: String,
        pub small_description: String,
        pub downloads: usize,
        pub follows: usize,
        pub latest_mc_ver: String,
        pub license: String,
    }

    pub fn shallow_search(
        query: String,
        limit: Option<usize>,
    ) -> Result<Vec<ShallowSearchResult>, Box<dyn std::error::Error>> {
        // FIXME: shite error handling
        let slimit;
        if let Some(ok) = limit {
            slimit = Some(ok + 1);
        } else {
            slimit = Some(10 + 1);
        }

        let results = femtorinth::search_mods(query, None, slimit)?;

        let mut res: Vec<ShallowSearchResult> = vec![];
        for hit in results.hits {
            let id = hit.get_clean_id();
            let title = hit.title.clone();
            let author_username = hit.author.clone();
            let small_description = hit.description.clone();
            let downloads = hit.downloads;
            let follows = hit.follows;
            let latest_mc_ver = hit.latest_version.clone();
            let license = hit.license.clone();

            let ssr = ShallowSearchResult {
                id,
                title,
                author_username,
                small_description,
                downloads,
                follows,
                latest_mc_ver,
                license,
            };

            res.push(ssr);
        }

        Ok(res)
    }
}

use bunt::{eprintln, print, println};
use downloader::{Download, Downloader};
use sha1::Digest;
use std::{io::Write, mem};

use crate::rintha::{shallow_search, version_list};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = std::env::args().nth(1).unwrap();

    let results = shallow_search(query, None)?;
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
