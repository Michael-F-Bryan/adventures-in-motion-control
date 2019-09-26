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
import { Component, Vue, Emit } from "vue-property-decorator";
import MotionParameters from "../MotionParameters";

@Component
export default class Controls extends Vue {
  public motion = new MotionParameters();

  onHomePressed(e: Event) {
    e.preventDefault();
    this.home();
  }

  @Emit()
  home() {
    return { speed: this.motion.homingSpeed };
  }
}
</script>
