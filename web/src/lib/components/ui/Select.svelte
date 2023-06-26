<script lang="ts">
 type SelectOption = { value: any; label: string };
 type SelectOptions = SelectOption[];

 export let options: SelectOptions;
</script>

<select name="selector" id="selector" on:change {...$$restProps}>
 {#each options as item}
  <option value={item.value}>{item.label}</option>
 {/each}
</select>

<style>
 select {
  width: 100%;
  background-color: #fff;
 }

 @media (prefers-color-scheme: dark) {
  select {
   background-color: var(--surface-3);
  }
 }
</style>
