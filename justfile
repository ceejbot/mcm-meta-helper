set windows-shell := ["pwsh.exe", "-Command"]
set shell := ["bash", "-uc"]

TOOL_NAME := "mcm-meta-helper"

# List available recipes.
_help:
	@just -l

# Build for release.
release:
	cargo build --release

# Run cargo install for this tool
install:
	cargo install --path .

# Install required tools
setup:
	cargo install cargo-nextest tomato-toml

# Run tests.
test:
	cargo nextest run

# Clippy has opinions. Find out what they are.
@lint:
	cargo clippy --all-targets --no-deps --fix --allow-dirty
	cargo +nightly fmt

# Set the crate version and tag the repo to match. Requires bash.
tag VERSION:
    #!/usr/bin/env bash
    set -e
    tomato set package.version {{VERSION}} Cargo.toml
    cargo check
    git commit Cargo.toml Cargo.lock -m "{{VERSION}}"
    git tag "{{VERSION}}"
    echo "Release tagged for version {{VERSION}}"

# Build a mod archive for the Nexus.
[unix]
archive:
    #!/usr/bin/env bash
    set -e
    version=$(tomato get package.version Cargo.toml)
    release_name={{TOOL_NAME}}_v${version}
    mkdir -p "releases/$release_name"
    cp -rp root/* "releases/${release_name}/"
    cp -p target/release/{{TOOL_NAME}}.exe "releases/${release_name}/"
    cp -p target/release/{{TOOL_NAME}}.pdb "releases/${release_name}/"
    cd releases
    rm -f "$release_name".7z
    7z a "$release_name".7z "$release_name"
    rm -rf "$release_name"
    cd ..
    echo "Mod archive for v${version} ready at releases/${release_name}.7z"

# Remind you to run this in WSL.
[windows]
@archive:
	write-host "You need to run this in WSL to get bash."
