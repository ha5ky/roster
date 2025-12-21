# Project variables
APP_NAME := roster
CARGO ?= cargo
BUILD_DIR := target

# Default target
.PHONY: all
all: build

# Development
.PHONY: run
run:
	$(CARGO) run

.PHONY: check
check:
	$(CARGO) check

.PHONY: fmt
fmt:
	$(CARGO) fmt

.PHONY: test
test:
	$(CARGO) test

.PHONY: clean
clean:
	$(CARGO) clean

# Build parameters
# Default to host architecture
ARCH ?= $(shell uname -m)
OS ?= $(shell uname -s | tr '[:upper:]' '[:lower:]')

# Map OS/ARCH to Rust target triples
# Note: To cross-compile, you may need to install targets via `rustup target add <target>`
# or use `cross` (cargo install cross) and run `make build CARGO=cross ...`

ifeq ($(OS),linux)
    ifeq ($(ARCH),x86_64)
        TARGET := x86_64-unknown-linux-gnu
    else ifeq ($(ARCH),amd64)
        TARGET := x86_64-unknown-linux-gnu
    else ifeq ($(ARCH),aarch64)
        TARGET := aarch64-unknown-linux-gnu
    else ifeq ($(ARCH),arm64)
        TARGET := aarch64-unknown-linux-gnu
    endif
else ifeq ($(OS),darwin)
    ifeq ($(ARCH),x86_64)
        TARGET := x86_64-apple-darwin
    else ifeq ($(ARCH),amd64)
        TARGET := x86_64-apple-darwin
    else ifeq ($(ARCH),arm64)
        TARGET := aarch64-apple-darwin
    endif
else ifeq ($(OS),windows)
    ifeq ($(ARCH),x86_64)
        TARGET := x86_64-pc-windows-msvc
    else ifeq ($(ARCH),amd64)
        TARGET := x86_64-pc-windows-msvc
    endif
endif

# If TARGET is defined and different from host, use it
ifdef TARGET
    BUILD_FLAGS := --target $(TARGET)
    BINARY_PATH := $(BUILD_DIR)/$(TARGET)/release/$(APP_NAME)
else
    BUILD_FLAGS :=
    BINARY_PATH := $(BUILD_DIR)/release/$(APP_NAME)
endif

.PHONY: build
build:
	@echo "Building for OS=$(OS) ARCH=$(ARCH) TARGET=$(TARGET)..."
	$(CARGO) build --release $(BUILD_FLAGS)

# Cross compilation shortcuts
.PHONY: build-linux-amd64
build-linux-amd64:
	$(MAKE) build OS=linux ARCH=amd64 CARGO=cargo-zigbuild

.PHONY: build-linux-arm64
build-linux-arm64:
	$(MAKE) build OS=linux ARCH=arm64

.PHONY: build-mac-amd64
build-mac-amd64:
	$(MAKE) build OS=darwin ARCH=amd64

.PHONY: build-mac-arm64
build-mac-arm64:
	$(MAKE) build OS=darwin ARCH=arm64

.PHONY: build-win-amd64
build-win-amd64:
	$(MAKE) build OS=windows ARCH=amd64

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make run                  - Run the application locally"
	@echo "  make build                - Build release binary for host"
	@echo "  make check                - Run cargo check"
	@echo "  make clean                - Clean build artifacts"
	@echo ""
	@echo "Cross compilation examples (requires rustup target add <target> or use CARGO=cross):"
	@echo "  make build OS=linux ARCH=amd64    - Build for Linux AMD64"
	@echo "  make build-linux-amd64            - Shortcut for Linux AMD64"
	@echo "  make build-mac-arm64              - Shortcut for macOS ARM64"
	@echo ""
	@echo "To use 'cross' for easier cross-compilation:"
	@echo "  make build-linux-amd64 CARGO=cross"
