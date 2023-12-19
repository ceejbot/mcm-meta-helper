# mcm-meta-helper

`mcm-meta-helper` is a command-line tool for validating your MCM Helper configuration and translation files. It reports schema errors in the helper layout file `config.json` as well as missing and unused translation tags.

It will also look for any I4 json files in your mod and scan those for requested translations.

## Usage

For full help, run `mcm-meta-helper help`, or `mcm-meta-helper <command> help` for help with a specific command.

The most common usage is to change your working directory to your mod directory, then run `mcm-meta-helper check`.

## License

[The Parity Public License.](https://paritylicense.com) This license requires people who build with this software to share their work with the community, too. In Skyrim modding language, this license allows "cathedral" modding, not "parlor" modding.
