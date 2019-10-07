<template>
  <div>
    <b-form inline @submit="onHomePressed">
      <label class="sr-only" for="homing-speed">Homing Speed</label>
      <b-input-group append="mm/s" class="mb-2 mr-sm-2 mb-sm-0">
        <b-input
          type="number"
          step="0.01"
          min="0"
          id="homing-speed"
          v-model.number="motion.homingSpeed"
        ></b-input>
      </b-input-group>

      <b-button type="submit" variant="primary">Home</b-button>
    </b-form>
  </div>
</template>

<script lang="ts">
import { Component, Vue, Emit, Prop } from "vue-property-decorator";
import MotionParameters from "../MotionParameters";
import { Packet } from "anpp";

@Component
export default class Controls extends Vue {
  public motion = new MotionParameters();
  @Prop()
  public send: (pkt: Packet) => Promise<Packet>;

  onHomePressed(e: Event) {
    e.preventDefault();
    this.home();
  }

  home() {
    return { speed: this.motion.homingSpeed };
  }
}
</script>
