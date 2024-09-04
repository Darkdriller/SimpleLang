# Makefile

# Extract the version from Cargo.toml
VERSION=$(shell grep '^version =' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

# Detect the operating system
UNAME_S := $(shell uname -s)
ifeq ($(OS),Windows_NT)
	OS_NAME := windows
else
	ifeq ($(UNAME_S),Darwin)
		OS_NAME := macos
	else
		OS_NAME := linux
	endif
endif

# The name of the zip file, which includes the version number and OS name
ZIP_NAME=compiler-v-$(VERSION)-$(OS_NAME).zip

# Default target: build the project and create the zip
all: build zip version	

# Build the project using cargo
build:
	@echo "Building the project..."
	cargo build --release

# Create a zip file of the target/release folder with the version number and OS name
zip:
	@echo "Creating zip file..."
	zip -r $(ZIP_NAME) target/release/* Cargo.toml README.md

# Clean the build directory
clean:
	@echo "Cleaning up..."
	cargo clean
	rm -rf $(ZIP_NAME)

# Display the version and OS name
version:
	@echo "Project version: $(VERSION)"
	@echo "Operating System: $(OS_NAME)"

