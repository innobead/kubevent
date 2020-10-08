.PHONY: $(MAKECMDGOALS)

PROJECT:=$(shell basename $(CURDIR))
COMMIT:=$(shell git rev-parse --short HEAD)-$(shell date "+%Y%m%d%H%M%S")
TAG:=$(shell git describe --tags --dirty)
BUILD_CACHE_DIR:=$(CURDIR)/.cache

help:
	@grep -E '^[a-zA-Z%_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

build: fmt ## Build binaries
	cargo build ${CARGO_OPTS}

fmt:## Format & Lint codes
	rustup component add rustfmt clippy
	cargo fmt
	cargo clippy

release: ## Release binaries
	CARGO_OPTS="--release" $(MAKE) build

install: ## Install binaries
	cargo install ${CARGO_OPTS}

clean: ## Clean build caches
	cargo clean
	rm -rf $(BUILD_CACHE_DIR)
