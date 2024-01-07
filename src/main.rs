use clap::{Parser, Subcommand};
use comfy_table::presets::NOTHING;
use comfy_table::*;
use eyre::{Context, Report, Result};
use jsonschema::JSONSchema;
use owo_colors::OwoColorize;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;

mod moddir;
pub use moddir::*;
mod translation;
pub use translation::*;
mod skyui_translations;
pub use skyui_translations::*;
mod formatting;
pub use formatting::*;

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
    Check { language: String },
    /// Update all translation files with missing translation strings and placeholders.
    Update,
    /// Validate the mcm config json file against the MCM helper schema
    Validate,
}

fn check(args: &Args, language: &String) -> Result<bool, Report> {
    let check_all = *language == "all".to_string();

    let mut moddir = ModDirectory::new(args.moddir.as_str())?;

    let requested = moddir
        .all_needed_translations()
        .context("Finding all requested translations")?;
    let requested_set: HashSet<String> =
        HashSet::from_iter(requested.iter().map(|xs| xs.to_owned()));

    let mut trfiles = moddir.translation_files()?;

    let report_for =
        |table: &mut Table, language: &str, trfile: &mut Translation| -> Result<bool> {
            let provided = trfile.provided_translations()?;
            let provided_set: HashSet<String> =
                HashSet::from_iter(provided.iter().map(|xs| xs.to_owned()));

            let winnowed = requested_set.difference(&SKYUI_KEYS);
            let winnowed: HashSet<String> = winnowed.cloned().collect();
            let missing = winnowed.difference(&provided_set);
            let mut missing_tags: Vec<String> = missing.cloned().collect();
            missing_tags.sort();

            let unused = provided_set.difference(&requested_set);
            let mut unused_tags: Vec<String> = unused.cloned().collect();
            unused_tags.sort();

            if missing_tags.is_empty() && unused_tags.is_empty() {
                log::debug!("{}: no problems found", language.bold().blue());
                return Ok(true);
            }

            let count_cell = if missing_tags.len() == 1 {
                Cell::new("1 missing translation found")
            } else {
                Cell::new(format!("{} missing translations found", missing_tags.len()))
            };
            table.add_row(vec![
                Cell::new(language)
                    .fg(Color::Blue)
                    .add_attribute(Attribute::Bold),
                count_cell,
            ]);

            if args.quiet {
                return Ok(missing_tags.is_empty());
            }

            #[allow(clippy::comparison_chain)]
            if missing_tags.len() == 1 {
                table.add_row(vec![
                    Cell::new("missing")
                        .fg(Color::Red)
                        .add_attribute(Attribute::Bold),
                    Cell::new(missing_tags.first().cloned().unwrap_or_default()), // this is hilarious
                ]);
            } else if missing_tags.len() > 1 {
                table.add_row(vec![
                    Cell::new("missing")
                        .fg(Color::Red)
                        .add_attribute(Attribute::Bold),
                    Cell::new(grid_string(&missing_tags, 20)),
                ]);
            }

            // Clippy wanted me to write it this way, and I think I hate it.
            match unused_tags.len().cmp(&1) {
                Ordering::Less => {}
                Ordering::Equal => {
                    table.add_row(vec![
                        Cell::new("unused")
                            .fg(Color::Yellow)
                            .add_attribute(Attribute::Bold),
                        Cell::new(unused_tags.first().cloned().unwrap_or_default()), // this is still hilarious
                    ]);
                }
                Ordering::Greater => {
                    let report_cell = if args.verbose {
                        let gridded = grid_string(&unused_tags, 20);
                        Cell::new(gridded.trim()).set_delimiter('\n')
                    } else {
                        Cell::new(format!("{} translations", unused_tags.len()))
                    };
                    table.add_row(vec![
                        Cell::new("unused")
                            .fg(Color::Yellow)
                            .add_attribute(Attribute::Bold),
                        report_cell,
                    ]);
                }
            }

            // We do not fail tests if we have unused translations.
            Ok(missing_tags.is_empty())
        };

    let mut table = Table::new();
    table
        .load_preset(NOTHING)
        .set_content_arrangement(ContentArrangement::Dynamic);

    let mut checks_passed = true;
    if check_all {
        for (language, mut trfile) in trfiles {
            checks_passed &= report_for(&mut table, language.as_str(), &mut trfile)?;
            if !args.quiet {
                table.add_row(vec![Cell::new("---"), Cell::new("")]);
            }
        }
    } else {
        let trfile = trfiles.get_mut(language.as_str()).unwrap_or_else(|| {
            panic!(
                "Can't find a translation file for language {}",
                language.bold().yellow()
            )
        });
        checks_passed = report_for(&mut table, language.as_str(), trfile)?;
    }
    log::warn!("{}", table);

    Ok(checks_passed)
}

fn update(args: &Args) -> Result<bool, Report> {
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

    for (_language, mut trfile) in trfiles {
        let provided = trfile.provided_translations()?;
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

    Ok(true)
}

fn validate_config(args: &Args) -> Result<bool, Report> {
    // from moddir, read ./mcm/config/**/config.json
    let mut moddir = ModDirectory::new(args.moddir.as_str())?;
    let Some(fpath) = moddir.find_config()? else {
        log::info!(
            "No MCM Helper {} files found to check.",
            "config.json".blue()
        );
        return Ok(false);
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
        return Ok(true);
    }

    log::warn!("⚠️  {} has errors!", display_name.bold().red());

    let result = schema.validate(&config);
    if let Err(errors) = result {
        for error in errors {
            log::warn!("{:?}: {}", error.kind, error);
            log::warn!("Instance path: {}\n", error.instance_path);
        }
    }

    Ok(false)
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
        Ok(passed) => {
            if passed {
                Ok(())
            } else {
                Err(eyre::eyre!("The checks found problems!"))
            }
        }
        Err(e) => {
            log::error!("mcm-meta-helper couldn't run!");
            log::error!("{e:#}");
            log::error!("The command run was:\n{}", args.bold());
            Err(e)
        }
    }
}
