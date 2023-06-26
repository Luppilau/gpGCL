<script>
 export let disabled = false;
</script>

<button on:click {disabled} {...$$restProps}>
 <slot />
</button>

<style>
 button {
  border: none;
  box-shadow: none;
 }
</style>
