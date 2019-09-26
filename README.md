# Adventures in Motion Control

[![Build Status](https://travis-ci.com/Michael-F-Bryan/adventures-in-motion-control.svg?branch=master)](https://travis-ci.com/Michael-F-Bryan/adventures-in-motion-control)

(**[API Documentation][api-docs]/[Blog Series][blog]**)

A realistic simulator for a 3D printer motion controller.

Think of this as a worked example of how embedded systems are made. The project
is a lot more than just a gcode interpreter, it gives you a full-blown motion
controller which behaves like a real 3D printer complete with automation
sequences and diagnostics.

To follow along with the making of this project, check out the [Adventures in
Motion Controller][blog] blog series.

## Getting Started

This project has components from two different languages, which makes
building things a little... complex.

First you'll need to generate the WASM bundle for the `sim` crate using
[wasm-pack][wp].

```console
$ wasm-pack build --release sim
$ ls sim/pkg
  aimc_sim.d.ts aimc_sim.js aimc_sim_bg.d.ts aimc_sim_bg.wasm package.json
  README.md sim.d.ts sim.js sim_bg.d.ts sim_bg.wasm
```

The generated WASM also needs to be linked into our JavaScript.
In order to do this first go to `sim/pkg`

```console
$ cd sim/pkg
$ yarn link
```
This will globally register `aimc_sim` package.
Next go to `frontend`

```console
$ cd frontend
$ yarn link aimc_sim
```

Next, you'll need to compile the `frontend` JavaScript code. During development,
you'll probably want to use the dev server:

```console
$ yarn install
$ yarn serve
```

Or to generate a production build:

```console
$ yarn run build
$ ls dist
  0.bootstrap.js 1.bootstrap.js bootstrap.js ea9e01c84ad93861b0ad.module.wasm
  index.html
```

If you want the code to be automatically recompiled whenever there are any
changes, you can use`watchexec`. Open new terminal end execute.

TL;DR:

```console
watchexec \
    --clear \
    --restart \
    --ignore 'sim/pkg/*' \
    --ignore 'frontend/package.json' \
    --ignore 'frontend/yarn.lock' \
    --ignore 'frontend/node_modules/*' \
    'wasm-pack build --release sim'
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[blog]: http://adventures.michaelfbryan.com/tags/adventures-in-motion-control/
[api-docs]: https://michael-f-bryan.github.io/adventures-in-motion-control/
[wp]: https://github.com/rustwasm/wasm-pack
