# Strongest Cubical

## 🧱 The Cubical Dungeon Guardian Speaks

A dim lantern flickers above rows of perfectly aligned stone cubes.
From behind one of the cubicles, a hunched figure emerges—half-scribe, half-daemon, all SQL.

“Ahh… a wanderer! You’ve entered the Cubical Dungeon, where data goes to die—or be optimized.
I am the Warden of Cubes, caretaker of rows and columns long forgotten.
You seek passage? Then face my challenge.”

He slams a stone tablet onto a slab between you.

“Monsters lurk in each cube. Some weak, some mighty, some too embarrassed to list their hit points.
Your challenge: for every cube, determine the single strongest monster that dwells within it.
When you believe you’ve solved this riddle using SQLite… run the Test Ritual to see whether your answer is worthy.”

The Warden fades back into the cubicles, whispering:
“Migrate wisely, wanderer…”

## 🧩 The Challenge

After running the migrations below, write a query that produces a table with:

    cube_id | cube_name | monster_id | monster_name | hp

…containing the strongest (highest HP) monster in each cube.
If a cube has no monsters, it should not appear (inner semantics).

Create your solution as a view named strongest_monsters.


## Technical Instructions

    sqlite3 test.db < migration.sql
    sqlite3 test.db

## Compiling to WebAssembly (WASM)

This project can be compiled to WebAssembly using the `wasm32-wasip1` target. The following setup is required:

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

