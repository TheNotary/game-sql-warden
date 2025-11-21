//import { WASI } from "https://unpkg.com/@bytecodealliance/preview2-shim@0.17.5/dist/wasi.js";
import { WASIShim } from '@bytecodealliance/preview2-shim/instantiation';
import type {
    VersionedWASIImportObject,
    WASIImportObject,
} from '@bytecodealliance/preview2-shim/instantiation';

const shim = new WASIShim();


const outputElem = document.getElementById("output");

function log(msg) {
  outputElem.textContent += msg + "\n";
}

async function main() {
  // Load your WASM binary
  const wasmBytes = await fetch("./target/wasm32-wasip2/release/sql-warden.wasm")
    .then(r => r.arrayBuffer());

  // Set up WASI Preview 2
  const wasi = new WASI({
    stdout: (data) => log(new TextDecoder().decode(data)),
    stderr: (data) => log("[ERR] " + new TextDecoder().decode(data)),
  });

  // Instantiate the WASM module + WASI
  const { instance } = await WebAssembly.instantiate(wasmBytes, {
    "wasi:cli": wasi.wasiImport,  // WASI Preview 2 entrypoints
    "wasi:io": wasi.wasiImport,
    "wasi:filesystem": wasi.wasiImport,
    "wasi:random": wasi.wasiImport,
  });

  // Start your WASI component
  await wasi.start(instance);

  log("Program finished.");
}

main();

