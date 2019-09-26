<template>
  <div class="console">
    <MessageViewer v-for="(msg, i) in messages" :key="i" :msg="msg" />
  </div>
</template>

<script lang="ts">
import { Component, Vue, Prop } from "vue-property-decorator";
import MessageViewer from "@/components/MessageViewer.vue";
import { Message, areMessages } from "../Message";

@Component({ components: { MessageViewer } })
export default class Terminal extends Vue {
  @Prop({ required: true, default: () => [], validator: areMessages })
  public messages!: Message[];
}
</script>

<style>
.console {
  background-color: black;
  max-height: 45vh;
  overflow-y: scroll;
  padding: 1em;
}
</style>