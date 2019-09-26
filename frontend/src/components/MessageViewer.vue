<template>
  <div class="message">
    <span class="timestamp">{{timestamp}}</span>
    {{msg.toString()}}
  </div>
</template>

<script lang="ts">
import { Component, Vue, Prop } from "vue-property-decorator";
import { Message, isMessage } from "../Message";

@Component({})
export default class MessageViewer extends Vue {
  @Prop({ required: true, validator: isMessage })
  public msg!: Message;

  public get timestamp(): string {
    return this.msg.timestamp
      .toISOString()
      .replace("T", " ")
      .replace("Z", "");
  }
}
</script>

<style>
.message {
  display: inline;
  color: rgb(51, 255, 0);
  font: 1.3rem Inconsolata, monospace;
}

.timestamp {
  font-weight: bold;
}

.timestamp::after {
  content: ":";
}
</style>