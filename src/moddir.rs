use eyre::{Report, Result};
use once_cell::sync::Lazy;
use serde_json::Value;
use walkdir::WalkDir;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

use crate::Translation;

pub static RIPGREP: Lazy<String> = Lazy::new(|| {
    if let Ok(_) = Command::new("rg").arg("--version").output() {
        "rg".to_string()
    } else if let Ok(_) = Command::new("rg.exe").arg("--version").output() {
        "rg.exe".to_string()
    } else {
        "".to_string()
    }
});

#[derive(Debug, Clone)]
pub struct ModDirectory {
    /// Path to the mod directory.
    modpath: PathBuf,
    /// path to the config.json file
    config_path: Option<PathBuf>,
    /// The name of the mod, guessed, for file construction.
    name: String,
    /// Hashmap of language => filename, because modname is not predictable
    translations: Option<HashMap<String, Translation>>,
    /// The discovered data directory for this mod tree.
    datadir: PathBuf,
}

impl ModDirectory {
    pub fn new(directory: &str) -> Result<Self> {
        let modpath = PathBuf::from(directory).canonicalize()?;
        let components = modpath.components();
        let lastbits: PathBuf = components.clone().rev().take(1).collect();
        let name = lastbits.display().to_string();

        let datadir = find_data_dir(&modpath).expect(&format!(
            "{} does not contain a valid MCM Helper-using mod.",
            modpath.display()
        ));

        Ok(Self {
            config_path: None,
            modpath,
            name,
            translations: None,
            datadir,
        })
    }

    pub fn translation_files(&mut self) -> Result<HashMap<String, Translation>> {
        let search_dir: PathBuf = [
            self.datadir.clone(),
            PathBuf::from("Interface"),
            PathBuf::from("Translations"),
        ]
        .iter()
        .collect();
        if !search_dir.exists() {
            return Ok(HashMap::new());
        }

        let files: Vec<PathBuf> = std::fs::read_dir(search_dir)?
            .filter_map(|xs| {
                let Ok(entry) = xs else {
                    return None;
                };
                if entry.path().is_dir() {
                    return None;
                }
                if let Some(extension) = entry.path().extension() {
                    if extension.to_str() == Some("txt") {
                        Some(entry.path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        let mut mapping = HashMap::new();
        for file in files {
            let Some(fname) = file.file_name() else {
                continue;
            };
            let annoying = fname.to_string_lossy().replace(".txt", "");
            let Some(pieces) = annoying.split_once('_') else {
                continue;
            };
            let language = pieces.1;
            let translation = Translation::new(file, language);
            mapping.insert(pieces.1.to_owned(), translation);
        }
        Ok(mapping)
    }

    pub fn relevant_jsons(&mut self) -> Result<Vec<PathBuf>> {
        todo!()
    }

    pub fn all_needed_translations(&mut self) -> Result<Vec<String>> {
        let mut search_list = self.find_i4_jsons()?;
        if let Some(config) = self.find_config()? {
            search_list.push(config);
        };

        let mut requested: Vec<String> = search_list
            .iter()
            .filter_map(|jpath| {
                let file = File::open(jpath).ok()?;
                let rdr = std::io::BufReader::new(file);
                let cfgjson: serde_json::Value = serde_json::from_reader(rdr).ok()?;
                let requested = collect_translation_keys(&cfgjson);
                Some(requested)
            })
            .flatten()
            .collect();

        requested.sort();
        Ok(requested)
    }

    /// Search for the potentially unused tags in a source directory, skipping jsons.
    /// If ripgrep isn't present (as either rg or rg.exe) this doesn't filter but
    /// also does not trigger errors.
    pub fn ripgrep_search(&mut self, lookfor: Vec<String>) -> Vec<String> {
        // Test for ripgrep first.
        if RIPGREP.is_empty() {
            return lookfor;
        }

        lookfor
            .iter()
            .filter(|xs| {
                let escaped = xs.replace('$', "\\$");
                let mut cmd = Command::new(RIPGREP.as_str());
                cmd.arg("--quiet")
                    .arg("-Tjson")
                    .arg("--glob")
                    .arg("!Translations");
                cmd.arg(escaped);
                cmd.arg(self.modpath.to_string_lossy().to_string());
                let Ok(status) = cmd.status() else {
                    return true;
                };
                // rg exits with non-zero status code if the search fails
                !status.success()
            })
            .cloned()
            .collect()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn languages(&mut self) -> Result<Vec<String>> {
        let mapping = self.translation_files()?;
        let mut result: Vec<String> = mapping.keys().cloned().collect();
        result.sort();
        Ok(result)
    }

    pub fn provided_translations_for(&mut self, language: &str) -> Result<Vec<String>> {
        let lang = language.to_string().to_lowercase();
        let Some(mut trfile) = self.translation_file_for(lang.as_str())? else {
            return Ok(Vec::new());
        };

        trfile.provided_translations()
    }

    pub fn translation_file_for(&mut self, language: &str) -> Result<Option<Translation>> {
        if self.translations.is_none() {
            let found = self.translation_files()?;
            self.translations = Some(found);
        }
        Ok(self.translations.as_ref().unwrap().get(language).cloned())
    }

    pub fn find_config(&mut self) -> Result<Option<PathBuf>, Report> {
        let search_dir: PathBuf = [
            self.datadir.as_os_str(),
            OsStr::new("mcm"),
            OsStr::new("config"),
        ]
        .iter()
        .collect();
        if !search_dir.exists() {
            return Ok(None);
        }

        let files: Vec<PathBuf> = std::fs::read_dir(search_dir)?
            .filter_map(|xs| {
                let Ok(entry) = xs else {
                    return None;
                };
                if entry.path().is_dir() {
                    let files: Vec<PathBuf> = std::fs::read_dir(entry.path())
                        .ok()?
                        .filter_map(|xs| {
                            let Ok(entry) = xs else {
                                return None;
                            };
                            if entry.file_name().to_str() == Some("config.json") {
                                Some(entry.path())
                            } else {
                                None
                            }
                        })
                        .collect();
                    Some(files)
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        let cfg = files.first().map(|xs| (*xs.clone()).to_path_buf());
        self.config_path = cfg.clone();
        Ok(cfg)
    }

    /// Find all inventory injector files for this mod.
    pub fn find_i4_jsons(&mut self) -> Result<Vec<PathBuf>, Report> {
        let search_dir: PathBuf = [
            self.datadir.as_os_str(),
            OsStr::new("SKSE"),
            OsStr::new("plugins"),
            OsStr::new("InventoryInjector"),
        ]
        .iter()
        .collect();
        if !search_dir.exists() {
            return Ok(Vec::new());
        }

        let files: Vec<PathBuf> = std::fs::read_dir(search_dir)?
            .filter_map(|xs| {
                let Ok(entry) = xs else {
                    return None;
                };
                if let Some(extension) = entry.path().extension() {
                    if extension.to_str() == Some("json") {
                        Some(entry.path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        Ok(files)
    }
}

fn collect_translation_keys(value: &serde_json::Value) -> Vec<String> {
    match value {
        serde_json::Value::Object(m) => keys_from_mapping(m),
        serde_json::Value::Array(m) => keys_from_array(m),
        _ => Vec::new(),
    }
}

fn keys_from_mapping(mapping: &serde_json::Map<String, Value>) -> Vec<String> {
    mapping
        .iter()
        .filter_map(|(_k, v): (&String, &Value)| match v {
            Value::String(value) => {
                if value.starts_with('$') {
                    Some(vec![value.clone()])
                } else {
                    None
                }
            }
            Value::Array(arr) => Some(keys_from_array(arr)),
            Value::Object(obj) => Some(keys_from_mapping(obj)),
            _ => None,
        })
        .collect::<Vec<Vec<String>>>()
        .iter()
        .flatten()
        .map(|xs| xs.trim().to_owned())
        .collect::<Vec<String>>()
}

fn keys_from_array(arr: &[Value]) -> Vec<String> {
    arr.iter()
        .filter_map(|v| match v {
            Value::String(value) => {
                if value.starts_with('$') {
                    Some(vec![value.clone()])
                } else {
                    None
                }
            }
            Value::Array(arr) => Some(keys_from_array(arr)),
            Value::Object(obj) => Some(keys_from_mapping(obj)),
            _ => None,
        })
        .collect::<Vec<Vec<String>>>()
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<String>>()
}

/// Directories to skip.
const IGNORE_DIRS: [&str; 3] = ["target", "build", "extern"];

/// Filter a list of immediate subdirectories of a given directory for only
/// directories relevant for considering as potential data dirs.
fn find_relevant_dirs(path: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| {
            let basename = e
                .path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();
            if basename.starts_with(".") {
                None
            } else if IGNORE_DIRS.contains(&basename.as_str()) {
                None
            } else {
                Some(e.path().to_path_buf())
            }
        })
        .collect()
}

/// Look for the two required mcm-helper subdirs in a list of subdirectories,
/// returning true if they're found.
fn is_data_dir(dirs: &[PathBuf]) -> bool {
    if dirs.iter().any(|e| {
        e.file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_lowercase()
            == "interface"
    }) {
        true
    } else {
        false
    }
}

/// Find a subdirectory of moddir that has both "interface" and "translations"
/// as subdirectories (names case-insensitive). This is our starting point for
/// finding config.json. If we don't find one, this mod directory is not valid
/// for us, because we need mcm config and translation files to do our work.
fn find_data_dir(top: &PathBuf) -> Option<PathBuf> {
    let relevant = find_relevant_dirs(top);
    if is_data_dir(relevant.as_slice()) {
        Some(top.clone())
    } else {
        for entry in relevant {
            if let Some(found) = find_data_dir(&entry) {
                return Some(found);
            }
        }
        None
    }
}
