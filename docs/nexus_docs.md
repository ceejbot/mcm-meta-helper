`mcm-meta-helper` is a command-line tool intended to help modders manage translation files for their MCM Helper configuration files. It also helps with Inventory Interface Information Injector (I4) json files.

I wrote it because I got tired of using some terrible `jq` and `sed` hacks to find missing translations in my mods.

## How to use it

Install it from the archive here on the Nexus, or from the [releases on GitHub](https://github.com/ceejbot/mcm-meta-helper/releases/). The releases page has additional ways to install it, using homebrew or a powershell script. Put the executable in your path somewhere and fire up a terminal with your favorite shell. Change directories to your mod, then run the tool to check. Here are the three checks it can run for you:

`mcm-meta-helper check all`: Check all languages found in the mod's `Interface/Translations/` directory for missing translations and translations that are provided but unused in the JSON files.

`mcm-meta-helper check <language>`: Run the checks for only the given language.

`mcm-meta-helper update`: Updates all translation files that are missing translations with stubs for the missing entries. These new entries are added at the end of the file.

`mcm-meta-helper copy <language>`: Copy into all other language files translations that the given language has that they're missing. This command lets you add new translations in a single language, then add them to the end of the other language files. Existing translations aren't touched.

`mcm-meta-helper validate`: Validates the mod's `config.json` file (in `mcm/config/MOD_NAME/config.json`) against the official MCM Helper schema. This often reports errors with valid and working config files, so you shouldn't use this to replace testing. The schema has possibly drifted a bit from the reality of the code.

The tool has some options to make checking your mods easier. For instance, to check a mod that isn't the current directory, pass `--moddir /path/to/mod`. Run `mcm-meta-helper <command> --help` to get help for a specific command. If a check fails, the tool exits with a non-zero status code.

You can control the verbosity of the reporting output by using `--verbose` or `-v` to make it chattier, and `--quiet` to make it quieter.

## Full source search available

If the tool finds [ripgrep](https://github.com/BurntSushi/ripgrep?tab=readme-ov-file#ripgrep-rg) available on your path, as either `rg` or `rg.exe`, it will do a full text search of all files in the mod directory for translation usage. Ripgrep is aware of `.gitingore` files. In this tool's usage, it skips json files as well as build output directories.

This feature brought the false-positive unused translation reports for my mods down to a single truly-unused translation, which I could then clean up.

## Things the tool does not do (yet)

It could fall back from `rg` to similar tools like `ack` and `ag`. It could also be more restrictive about what files it searches. If you experience regular false positives, let me know.

## Full help output

```text
$ mcm-meta-helper help
$ mcm-meta-helper help
Help manage MCM Helper translation files by checking for missing or unused translations.

Can also compare your config.json file against the MCM Helper schema to report problems,
though this is unreliable at the moment because the schema is not quite right.

Usage: mcm-meta-helper [OPTIONS] <COMMAND>

Commands:
  check     Cross-check required translation strings versus the ones found in
            translation files
  copy      Copy translations from the source language file to any language file
            missing translations
  update    Update all translation files with missing translation strings and
            placeholders
  validate  Validate the mcm config json file against the MCM helper schema
  help      Print this message or the help of the given subcommand(s)

Options:
  -m, --moddir <MODDIR>
          The mod directory containing the mod to analyze
          [default: .]

  -v, --verbose
          Print out more information as the tool runs

  -q, --quiet
          Print out only very important information

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Permissions and credits

This is a Rust project. [Source is available on GitHub](https://github.com/ceejbot/mcm-meta-helper) under the [the Parity Public License](https://paritylicense.com). This license allows you to use and share this software for free, but you have to share software that builds on it alike. In Skyrim modding language, this license supports "cathedral" modding, not "parlor" modding.
