use std::collections::HashMap;
use std::env::consts::OS;
use std::path::{Path, PathBuf};

pub use femtorinth::data_structures::{ModID, ModReleaseType, Version, VersionID};
pub use femtorinth::version_list;
use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct FullConfig {
    pub custom_mod_dir: Option<String>,
    pub current_profile: String,
    pub config_revision: usize,
    pub profiles: HashMap<String, Profile>,
}

impl Default for FullConfig {
    fn default() -> Self {
        FullConfig {
            custom_mod_dir: None,
            current_profile: "default".into(),
            config_revision: 0,
            profiles: HashMap::from([("default".into(), Profile::default())]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub mods: Option<Vec<ConfigMod>>,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            name: "default".into(),
            mods: None,
        }
    }
}

impl Profile {
    pub fn add_mod(&mut self, cmod: ConfigMod) -> Result<(), RinthaError> {
        let mods = &self.mods;
        if mods.is_some() {
            // its fine to unwrap because we already know it isn't None
            if self.mods.as_mut().unwrap().contains_mod(cmod.clone()) {
                Err(RinthaError::ModAlreadyInstalled)
            } else {
                self.mods.as_mut().unwrap().push(cmod);
                Ok(())
            }
        } else {
            self.mods = Some(vec![cmod]);
            Ok(())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigMod {
    pub id: ModID,
    pub title: String,
    pub author_username: String,
    pub small_description: String,
    pub latest_mc_ver: String,
    pub license: String,
    pub sha1: String,
    pub installed_version_id: VersionID,
    pub installed_version_number: String,
    pub installed_version_type: ModReleaseType,
    pub supported_game_versions: Vec<String>,
    pub current_filename: String,
}

/*
 * disabled until further notice
 *
impl Default for ConfigMod {
    fn default() -> Self {
        ConfigMod {
            id: ModID("".into()),
            title: "".into(),
            author_username: "".into(),
            small_description: " field".into(),
            latest_mc_ver: "".into(),
            license: "".into(),
            installed_version_id: VersionID("".into()),
            installed_version_number: "".into(),
            installed_version_type: ModReleaseType::Alpha,
            supported_game_versions: vec!["".into()],
            current_filename: "".into(),
        }
    }
}
 */

pub trait ModChecking {
    fn contains_mod(&self, mod_manifest: ConfigMod) -> bool;
}

impl ModChecking for Vec<ConfigMod> {
    fn contains_mod(&self, mod_manifest: ConfigMod) -> bool {
        if self.is_empty() {
            false
        } else {
            let mut res = false;
            for modif in self.iter() {
                if modif.id.0 == mod_manifest.id.0 {
                    res = true;
                }
            }
            res
        }
    }
}

#[derive(Debug, Error)]
pub enum RinthaError {
    #[error("The hash for the file did not match.")]
    BadFileHash,
    #[error("This platform isn't supported by Rintha.")]
    UnsupportedPlatform,
    #[error("This mod is already installed!")]
    ModAlreadyInstalled,
}

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

pub fn mod_dir() -> Result<PathBuf, RinthaError> {
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
        _ => Err(RinthaError::UnsupportedPlatform),
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
