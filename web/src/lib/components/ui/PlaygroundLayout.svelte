<script lang="ts">
 import Card from "./Card.svelte";
</script>

<Card minHeight="350px">
 <h3>Source program</h3>

 <slot name="source" />
</Card>

<Card minHeight="300px" style="flex-grow: 4">
 <h3>Output</h3>
 <slot name="output" />
</Card>

<style>
</style>
