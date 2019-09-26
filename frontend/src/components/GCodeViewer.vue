<template>
  <div>
    <pre><code v-for="(line, i) in lines" :key="i">{{line}}<br /></code></pre>
  </div>
</template>

<script lang="ts">
import { Component, Vue, Prop } from "vue-property-decorator";

@Component
export default class GCodeViewer extends Vue {
  @Prop({ type: String, default: "" })
  public text!: string;

  public get lines(): string[] {
    return this.text.split("\n");
  }
}
</script>

<style scoped>
pre {
  background: #fff;
  padding: 15px 16px 14px;
  white-space: pre;
  word-wrap: break-word;
  overflow: auto;
}

pre {
  margin: 20px 25px;
  border: 1px solid #d9d9d9;
  border-radius: 2px;
  position: relative;
  box-shadow: 0 1px 1px rgba(0, 0, 0, 0.08);
}

pre code {
  font-family: "Inconsolata", "Monaco", "Consolas", "Andale Mono",
    "Bitstream Vera Sans Mono", "Courier New", Courier, monospace;
  display: block;
  margin: 0 0 0 60px;
  overflow-x: auto;
  font-size: 13px;
  line-height: 1.5em;
  color: #444;
}

pre code {
  /* makes sure only commands have a left margin */
  margin: 0 !important;
}

pre:hover::after {
  opacity: 0;
  visibility: visible;
}

pre code {
  color: #78909c;
}

/* Line numbers for code snippets */
pre {
  counter-reset: line;
}

pre code:before {
  counter-increment: line;
  content: counter(line);
  display: inline-block;
  border-right: 1px solid #ddd;
  padding: 0 0.5em;
  margin-right: 0.5em;
  color: #888;
  width: 2em;

  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
</style>