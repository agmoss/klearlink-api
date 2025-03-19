SHELL := /bin/bash

# Colors
GREEN := \033[1;32m
BLUE := \033[1;34m
NC := \033[0m

# Function to print command name before execution and completion after
define PRINT_CMD
	@echo -e "$(GREEN)>>> Running $@$(NC)"
endef

define PRINT_DONE
	@echo -e "$(BLUE)>>> Done: $@$(NC)\n"
endef

# Diesel CLI
install-diesel-cli:
	$(PRINT_CMD)
	cargo install diesel_cli --no-default-features --features postgres
	$(PRINT_DONE)

diesel-cli:
	$(PRINT_CMD)
	curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
	$(PRINT_DONE)

diesel-setup:
	$(PRINT_CMD)
	diesel setup
	$(PRINT_DONE)

# Database Migrations
migrate-up:
	$(PRINT_CMD)
	diesel migration run
	$(PRINT_DONE)

migrate-down:
	$(PRINT_CMD)
	diesel migration redo
	$(PRINT_DONE)

create-migration:
	$(PRINT_CMD)
	diesel migration generate seed-user
	$(PRINT_DONE)

# Code Quality
format: 
	$(PRINT_CMD)
	cargo fmt
	$(PRINT_DONE)

lint:
	$(PRINT_CMD)
	cargo clippy
	$(PRINT_DONE)

test:
	$(PRINT_CMD)
	cargo test -- --test-threads=1
	$(PRINT_DONE)

# SQL Linter
install-sql-linter:
	$(PRINT_CMD)
	curl -fsSL https://raw.githubusercontent.com/quarylabs/sqruff/main/install.sh | bash
	$(PRINT_DONE)

lint-sql:
	$(PRINT_CMD)
	sqruff lint migrations
	$(PRINT_DONE)

lint-sql-fix:
	$(PRINT_CMD)
	sqruff fix migrations
	$(PRINT_DONE)

# Run the application
run: 
	$(PRINT_CMD)
	cargo run
	$(PRINT_DONE)

.PHONY: install-diesel-cli diesel-cli diesel-setup migrate-up migrate-down format lint test install-sql-linter lint-sql lint-sql-fix run
