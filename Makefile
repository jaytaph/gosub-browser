.SILENT:

#SHELL=/usr/bin/env bash -O globstar

all: help

test: test_commands test_unit test_clippy test_fmt ## Runs tests

bench: ## Benchmark the project
	cargo bench

build: ## Build the project
	source test-utils.sh ;\
	section "Cargo build" ;\
	cargo build

fix: ## Fix formatting and clippy errors (deprecated)
	echo "Use 'make format' instead"

format:  ## Fix formatting and clippy errors
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged

INCLUDE_DIR := gosub-bindings/include
SRC_DIR := gosub-bindings/tests
CPPFLAGS := -I$(INCLUDE_DIR)
CFLAGS := -std=c99 -g -Wall -Wextra
LDFLAGS := -Ltarget/debug
LDLIBS := -lgosub_bindings
CC := gcc
bindings: ## build the gosub-bindings library and compile C code tests
	# cargo build --package gosub-bindings
	$(CC) $(SRC_DIR)/render_tree_test.c -o $(SRC_DIR)/render_tree_test $(CPPFLAGS) $(LDLIBS) $(LDFLAGS) $(CFLAGS)
	./$(SRC_DIR)/render_tree_test

test_unit:
	source test-utils.sh ;\
	section "Cargo test" ;\
	cargo test --features debug_parser

test_clippy:
	source test-utils.sh ;\
	section "Cargo clippy" ;\
	cargo clippy -- -D warnings

test_fmt:
	source test-utils.sh ;\
	section "Cargo fmt" ;\
	cargo fmt -- --check

test_commands:
	cargo run --bin html5-parser-test >/dev/null
	cargo run --bin parser-test >/dev/null
	cargo run --bin config-store list >/dev/null

help: ## Display available commands
	echo "Available make commands:"
	echo
	grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
