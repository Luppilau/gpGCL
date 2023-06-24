<script lang="ts">
 import Card from "./Card.svelte";
</script>

<Card>
 <div class="controls">
  <h3>Source program</h3>

  <slot name="controls" />
 </div>

 <slot name="source" />
</Card>

<Card>
 <h3>Output</h3>
 <slot name="output" />
</Card>

<style>
 .controls {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
 }
</style>
