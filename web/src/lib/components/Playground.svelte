<script lang="ts">
 import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

 import { all_examples } from "$lib/examples/example-programs";
 import { data } from "$lib/examples/example-data";

 import Editor from "./Editor.svelte";
 import Plot from "./visualizations/Plot.svelte";
 import Card from "./ui/Card.svelte";
 import Select from "./ui/Select.svelte";
 import Button from "./ui/Button.svelte";
 import { toast } from "@zerodevx/svelte-toast";

 let value: string = all_examples[0].value;
 let editor: Monaco.editor.IStandaloneCodeEditor | null = null;
 let model: Monaco.editor.ITextModel | null = null;
 let monaco: typeof Monaco | null = null;

 function handleSelect(target: EventTarget | null) {
  model?.setValue((target as HTMLSelectElement).value);
 }
</script>

<Card>
 <div class="controls">
  <h3>Source program</h3>
  <Button on:click={() => toast.push("ERROR", { target: "top" })}>
   Execute
  </Button>
 </div>

 <Select
  options={all_examples}
  on:change={(selected) => handleSelect(selected.target)}
 />

 <Editor bind:value bind:model bind:editor bind:monaco />
</Card>

<Card>
 <h3>Output</h3>
 <Plot type="bar" {data} />
</Card>

<style>
 .controls {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
 }
</style>
