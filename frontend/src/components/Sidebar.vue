<template>
  <div class="sidebar" role="tablist">
    <b-card no-body class="mb-1">
      <b-card-header header-tag="header" class="p-1" role="tab">
        <b-button block href="#" v-b-toggle.accordion-1 variant="info">Overview</b-button>
      </b-card-header>
      <b-collapse id="accordion-1" visible accordion="my-accordion" role="tabpanel">
        <b-card-body>
          <table class="table">
            <tr>
              <td>FPS</td>
              <td>
                {{frequency}} Hz
                <small>({{tick_duration_us}} μs)</small>
              </td>
            </tr>
          </table>
        </b-card-body>
      </b-collapse>
    </b-card>

    <b-card no-body class="mb-1">
      <b-card-header header-tag="header" class="p-1" role="tab">
        <b-button block href="#" v-b-toggle.accordion-2 variant="info">Current State</b-button>
      </b-card-header>
      <b-collapse id="accordion-2" accordion="my-accordion" role="tabpanel">
        <b-card-body>
          <table class="table">
            <tr>
              <td>X</td>
              <td>123</td>
            </tr>
            <tr>
              <td>Y</td>
              <td>321</td>
            </tr>
            <tr>
              <td>Z</td>
              <td>456.7</td>
            </tr>
          </table>
        </b-card-body>
      </b-collapse>
    </b-card>
  </div>
</template>

<script lang="ts">
import { Component, Vue, Prop } from "vue-property-decorator";

@Component
export default class Sidebar extends Vue {
  @Prop({ default: 0, type: Number })
  public frequency = 0;
  @Prop({ default: 0, type: Number })
  public tick_duration_us = 0;
}
</script>

<style>
.sidebar {
  margin-top: 30px;
  margin-left: 30px;
  width: 350px;
  height: 100%;
}
</style>