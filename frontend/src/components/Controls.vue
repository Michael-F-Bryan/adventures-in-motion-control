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

@Component
export default class Controls extends Vue {
  public motion = new MotionParameters();
  @Prop({ required: true })
  public send!: (req: Request) => Promise<Response>;

  public onHomePressed(e: Event) {
    e.preventDefault();
    console.log("Going Home!");
    this.home()
      .then(resp => console.log(resp.toString(), resp))
      .catch(console.error);
  }

  private home() {
    return this.send(new GoHome(this.motion.homingSpeed));
  }
}
</script>
