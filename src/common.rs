use std::env::consts::OS;
use std::path::{Path, PathBuf};

pub use femtorinth::data_structures::{ModID, Version};
pub use femtorinth::version_list;
use shellexpand::tilde;

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

pub fn mod_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = tilde("~");
    let home = Path::new(home.as_ref());

    match OS {
        "macos" => Ok(home
            .join("Library")
            .join("ApplicationSupport")
            .join("minecraft")
            .join("mods")),
        "linux" => Ok(home.join(".minecraft").join("mods")),
        "windows" => Ok(home
            .join("AppData")
            .join("Roaming")
            .join(".minecraft")
            .join("mods")),
        _ => panic!("unsupported platform"), // FIXME: make into error instead of panic
    }
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
