import * as wasm from "aimc-sim";

let world;

function init() {
    console.log("Initializing the world");
    world = wasm.setup_world();
    requestAnimationFrame(animate);
}

function animate() {
    console.log("Polling...")
    wasm.poll(world);
    requestAnimationFrame(animate);
}

init();