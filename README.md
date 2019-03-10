# rq

Easy q for anything: reading lists, pending tasks... Based on [fdavidcl/q](https://github.com/fdavidcl/q)

## Dependencies

Rust

## Installation

Clone this repo and build it

	cargo build --release

Copy rq to any folder in your path like

	cp target/release/rq ~/.local/bin

Maybe you'll need to `chmod +x rq`

## Usage

Pop something from your q:

	$ rq

Add something to your q:

	$ rq learn rust
