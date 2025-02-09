SHELL := /bin/bash


diesel-cli:
	curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh

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
	cargo test -- --test-threads=1

install-sql-linter:
	curl -fsSL https://raw.githubusercontent.com/quarylabs/sqruff/main/install.sh | bash

lint-sql:
	sqruff lint migrations

lint-sql-fix:
	sqruff fix migrations

run: 
	cargo run
