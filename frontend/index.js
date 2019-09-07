import * as wasm from "aimc-sim";

let world;

function init() {
    console.log("Initializing the world");
    world = wasm.setup_world("#fps-counter");
    world.on_data_sent(data => console.log("Received", data));
    requestAnimationFrame(animate);
}

function animate() {
    wasm.poll(world);
    requestAnimationFrame(animate);
}

init();