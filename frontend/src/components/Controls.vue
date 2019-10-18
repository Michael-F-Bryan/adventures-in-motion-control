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

    <b-form inline @submit="onSendGcode">
      <label class="sr-only" for="gcode-send">Manually send g-code</label>
      <b-input-group prepend="Manual g-code" class="mb-2 mr-sm-2 mb-sm-0">
        <b-input id="gcode" v-model="gcodeProgram"></b-input>
      </b-input-group>

      <b-button type="submit" variant="primary">Send</b-button>
    </b-form>
  </div>
</template>

<script lang="ts">
import { Component, Vue, Emit, Prop } from "vue-property-decorator";
import MotionParameters from "../MotionParameters";
import { Request, Response, GoHome, Nack, GcodeProgram } from "../messaging";
import { Packet } from "anpp";

@Component
export default class Controls extends Vue {
  public motion = new MotionParameters();
  public gcodeProgram: string = "";
  @Prop({ required: true })
  public send!: (req: Request) => Promise<Response>;

  public onHomePressed(e: Event) {
    e.preventDefault();
    console.log("Going Home!");
    this.home()
      .then(resp => console.log(resp.toString(), resp))
      .catch(console.error);
  }

  public onSendGcode(e: Event) {
    e.preventDefault();

    const program = this.gcodeProgram;
    this.gcodeProgram = "";

    if (program.length > 0) {
      console.log("Sending", program);
      this.sendGcode(program)
        .then(resp => console.log(resp.toString(), resp))
        .catch(console.error);
    }
  }

  private home() {
    return this.send(new GoHome(this.motion.homingSpeed));
  }

  private sendGcode(program: string) {
    const buffer = new TextEncoder().encode(program);
    return this.send(new GcodeProgram(0, buffer));
  }
}
</script>
