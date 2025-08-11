# Web Fourier

An interactive Fourier analysis and visualization tool that runs in the browser, built with Rust and WebAssembly. This project is an excuse for me to learn more about WebAssembly and Rust! 🦀

## Features

- **Interactive Signal Analysis**: Explore predefined signals (step, sine, square, triangle) with real-time Fourier transforms
- **Frequency Filtering**: Adjust frequency ranges to see how filtering affects the original signal
- **Animated Visualizations**: Watch mathematical patterns come to life:
  - Epitrochoids
  - Rose curves
  - Lissajous curves
  - Spirographs
- **Real-time Spectrum Analysis**: View power spectra of signals and animations

## Demo

Open [pages](https://guilmont.github.io/web_fourier/) in your browser, or run `make serve` to start a local development server.

## Building

### Prerequisites

- Rust with `wasm32-unknown-unknown` target
- TypeScript compiler (`tsc`)
- Python 3 with HTTP.Server (for local development server)

### Quick Start

```bash
# Build everything (Rust → WASM + TypeScript)
make build

# Serve locally at http://localhost:8000
make serve
```

### Development Workflow

```bash
# Build only the WASM module
make wasm

# Build only the frontend (TypeScript)
make frontend

# Check Rust code without building
make check

# Clean build artifacts
make clean
```

## Architecture

- **Rust (`src/`)**: Core Fourier transform math, plotting logic, and animation engine
- **TypeScript (`typescript/`)**: UI controls and WASM integration
- **WebAssembly**: Bridge between Rust and browser
- **Canvas API**: 2D graphics rendering via the [web_canvas](https://github.com/guilmont/rust_canvas) library

## Project Structure

```
├── src/           # Rust source code
├── typescript/    # TypeScript frontend code
├── docs/          # Built web app (HTML, JS, WASM)
├── vendor/        # Dependencies (web_canvas library)
└── Makefile       # Build automation
```

## How It Works

1. Mathematical functions are implemented in Rust for performance
2. Rust code is compiled to WebAssembly
3. TypeScript handles UI interactions and calls WASM functions
4. All graphics are rendered using a custom canvas abstraction layer

## Learning Goals

This project explores:
- Rust ↔ TypeScript interop via WebAssembly
- High-performance mathematical computing in the browser
- Real-time graphics and animation
- Fourier analysis and signal processing

## License

See [LICENSE](LICENSE) for details.
