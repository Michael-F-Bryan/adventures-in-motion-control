import * as wasm from "aimc-sim";

let world;

function init() {
    console.log("Initializing the world");
    world = wasm.setup_world("#fps-counter");
    requestAnimationFrame(animate);

    world.on_data_sent(data => {
        const str = new TextDecoder("utf-8").decode(data.slice(5));
        console.log("Received", data, str);
    });
    setTimeout(() => world.echo("Hello, World!"), 500);
}

function animate() {
    wasm.poll(world);
    requestAnimationFrame(animate);
}

init();