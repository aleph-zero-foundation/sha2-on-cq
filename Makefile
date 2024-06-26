.PHONY: render_table
render_table: ## Render PLONKish table for example inputs
	cargo run --release --bin render_table

.PHONY: build
build: ## Build the project
	cargo build --release

.PHONY: test
test: ## Run the tests
	cargo test

.PHONY: lint
lint: ## Run the linter
	cargo +nightly fmt
	cargo clippy --release -- -D warnings

.PHONY: clean
clean: ## Clean all the workspace build files
	cargo clean

.PHONY: help
help: ## Displays this help
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make \033[1;36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[1;36m%-25s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)