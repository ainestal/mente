.PHONY: run release

run:
	@echo "Running..."
	@cargo run

talk:
	@echo "Running..."
	@cargo run --bin talk

release:
	@echo "Building release..."
	@cargo build --release
	@echo "Running release..."
	@./target/release/mente