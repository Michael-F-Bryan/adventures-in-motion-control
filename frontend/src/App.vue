<template>
  <div id="app">
    <h1>Hello, World!</h1>
    <p>{{frequency}} Hz ({{tick_duration_us}} μs)</p>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import * as wasm from "aimc_sim";

@Component({})
export default class App extends Vue {
  private app?: wasm.App;
  private animateToken = 0;
  public frequency = 0;
  public tick_duration_us = 0;

  mounted() {
    // setup the world
    this.app = wasm.setup_world();

    // and schedule the animate() function to be called on the next tick
    this.animateToken = requestAnimationFrame(this.animate.bind(this));
  }

  beforeDestroy() {
    // make sure the animate method is cancelled when this component is removed
    // from the screen
    cancelAnimationFrame(this.animateToken);
  }

  animate() {
    // schedule animate to be called again
    this.animateToken = requestAnimationFrame(this.animate.bind(this));

    if (this.app) {
      // poll the app to let it make progress
      wasm.poll(this.app, this);
    }
  }

  set_fps(frequency: number, tick_duration_ms: number) {
    this.frequency = Math.round(frequency * 10) / 10;
    this.tick_duration_us = Math.round(tick_duration_ms * 1000);
  }

  send_data(data: Uint8Array) {
    console.log(new TextDecoder("utf-8").decode(data));
    // TODO: actually handle the message...
  }
}
</script>
