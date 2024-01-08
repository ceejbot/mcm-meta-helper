//! A struct for translation files. Read and get information about.
//! Eventually, modify.

use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, Read, Write};
use std::path::PathBuf;

use byte_slice_cast::AsByteSlice;
use byte_slice_cast::AsMutSliceOf;
use eyre::{Context, Report, Result};

#[derive(Debug, Clone)]
pub struct Translation {
    fpath: PathBuf,
    display_name: String,
    language: String,
    translations: Option<HashMap<String, String>>,
}

impl Translation {
    pub fn new(fpath: PathBuf, lang: &str) -> Self {
        let language = lang.to_owned();
        let display_name = if let Some(fname) = fpath.file_name() {
            // we know this is the case we're executing...
            fname.to_string_lossy().to_string()
        } else {
            format!("something_{lang}.txt")
        };
        Self {
            fpath,
            display_name,
            language,
            translations: None,
        }
    }

    pub fn display(&self) -> &str {
        self.display_name.as_str()
    }

    pub fn provided_translations(&mut self) -> Result<Vec<String>> {
        let map = self.load_translations()?;
        let mut result: Vec<String> = map.keys().cloned().collect();
        result.sort();
        Ok(result)
    }

    pub fn load_translations(&mut self) -> Result<HashMap<String, String>, Report> {
        if self.translations.is_some() {
            return Ok(self.translations.as_ref().unwrap().clone());
        }

        let mut file = File::open(&self.fpath).context(format!(
            "opening the {} translation file: {}",
            self.language, self.display_name
        ))?;
        let mut bytes: Vec<u8> = Vec::new();
        let count = file.read_to_end(&mut bytes).context(format!(
            "reading the {} translation file: {}",
            self.language, self.display_name
        ))?;
        if count == 0 {
            return Ok(HashMap::new());
        }

        let widebytes = bytes.as_mut_slice_of::<u16>().context(format!(
            "decoding the {} translation file: {}",
            self.language, self.display_name
        ))?;
        let mut utf8bytes: Vec<u8> = vec![0; count];
        let _widecount = match ucs2::decode(widebytes, &mut utf8bytes) {
            Ok(c) => c,
            Err(e) => {
                log::error!("{e:?}");
                return match e {
                    ucs2::Error::BufferOverflow => Err(eyre::eyre!(
                        "Not enough space left in the output buffer to decode UCS-2 characters;\n{} might not be a valid UCS-2 file.",
                        self.display()
                    )),
                    ucs2::Error::MultiByte => Err(eyre::eyre!(
                        "Input contained a character which cannot be represented in UCS-2;\n{} might not be a valid UCS-2 file.",
                     self.display())),
                };
            }
        };

        let reader = std::io::BufReader::new(utf8bytes.as_slice()).lines();
        let mut translations: HashMap<String, String> = HashMap::new();
        for maybe_line in reader {
            let Ok(line) = maybe_line else {
                continue;
            };
            let line = line.trim().trim_matches('\0');
            if line.len() < 4 {
                continue;
            }
            let Some((key, value)) = line.split_once('\t') else {
                if !line.starts_with('-') {
                    log::trace!("Line with len={} does not contain a tab! Line:", line.len());
                    log::trace!("{line}");
                }
                continue;
            };
            translations.insert(key.trim().to_owned(), value.trim().to_owned());
        }
        Ok(translations)
    }

    pub fn add_stub_translation(&mut self, stubs: &[&String]) -> Result<()> {
        let mut lines: Vec<String> = Vec::new();
        lines.push("\n---------- new translation stubs ----------\n".to_string());
        stubs.iter().for_each(|stub| {
            lines.push(format!(
                "{}\ttranslation for {}",
                stub,
                stub.replacen('$', "", 1)
            ));
        });

        let input = lines.join("\n");
        let mut widebuf: Vec<u16> = vec![0; input.len() * 2];
        let count = match ucs2::encode(input.as_str(), &mut widebuf) {
            Ok(v) => v,
            Err(e) => return Err(Report::msg(format!("error while encoding strings: {e:?}"))),
        };
        widebuf.resize(count, 0);

        let narrow = widebuf.as_byte_slice();
        let mut options = OpenOptions::new();
        let mut file = options.append(true).open(&self.fpath)?;
        file.write_all(narrow)?;
        file.flush()?;
        Ok(())
    }
}
