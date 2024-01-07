`mcm-meta-helper` is a command-line tool intended to help modders manage translation files for their MCM Helper configuration files. It also helps with Inventory Interface Information Injector (I4) json files.

I wrote it because I got tired of using some terrible `jq` and `sed` hacks to find missing translations in my mods.

## How to use it

Install it from the archive here on the Nexus, or from the [releases on GitHub](https://github.com/ceejbot/mcm-meta-helper/releases/). Put it in your path somewhere and fire up a terminal with your favorite shell. Change directories to your mod, then run the tool to check. Here are the three checks it can run for you:

`mcm-meta-helper check`: Reports on missing translations or translations that are provided but unused in the JSON files.

`mcm-meta-helper update`: Adds stubs for missing translations to all detected translation files with omissions.

`mcm-meta-helper validate`: Validates the mod's `config.json` file (in `mcm/config/MOD_NAME/config.json`) against the official MCM Helper schema. This often reports errors with valid and working config files, so you shouldn't use this to replace testing. The schema has possibly drifted a bit from the reality of the code.

The tool has a number of options to make checking your mods easier. For instance, to check a mod that isn't the current directory, pass `--moddir /path/to/mod`. Run `mcm-meta-helper <command> --help` to get help for a specific command.

## Full help output

```text
Help manage MCM Helper translation files by checking for missing or unused translations.

Can also compare your config.json file against the MCM Helper schema to report problems,
though this is unreliable at the moment because the schema is not quite right.

Usage: mcm-meta-helper [OPTIONS] <COMMAND>

Commands:
  check     Cross-check required translation strings versus the ones found in
            translation files
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

This is a Rust project. [Source is available on GitHub](https://github.com/ceejbot/mcm-meta-helper) under the [the Parity Public License](https://paritylicense.com). This license allows you to use and share this software for free, but you have to share software that builds on it alike. In Skyrim modding language, this mod requires "cathedral" modding, not "parlor" modding.
