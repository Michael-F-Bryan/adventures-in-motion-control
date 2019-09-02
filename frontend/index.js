import * as wasm from "aimc-sim";

let world;

function init() {
    console.log("Initializing the world");
    world = wasm.setup_world();
    requestAnimationFrame(animate);
}

function animate() {
    console.log("Polling...")
    try {
        wasm.poll(world);
    } catch (error) {
        console.error(error);
    }

    requestAnimationFrame(animate);
}

init();