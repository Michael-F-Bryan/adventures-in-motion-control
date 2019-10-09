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
import { Request, Response, GoHome, Nack } from "../messaging";
import { Packet } from "anpp";

function alwaysFails(req: Request): Promise<Response> {
  return Promise.reject("Not Connected");
}

@Component
export default class Controls extends Vue {
  public motion = new MotionParameters();
  @Prop({ default: () => alwaysFails })
  public send!: (req: Request) => Promise<Response>;

  onHomePressed(e: Event) {
    e.preventDefault();
    console.log("Going Home!");
    this.home();
  }

  async home() {
    const response = await this.send(new GoHome(this.motion.homingSpeed));
  }
}
</script>
