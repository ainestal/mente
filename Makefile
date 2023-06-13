.PHONY: run release

run:
	@echo "Running..."
	@cargo run

release:
	@echo "Building release..."
	@cargo build --release
	@echo "Running release..."
	@./target/release/mente