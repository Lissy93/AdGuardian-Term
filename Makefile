# The location of cargo bin (e.g. /usr/bin/cargo)
CARGO_BIN = cargo
# Any options for cargo
CARGO_OPTS =
# The full command to run cargo
CARGO = $(CARGO_BIN) $(CARGO_OPTS)

# Main make command - build, test and run the app
all: clean build test doc run

# Run the program, will first compile if needed
run: build
	$(CARGO) run

# Compile the source and dependencies
build:
	$(CARGO) build --all-features

# Run the unit and rustdoc tests
test: build
	$(CARGO) test --all-features

# Execute benchmkark tests
bench: build
	$(CARGO) bench

# Generate documentation from rustdoc
doc:
	$(CARGO) doc --no-deps --all-features

# Check the source for compilation errors
check:
	$(CARGO) check --all-features

# Format all bin + lib files
fmt:
	$(CARGO) fmt -- --check

# Check for common mistakes, and improvements
clippy:
	$(CARGO) clippy -- -D warnings

# Remove the target directory and compiled files
clean:
	$(CARGO) clean

.PHONY: all run build clean check test bench doc fmt clippy
