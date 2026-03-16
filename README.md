# Game of Life Playground

This repository is a playground for multiple Conway's Game of Life implementations and frontends.
It is organized around comparable engines in different languages, plus small GUI, console, FFI, and WebAssembly integrations.

## Overview

- `rust/game-of-life-engine` is the shared Rust core engine.
- `rust/game-of-life-ffi` builds `game_of_life_ffi.dll`, which is consumed by the C# engine project.
- `rust/game-of-life-wasm` builds the WebAssembly package used by the JavaScript site under `js/`.
- Rust provides a GUI app (`game-of-life-pixel`) and a console app (`game-of-life-console`).
- C# provides a GUI app (`GameOfLifePixel`) and a console app (`GameOfLifeConsole`).
- C++ currently provides an engine library (`GameOfLifeEngine`) and a console app (`GameOfLifeConsole`).
- `_benchmark/benchmark.py` is used to run console implementations and compare performance.

## Repository Layout

- `rust/` Rust engine, console app, GUI app, FFI crate, and WASM crate
- `csharp/GameOfLife/` C# engine, GUI app, and console app
- `cpp/GameOfLife/` C++ engine library and console app
- `js/` simple website that loads the Rust WASM package
- `_benchmark/` benchmark runner and helper script

## Prerequisites

### General

- Git
- PowerShell or another terminal

### Rust Projects

- Stable Rust toolchain
- Cargo

### WASM Frontend

- `wasm-pack`
- Rust `wasm32-unknown-unknown` target
- Python 3
- A modern browser

### C# Projects

- .NET 9 SDK
- Stable Rust toolchain and Cargo

The C# projects depend on the Rust FFI build output. Build `rust/game-of-life-ffi` first so that `game_of_life_ffi.dll` exists before building or running the C# solution.

### C++ Projects

- Windows
- Visual Studio 2022 or MSVC Build Tools
- MSVC toolset `v143`
- Windows SDK 10.0
- `msbuild`

### Benchmark

- Python 3
- Built console executables for the implementations you want to compare

## Build And Run

Run all commands from the repository root.

### Rust Console App

Build and run with interactive input:

```powershell
cargo run --release --manifest-path .\rust\game-of-life-console\Cargo.toml -- --size 1000
```

Example using a benchmark-generated input file. Replace the path if you want to use your own sample:

```powershell
cargo run --release --manifest-path .\rust\game-of-life-console\Cargo.toml -- --size 1000 --file .\_benchmark\test_cases\case_grid100_cells50_steps10.txt
```

Input file format:

- One `x,y` coordinate pair per line
- No header line

### Rust GUI App

```powershell
cargo run --release --manifest-path .\rust\game-of-life-pixel\Cargo.toml
```

### Rust FFI Build

```powershell
cargo build --release --manifest-path .\rust\game-of-life-ffi\Cargo.toml
```

### Rust WASM Build

```powershell
rustup target add wasm32-unknown-unknown
wasm-pack build .\rust\game-of-life-wasm --target web --out-dir pkg
```

### JavaScript Site

First build the WASM package, then serve the repository root:

```powershell
python -m http.server 8080
```

If your system exposes Python as `python3`, use `python3 -m http.server 8080` instead.

Open:

```text
http://localhost:8080/js/
```

The site must be served from the repository root because `js/script.js` imports `/rust/game-of-life-wasm/pkg/game_of_life_wasm.js`.

### C# Solution Build

Build the Rust FFI crate first, then build the C# solution:

```powershell
cargo build --release --manifest-path .\rust\game-of-life-ffi\Cargo.toml
dotnet build .\csharp\GameOfLife\GameOfLife.sln -c Release
```

### C# GUI App

```powershell
dotnet run --project .\csharp\GameOfLife\GameOfLifePixel\GameOfLifePixel.csproj -c Release
```

### C# Console App

```powershell
dotnet run --project .\csharp\GameOfLife\GameOfLifeConsole\GameOfLifeConsole.csproj -c Release -- --size 1000
```

### C++ Build

```powershell
msbuild .\cpp\GameOfLife\GameOfLife.sln /p:Configuration=Release /p:Platform=x64
```

### C++ Console App

After a successful `Release|x64` build, run the default MSBuild output:

```powershell
.\cpp\GameOfLife\x64\Release\GameOfLifeConsole.exe
```

## Controls

### Rust And C# GUI Apps

- `Space`: run or pause
- `Enter`: step one generation
- Mouse wheel: zoom
- Right mouse drag: pan
- Left mouse click: seed a random square

### JavaScript Site

- Click `Next Generation` to advance one step
- Press `Space` to trigger the same action

## Benchmarking

The benchmark setup is intended to compare console implementations across the different language versions in this repository.

Build the console applications you want to measure, then run the current benchmark harness from `_benchmark/`:

```powershell
python .\_benchmark\benchmark.py --exe csharp .\csharp\GameOfLife\GameOfLifeConsole\bin\Release\net9.0\GameOfLifeConsole.exe --exe rust .\rust\game-of-life-console\target\release\game-of-life-console.exe
```

There is also a helper script with the same Rust and C# command pair:

```powershell
.\_benchmark\run.ps1
```

The C++ console app follows the same general text interaction model, but the current benchmark harness example is wired directly to the CLI and file-input console variants used by the Rust and C# apps.

## Notes

- The C# engine project copies `rust/game-of-life-ffi/target/release/game_of_life_ffi.dll` into its output directory.
- The Rust and C# console apps accept `--size` and `--file` arguments, which is why they integrate directly with the current benchmark runner.
- The JavaScript frontend expects the generated WASM package under `rust/game-of-life-wasm/pkg/`.
