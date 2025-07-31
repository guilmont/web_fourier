# Copilot Instructions for web_fourier

## 1. Build, Lint, and Test Commands
- **Build all:** `make build`
- **Build WASM only:** `make wasm`
- **Build TypeScript only:** `make typescript`
- **Lint (Rust):** `cargo check`
- **Test (Rust):** `cargo test`
- **Run a single Rust test:** `cargo test <test_name>`
- **Clean:** `make clean`
- **Serve frontend:** `make serve`

## 2. Architecture & Codebase Structure
- **Rust backend (src/):** Core logic, math, animation, plotting, and WASM bindings.
- **Frontend (frontend/):** Static files (HTML, CSS, JS, WASM) for browser UI.
- **TypeScript (ts/):** TypeScript sources and config for frontend logic.
- **Build output:** WASM is built to `target/wasm32-unknown-unknown/release/` and copied to `frontend/`. `frontend/` also contains the compiled TypeScript and static assets.
- **No database**; all computation is in-browser or via WASM.
- **APIs:** Internal Rust modules (math, plotter, animation) exposed to JS via WASM.

## 3. Code Style Guidelines
- **Rust:**
  - Use `snake_case` for functions/variables, `CamelCase` for types/structs.
  - Prefer explicit types and error handling with `Result`/`Option`.
  - Organize code into modules by feature.
- **TypeScript:**
  - Use `camelCase` for variables/functions, `PascalCase` for classes.
  - Prefer ES6 imports and explicit types.
  - Use semicolons and 2-space indentation.
- **General:**
  - Keep imports ordered and minimal.
  - Document public functions and modules.
  - Avoid unused code and dead files.

