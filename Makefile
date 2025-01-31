SHELL := /bin/bash

install-diesel-cli:
	cargo install diesel_cli --no-default-features --features postgres

diesel-setup:
	diesel setup

migrations-up:
	diesel migration run

migrations-down:
	diesel migration redo

format: 
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

run: 
	cargo run