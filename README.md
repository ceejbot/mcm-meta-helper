# mcm-meta-helper

`mcm-meta-helper` is a command-line tool for validating your MCM Helper configuration and translation files. It reports schema errors in the helper layout file `config.json` as well as missing and unused translation tags.

It will also look for any I4 json files in your mod and scan those for requested translations.

## Usage

The most common usage is to change your working directory to your mod directory, then run `mcm-meta-helper check`.

```text
$ mcm-meta-helper help
Help manage MCM Helper translation files by checking for missing or unused translations.

Can also compare your config.json file against the MCM Helper schema to report problems, though this is unreliable at the moment because the schema is not quite right.

Usage: mcm-meta-helper [OPTIONS] <COMMAND>

Commands:
  check     Cross-check required translation strings versus the ones found
            in translation files.
  update    Update all mod translation files with missing translation strings
            and placeholders.
  validate  Validate the mcm config json file against the MCM helper schema
  help      Print this message or the help of the given subcommand(s)

Options:
  -m, --moddir <MODDIR>
          The mod directory containing the mod to analyze
          [default: .]
  -v, --verbose Print out more information as the tool runs
  -q, --quiet Print out only very important information
  -h, --help Print help (see a summary with '-h')
  -V, --version Print version
```

## Hacking

This is a Rust language project. Install the Rust tools with [rustup](https://rustup.rs), then run `cargo build` to build.

## License

[The Parity Public License.](https://paritylicense.com) This license requires people who build with this software to share their work with the community, too. In Skyrim modding language, this license allows "cathedral" modding, not "parlor" modding.
