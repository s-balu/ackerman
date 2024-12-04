# Define targets
.PHONY: all c rust clean

# Default target
all: c rust

# Compile C implementation
c: ackermann_c

ackermann_c: src/Ackermann.c
	@echo "Compiling C implementation..."
	gcc src/Ackermann.c -o ackermann_c

# Compile Rust implementation
rust: ackermann_rust

ackermann_rust: src/Ackermann.rs
	@echo "Compiling Rust implementation..."
	rustc src/Ackermann.rs -o ackermann_rust

# Clean up compiled files
clean:
	@echo "Cleaning up compiled binaries..."
	rm -f ackermann_c ackermann_rust
