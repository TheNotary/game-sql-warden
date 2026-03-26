# SQL Labyrinth

A game that test's player's SQL knowledge.

## Usage

```
cargo install --path .
sql-warden
```


## TODO

x Do something about the panic on the map screen

- main menu should include:
  x Start (Light Mode)
  x Continue (Dark Mode)
  - Reset all data

- Opening Screen
  -

- app.assess_db() will throw a panic if the user's SQL solution is incorrect........... wuuuuuuu!!!!!TTTTT?????

- It would be cool to see the user's SQL solution updating in real time, in the bottom pane, as they type it... need a vim and nano mode though, and syntax highlighting, and column allign?

- Capture the output from the test so the user has more feedback about what's is tested and what's wrong?

- Make the start screen the map screen

- Document controls on the map screen

- Redocument controls, including the map screen

- Create a hidden level visible after all challenges are complete



## Compiling to WebAssembly (WASM)

TODO: This is all speculative, the game isn't totally complete yet!  Also I needed Claude to write some of this because I forgot I'm on an old intel machine and I downloaded the wrong wasi-sdk, lol. Delete it you say?  I can't just delete it because it cost me 31 cents!

This project can be compiled to WebAssembly using the `wasm32-wasip1` target.

See the `/wasm-loader` folder for the web portion of things.

The following setup is required:

### Prerequisites

1. **Install the WASI SDK** (version 29.0 or compatible)
   - For x86_64 macOS:
     ```bash
     cd ~
     curl -L -O https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-29/wasi-sdk-29.0-x86_64-macos.tar.gz
     tar -xzf wasi-sdk-29.0-x86_64-macos.tar.gz
     mv wasi-sdk-29.0-x86_64-macos wasi-sdk
     ```
   - For ARM64 (Apple Silicon) macOS:
     ```bash
     cd ~
     curl -L -O https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-29/wasi-sdk-29.0-arm64-macos.tar.gz
     tar -xzf wasi-sdk-29.0-arm64-macos.tar.gz
     mv wasi-sdk-29.0-arm64-macos wasi-sdk
     ```
   - For Linux, download the appropriate version from the [WASI SDK releases page](https://github.com/WebAssembly/wasi-sdk/releases)

2. **Add the Rust WASM target**
   ```bash
   rustup target add wasm32-wasip1
   ```

### Configuration

The project includes a `.env` file with the necessary environment variables for WASM compilation:

```bash
export WASI_SDK="${HOME}/wasi-sdk"
export CLANG_LIB_WASI=$WASI_SDK/lib/clang/21/lib/wasm32-unknown-wasip1
export WASI_SYSROOT=$WASI_SDK/share/wasi-sysroot
export CC_wasm32_wasip1="$WASI_SDK/bin/clang --target=wasm32-wasi --sysroot=$WASI_SYSROOT"
export AR_wasm32_wasip1="$WASI_SDK/bin/llvm-ar"
export AR="$WASI_SDK/bin/llvm-ar"
export RUSTFLAGS="-Clink-arg=-L$CLANG_LIB_WASI -Clink-arg=-lclang_rt.builtins"
```

### Building SQL-Warden for WASM

Source the environment file and build for WASM:

```bash
source .env # if needed...
cargo build --target wasm32-wasip2 --release
```

The compiled WASM binary will be available at:
```
target/wasm32-wasip2/release/sql-warden.wasm
```

TODO: copy rust-gpt-4.1-best-mode.agent.md into .agents, copilot should just pick that up :D
