<script lang="ts">
 import Chart from "chart.js/auto";
 import type * as ChartT from "chart.js/auto";
 import { onMount } from "svelte";

 export let type: "line" | "bar";
 export let data: ChartT.ChartData;
 export let options: ChartT.ChartOptions = {};

 let canvas_element: HTMLCanvasElement;

 onMount(() => {
  new Chart(canvas_element, {
   type,
   data,
   options: { ...options, responsive: true, maintainAspectRatio: false },
  });
 });
</script>

<div class="container">
 <canvas bind:this={canvas_element} />
</div>

<style>
 .container {
  height: 100%;
  width: 100%;
  background-color: white;
 }
</style>
