.DEFAULT_GOAL := help

SHELL := bash

##@ Build

.PHONY: clean
clean: ## Clean project
	cargo clean

##@ Dependencies

.PHONY: check-deps
check-deps: ## Check unused dependencies
	@cargo shear

.PHONY: fix-deps
fix-deps: ## Check and remove unused dependencies
	@cargo shear --fix

##@ Lint & Format

.PHONY: check
check: ## Cargo check all the targets
	cargo check --workspace --all-targets --all-features

.PHONY: clippy
clippy: ## Check clippy rules
	cargo clippy --workspace --all-targets --all-features -- -D warnings

.PHONY: clippy-fix
clippy-fix: ## Fix clippy violations
	cargo clippy --workspace --all-targets --all-features --fix

.PHONY: fmt
fmt: ## Format all the Rust code
	cargo +nightly fmt --all

.PHONY: fmt-check
fmt-check: ## Check code format
	cargo +nightly fmt --all -- --check

.PHONY: fmt-toml
fmt-toml: ## Format all TOML files
	taplo format

.PHONY: check-toml
check-toml: ## Check all TOML files
	taplo format --check

##@ General

.PHONY: help
help: ## Display help messages
	@./.make/help "$(MAKEFILE_LIST)"
