# Workspace settings
WORKSPACE_DIR := .
CLI_DIR := $(WORKSPACE_DIR)/cli
WEB_DIR := $(WORKSPACE_DIR)/web
CONFIG_FILE := config.json
CARGO := cargo

.PHONY: build
build: ## Build the CLI and Web crates
	@echo "Building CLI and Web crates"
	$(CARGO) build --all

.PHONY: cli
cli: ## Run the CLI crate with cargo run and the --config flag
	@echo "Running CLI crate"
	$(CARGO) run -p veno-cli -- --config $(CONFIG_FILE)

.PHONY: web
web: ## Run the Web crate with cargo run and the --config flag
	@echo "Running Web crate"
	$(CARGO) run -p veno-web -- --config $(CONFIG_FILE)

.PHONY: help
help: ## Display this help message
	@echo "Available targets:"
	@awk 'BEGIN {FS = ":.*## "}; /^[a-zA-Z_-]+:.*## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)
