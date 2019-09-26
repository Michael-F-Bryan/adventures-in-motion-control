<template>
  <div class="message" :class="style">
    <span class="direction">{{arrow}}</span>
    <span class="timestamp">{{timestamp}}</span>
    <pre>{{msg.toString()}}</pre>
  </div>
</template>

<script lang="ts">
import { Component, Vue, Prop } from "vue-property-decorator";
import { Message, isMessage, Direction } from "../Message";

@Component
export default class MessageViewer extends Vue {
  @Prop({ required: true, validator: isMessage })
  public msg!: Message;

  public get timestamp(): string {
    return this.msg.timestamp
      .toISOString()
      .replace("T", " ")
      .replace("Z", "");
  }

  public get arrow(): string {
    switch (this.msg.direction) {
      case Direction.Sent:
        return "▶";
      case Direction.Received:
        return "◀";
    }
  }

  public get style(): string {
    switch (this.msg.direction) {
      case Direction.Sent:
        return "sent";
      case Direction.Received:
        return "received";
    }
  }
}
</script>

<style>
.message {
  display: flex;
}

.message::before {
  margin-right: 1em;
}

.message * {
  font: 1.3rem Inconsolata, monospace;
}

.sent * {
  color: goldenrod;
}

.received * {
  color: skyblue;
}

.timestamp {
  margin-right: 1em;
  font-weight: bold;
}

.timestamp::after {
  content: ":";
}

.direction {
  margin-right: 1em;
}
</style>