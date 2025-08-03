# Directories and files

all: build

.SILENT:

# Build everything
build: wasm frontend

# Build WebAssembly
wasm:
	echo "Building WebAssembly..."
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/web_fourier.wasm docs/web_fourier.wasm

# Compile TypeScript
frontend:
	echo "Loading vendor dependencies..."
	cp vendor/rust_canvas/dist/types/* typescript/
	cp vendor/rust_canvas/dist/*.js docs/

	echo "Compiling TypeScript..."
	tsc --project typescript/tsconfig.json

# Clean build artifacts
clean:
	echo "Cleaning build artifacts..."
	rm -rf target/
	rm -rf docs/*.js
	rm -f typescript/project.tsbuildinfo
	rm -rf typescript/*.d.ts

# Check Rust code
check:
	echo "Checking Rust code..."
	cargo check


# Serve locally for development
serve: build
	echo "Starting HTTP server on port 8000..."
	python3 -m http.server 8000 --directory docs

