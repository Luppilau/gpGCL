<script lang="ts">
 export let value: string;
 export let placeholder: string = "";
</script>

<input
 {...$$restProps}
 bind:value
 {placeholder}
 type="text"
 name="output_args"
 id="output_args"
 class="input"
/>
