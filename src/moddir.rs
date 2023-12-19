use eyre::{Report, Result};
use serde_json::Value;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;

use crate::Translation;

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
}

impl ModDirectory {
    pub fn new(directory: &str) -> Result<Self> {
        let modpath = PathBuf::from(directory).canonicalize()?;
        let components = modpath.components();
        let lastbits: PathBuf = components.clone().rev().take(1).collect();
        let name = lastbits.display().to_string();

        Ok(Self {
            config_path: None,
            modpath,
            name,
            translations: None,
        })
    }

    pub fn translation_files(&mut self) -> Result<HashMap<String, Translation>> {
        let search_dir: PathBuf = [
            self.modpath.clone(),
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

    pub fn name(&self) -> &str {
        self.name.as_str()
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
            self.modpath.as_os_str(),
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
            self.modpath.as_os_str(),
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
