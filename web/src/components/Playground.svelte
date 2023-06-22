<script lang="ts">
 import { coin_example, bounded_retransmission } from "./examples";
 import Editor from "./Editor.svelte";
 import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

 import { debounce } from "../helpers/debounce";
 import { validate_input } from "../helpers/validate_input";

 let value: string = coin_example;
 let editor: Monaco.editor.IStandaloneCodeEditor | null = null;
 let model: Monaco.editor.ITextModel | null = null;
 let monaco: typeof Monaco | null = null;

 $: debouncedHandleClick(value, monaco, model);

 const debouncedHandleClick = debounce(validate_input, 800);

 function handleSelect(target: EventTarget | null) {
  model?.setValue((target as HTMLSelectElement).value);
 }
</script>

<div class="container">
 <select
  name="example selector"
  id="example_selector"
  class="example_selector"
  on:change={(selected) => handleSelect(selected.target)}
 >
  <option value={coin_example}>Coin flip</option>
  <option value={bounded_retransmission}>Bounded Retransmission Protocol</option
  >
 </select>
 <Editor bind:value bind:model bind:editor bind:monaco />
</div>

<style>
 .container {
  display: grid;
  gap: 8px;
 }

 .example_selector {
  width: 100%;
  max-width: 150px;
 }
</style>
