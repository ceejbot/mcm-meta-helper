use clap::{Parser, Subcommand};
use eyre::{Context, Report, Result};
use jsonschema::JSONSchema;
use owo_colors::OwoColorize;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::*;

use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;

mod moddir;
pub use moddir::*;
mod translation;
pub use translation::*;
mod skyui_translations;
pub use skyui_translations::*;

/// Help manage MCM Helper translation files by checking for missing or unused translations.
///
/// Can also compare your config.json file against the MCM Helper schema to report
/// problems, though this is unreliable at the moment because the schema is not quite right.
#[derive(Parser, Debug)]
#[clap(name = "mcm-meta-helper", version)]
pub struct Args {
    /// The mod directory containing the mod to analyze.
    #[clap(long, short, global = true, default_value = ".")]
    moddir: String,
    /// Print out more information as the tool runs.
    #[clap(long, short, global = true)]
    verbose: bool,
    /// Print out only very important information.
    #[clap(long, short, global = true)]
    quiet: bool,
    /// What to do.
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Cross-check required translation strings versus the ones found in translation files.
    Check {
        /// Reference language to cross-check against.
        #[clap(long, short, default_value = "English")]
        language: String,
    },
    /// Update the specified translation files with missing translation strings and placeholders. NOT YET IMPLEMENTED.
    Update,
    /// Validate the mcm config json file against the MCM helper schema
    Validate,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Check { ref language } => {
                write!(f, "check --language {language}")
            }
            Command::Update => write!(f, "update"),
            Command::Validate => write!(f, "validate"),
        }
    }
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mcm-meta-helper ")?;
        if self.verbose {
            write!(f, "--verbose")?;
        }
        if self.quiet {
            write!(f, "--quiet")?;
        }
        if self.moddir.as_str() != "." {
            write!(f, "--moddir '{}'", self.moddir)?;
        }
        write!(f, " {}", self.cmd)
    }
}

pub fn print_in_grid(items: &Vec<impl ToString>, level: log::Level) {
    let width = if let Some((Width(w), Height(_h))) = terminal_size() {
        w - 2
    } else {
        72
    };

    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(2),
        direction: Direction::LeftToRight,
    });
    for item in items {
        grid.add(Cell::from(item.to_string()));
    }

    if let Some(g) = grid.fit_into_width(width.into()) {
        log::log!(level, "{}", g);
    } else {
        log::log!(level, "{}", grid.fit_into_columns(2));
    }
}

fn check(args: &Args, language: &str) -> Result<(), Report> {
    let mut moddir = ModDirectory::new(args.moddir.as_str())?;

    log::info!("\nChecking strings in {}...", moddir.name().bold().blue());
    let provided = moddir
        .provided_translations_for(language)
        .context(format!("Finding all translations into {language}"))?;
    log::debug!(
        "    {} {} translations found.",
        provided.len(),
        language.bold().blue()
    );
    let requested = moddir
        .all_needed_translations()
        .context("Finding all requested translations")?;
    log::debug!("    {} translations needed.", requested.len());

    let provided_set: HashSet<String> = HashSet::from_iter(provided.iter().map(|xs| xs.to_owned()));
    let requested_set: HashSet<String> =
        HashSet::from_iter(requested.iter().map(|xs| xs.to_owned()));

    let winnowed = requested_set.difference(&SKYUI_KEYS);
    let winnowed: HashSet<String> = winnowed.cloned().collect();
    let missing = winnowed.difference(&provided_set);

    let mut mvec: Vec<&String> = missing.collect();
    mvec.sort();

    let unused = provided_set.difference(&requested_set);
    let mut uvec: Vec<&String> = unused.collect();
    uvec.sort();
    if mvec.is_empty() && uvec.is_empty() {
        log::warn!("    No translation problems found. ✨");
    }
    if !mvec.is_empty() {
        if mvec.len() == 1 {
            log::warn!("    1 translation is {}!\n", "missing".bold().red());
        } else {
            log::warn!(
                "    {} translations are {}!\n",
                mvec.len(),
                "missing".bold().red()
            );
        }
        print_in_grid(&mvec, log::Level::Info);
    }
    if !uvec.is_empty() {
        if uvec.len() == 1 {
            log::warn!("    1 translation is {}.\n", "unused".bold().yellow());
        } else {
            log::warn!(
                "    {} translations are {}.\n",
                uvec.len(),
                "unused".bold().yellow()
            );
        }
        print_in_grid(&uvec, log::Level::Debug);
    }

    Ok(())
}

fn update(args: &Args) -> Result<(), Report> {
    let mut moddir = ModDirectory::new(args.moddir.as_str())?;

    let requested = moddir
        .all_needed_translations()
        .context("Finding all requested translations")?;
    let requested_set: HashSet<String> =
        HashSet::from_iter(requested.iter().map(|xs| xs.to_owned()));

    let trfiles = moddir.translation_files()?;
    let padding = trfiles.iter().fold(30, |acc, (_lang, trfile)| {
        let max = usize::max(acc, trfile.display().len());
        max
    });

    for (language, mut trfile) in trfiles {
        let provided = moddir
            .provided_translations_for(language.as_str())
            .context(format!("Finding all translations into {language}"))?;
        let provided_set: HashSet<String> =
            HashSet::from_iter(provided.iter().map(|xs| xs.to_owned()));

        let winnowed = requested_set.difference(&SKYUI_KEYS);
        let winnowed: HashSet<String> = winnowed.cloned().collect();
        let missing = winnowed.difference(&provided_set);

        let mut mvec: Vec<&String> = missing.collect();
        if mvec.is_empty() {
            log::debug!("{:>padding$}: none needed", trfile.display().bold().blue());
        } else {
            mvec.sort();
            trfile.add_stub_translation(mvec.as_slice())?;
            let prefix = if mvec.len() == 1 {
                "1 stub".to_string()
            } else {
                format!("{} stubs", mvec.len())
            };
            log::info!(
                "{:>padding$}: {} added",
                trfile.display().bold().blue(),
                prefix
            );
        }
    }

    Ok(())
}

fn validate_config(args: &Args) -> Result<(), Report> {
    // from moddir, read ./mcm/config/**/config.json
    let mut moddir = ModDirectory::new(args.moddir.as_str())?;
    let Some(fpath) = moddir.find_config()? else {
        log::info!(
            "No MCM Helper {} files found to check.",
            "config.json".blue()
        );
        return Ok(());
    };

    // last two segments of the path...
    let components = fpath.components();
    let lastbits: PathBuf = components
        .clone()
        .rev()
        .take(2)
        .collect::<Vec<std::path::Component>>()
        .iter()
        .rev()
        .collect();
    let display_name = lastbits.display().to_string();

    let schema_json: serde_json::Value =
        serde_json::from_str(include_str!("../schemas/config.schema.json"))?;
    let schema = JSONSchema::compile(&schema_json)
        .expect("the default MCM Helper schema should be valid json!");

    let file = File::open(&fpath)?;
    let rdr = std::io::BufReader::new(file);
    let config: serde_json::Value = serde_json::from_reader(rdr)?;
    if schema.is_valid(&config) {
        log::info!(
            "✅  {} is a valid MCM Helper file.",
            display_name.bold().blue(),
        );
    } else {
        log::warn!("⚠️  {} has errors!", display_name.bold().red());

        let result = schema.validate(&config);
        if let Err(errors) = result {
            for error in errors {
                log::warn!("{:?}: {}", error.kind, error);
                log::warn!("Instance path: {}\n", error.instance_path);
            }
        }
    }

    Ok(())
}

/// Process command-line options and act on them.
fn main() -> Result<(), Report> {
    let args = Args::parse();
    let level = if args.verbose {
        // Debug-level logging should be designed for modders to read when they
        // are trying to debug problems.
        log::Level::Debug
    } else if args.quiet {
        // Error- and warn-level logging should be designed to inform modders about truly important
        // problems or results.
        log::Level::Warn
    } else {
        // Info-level logging should be designed for modders to read normally.
        log::Level::Info
    };

    loggerv::Logger::new()
        .max_level(level)
        .line_numbers(false)
        .module_path(false)
        .colors(true)
        .init()
        .unwrap();

    let result = match args.cmd {
        Command::Check { ref language } => check(&args, language),
        Command::Update => update(&args),
        Command::Validate => validate_config(&args),
    };

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("mcm-meta-helper couldn't run!");
            log::error!("{e:#}");
            log::error!("The command run was:\n{}", args.bold());
            Err(e)
        }
    }
}
