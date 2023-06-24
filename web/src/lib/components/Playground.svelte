<script lang="ts">
 import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

 import { all_examples } from "$lib/examples/example-programs";
 import { data } from "$lib/examples/example-data";
 import { toast } from "@zerodevx/svelte-toast";

 import Editor from "./Editor.svelte";
 import Plot from "./visualizations/Plot.svelte";
 import Select from "./ui/Select.svelte";
 import Button from "./ui/Button.svelte";
 import PlaygroundLayout from "./ui/PlaygroundLayout.svelte";

 let value: string = all_examples[0].value;
 let editor: Monaco.editor.IStandaloneCodeEditor | null = null;
 let model: Monaco.editor.ITextModel | null = null;

 function handleSelect(target: EventTarget | null) {
  model?.setValue((target as HTMLSelectElement).value);
 }
</script>

<PlaygroundLayout>
 <span slot="controls" class="slot_wrapper">
  <Button on:click={() => toast.push("Not implemented", { target: "top" })}>
   Execute
  </Button>
 </span>

 <span slot="source" class="slot_wrapper">
  <Select
   options={all_examples}
   on:change={(selected) => handleSelect(selected.target)}
  />
  <Editor bind:value bind:editor enableValidation />
 </span>

 <span slot="output" class="slot_wrapper">
  <Plot type="bar" {data} />
 </span>
</PlaygroundLayout>

<style>
 .slot_wrapper {
  display: contents;
 }
</style>
