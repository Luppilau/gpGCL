<script>
 export let disabled = false;
</script>

<button on:click {disabled}>
 <slot />
</button>

<style>
 button {
  border: none;
  box-shadow: none;
 }
</style>
