binary := "kev-push"

alias docs := doc
alias fmt := format
alias rls := release
alias bld := build
alias sign := codesign

no_browser := ""

# list the available recipes
@default:
  just --list

# generate docs for the crate in a top-level "docs/" dir; pass "--open" to open a browser to the built docs
@doc open=no_browser:
	mkdir -p docs
	cargo doc {{open}} --no-deps --target-dir docs/

# debug build
@build:
	cargo build

# run (as release)
@run:
	cargo run --release

# release build
@release:
	cargo build --release

# Recipe requirements:
# - rustup toolchain install nightly --allow-downgrade -c rustfmt
#
# auto-format code
@format:
	rustup run nightly cargo fmt

# Recipe requirements:
# - cargo install clippy
# - rustup toolchain install nightly --allow-downgrade -c rustfmt
#
# check code format, clippy and tests
@test:
	rustup run nightly cargo fmt -- --check && \
	cargo clippy -- -Dwarnings && \
		cargo test  

# remove generated artifacts local to crate directory
@clean:
	cargo clean

# Recipe requirements:
# - graphviz (https://graphviz.org/download/)
# - cargo install cyclonedx
# 
# generates crate dependency graph and SBOM.
@deps:
	cargo deps | dot -Tsvg > assets/graph.svg
	cargo cyclonedx

# Recipe requirements:
# - Base R installation
# 
# add dark/light mode support to assets/graph.svg
@svg:
	extras/dot-embed-stylesheet.R

# Recipe requirements:
# - rustup add target aarch64-apple-darwin 
# - rustup add target x86_64-apple-darwin
# - codesign
# - Apple developer certificate identifier in "APPLE_DEV_ID" env var
#
# build a universal macOS binary in ~/bin and codesign it
@codesign:
	mkdir -p "${HOME}/bin" && \
		cargo build --release --target=aarch64-apple-darwin && \
		cargo build --release --target=x86_64-apple-darwin && \
		lipo -create -output "${HOME}/bin/{{binary}}" target/aarch64-apple-darwin/release/{{binary}} target/x86_64-apple-darwin/release/{{binary}} && \
		codesign --force --verify --verbose --sign "${APPLE_DEV_ID}" "${HOME}/bin/{{binary}}"
