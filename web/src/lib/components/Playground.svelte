<script lang="ts">
 import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

 import { all_examples } from "$lib/examples/example-programs";
 import { execute } from "$lib/helpers/server_functions";

 import Editor from "./Editor.svelte";
 import Plot from "./visualizations/Plot.svelte";
 import Select from "./ui/Select.svelte";
 import Button from "./ui/Button.svelte";
 import PlaygroundLayout from "./ui/PlaygroundLayout.svelte";
 import SpinningLoader from "./SpinningLoader.svelte";
 import TextOutput from "./ui/TextOutput.svelte";
 import TextField from "./ui/TextField.svelte";

 let valid: boolean;
 let value: string = all_examples[0].value;
 let executionArgs: string = all_examples[0].args;
 let editor: Monaco.editor.IStandaloneCodeEditor | null = null;

 function handleSelect(target: EventTarget | null) {
  let program = (target as HTMLSelectElement).value;
  editor?.getModel()?.setValue(program);
  executionArgs = all_examples.find((e) => e.value === program)?.args ?? "";
 }

 function handleExecute() {
  executionResponse = execute(value, executionArgs);
 }

 let executionResponse: Promise<any> | null = null;
</script>

<PlaygroundLayout>
 <span slot="source" class="slot_wrapper">
  <Select
   options={all_examples}
   on:change={(selected) => handleSelect(selected.target)}
  />
  <Editor bind:value bind:editor bind:valid enableValidation />
 </span>

 <span slot="title">
  <Button on:click={handleExecute} disabled={!valid}>Execute</Button>
 </span>

 <span slot="output" class="slot_wrapper">
  <div>
   <p>Execution arguments:</p>
   <div class="output_controls">
    <Select options={[{ label: "Additional arguments", value: "" }]} />
    <Select options={[{ label: "Additional arguments", value: "" }]} />
    <TextField bind:value={executionArgs} placeholder="Execution args" />
   </div>
  </div>

  {#if executionResponse == null}
   <div class="center_align">
    <h3>No results yet</h3>

    <p>Run the query to see results here</p>
   </div>
  {:else}
   {#await executionResponse}
    <div class="center_align">
     <SpinningLoader />
    </div>
   {:then value}
    <TextOutput>
     {value}
    </TextOutput>
    <!-- <Plot type={value.type} data={value.data} /> -->
   {:catch error}
    <div class="center_align">
     <h3>Something went wrong</h3>
    </div>
    <TextOutput>
     {error}
    </TextOutput>
   {/await}
  {/if}
 </span>
</PlaygroundLayout>

<style>
 .slot_wrapper {
  display: contents;
 }

 .center_align {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
  width: 100%;
 }
 .output_controls {
  display: flex;
  gap: var(--size-3);
 }

 .input {
  width: 100%;
 }
</style>
