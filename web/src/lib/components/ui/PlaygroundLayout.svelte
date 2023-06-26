<script lang="ts">
 import Card from "./Card.svelte";
</script>

<Card minHeight="350px">
 <h3>Source program</h3>

 <slot name="source" />
</Card>

<Card minHeight="300px" style="flex-grow: 4">
 <div class="title_container">
  <h3>Output</h3>
  <slot name="title" />
 </div>

 <slot name="output" />
</Card>

<style>
 .title_container {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
 }
</style>
