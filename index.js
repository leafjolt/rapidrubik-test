import init, { wasm_main } from "./pkg/rapidrubik.js";

async function run() {
  await init();
  wasm_main();
}

run();
