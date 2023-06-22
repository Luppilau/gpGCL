<script lang="ts">
 import { coin_example, bounded_retransmission } from "./examples";
 import Editor from "./Editor.svelte";
 import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

 let value: string = coin_example;
 let editor: Monaco.editor.IStandaloneCodeEditor | null = null;
 let model: Monaco.editor.ITextModel | null = null;

 function handleSelect(target: EventTarget | null) {
  model?.setValue((target as HTMLSelectElement).value);
 }

 async function handleClick() {
  console.log(value);

  const res = await fetch("http://127.0.0.1:8000/handle", {
   method: "POST",
   body: value,
  });
  const text = await res.text();
  console.log(text);
 }

 function setErrors() {
  // monaco.editor.setModelMarkers(model, "owner", [
  //  {
  //   startLineNumber: 1,
  //   startColumn: 1,
  //   endLineNumber: 1,
  //   endColumn: 1,
  //   message: "This is an error",
  //   severity: monaco.MarkerSeverity.Error,
  //  },
  // ]);
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
 <Editor bind:value bind:model bind:editor />

 <button on:click={handleClick}> Transpile </button>
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
