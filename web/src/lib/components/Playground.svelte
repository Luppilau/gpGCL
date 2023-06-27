<script lang="ts">
 import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

 import { all_examples } from "$lib/examples/example-programs";
 import { execute } from "$lib/helpers/server_functions";
 import { example_data } from "$lib/examples/example-data";

 import Editor from "./Editor.svelte";
 import Select from "./ui/Select.svelte";
 import Button from "./ui/Button.svelte";
 import PlaygroundLayout from "./ui/PlaygroundLayout.svelte";
 import TextField from "./ui/TextField.svelte";
 import SpinningLoader from "./ui/SpinningLoader.svelte";

 import Plot from "./visualizations/Plot.svelte";
 import TextOutput from "./visualizations/TextOutput.svelte";

 let valid: boolean; // Has the entered program been validated on the server?
 let value: string = all_examples[0].value; // The value of the editor
 let executionArgs: string = all_examples[0].args; // Extra arguments for execution of the tool
 let executionResponse: Promise<any> | null = null; // The response from the server
 let editor: Monaco.editor.IStandaloneCodeEditor | null = null; // The editor instance

 function handleSelect(target: EventTarget | null) {
  let program = (target as HTMLSelectElement).value;
  editor?.getModel()?.setValue(program);
  executionArgs = all_examples.find((e) => e.value === program)?.args ?? "";
 }

 function handleExecute() {
  executionResponse = execute(value, executionArgs);
 }
</script>

<PlaygroundLayout>
 <!-- Editor section -->
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
    <!-- <Select options={[{ label: "Additional arguments", value: "" }]} /> -->
    <!-- <Select options={[{ label: "Additional arguments", value: "" }]} /> -->
    <TextField
     bind:value={executionArgs}
     placeholder="Execution args"
     style="width: 100%"
    />
   </div>
  </div>

  <!--  Visuzalisation results -->
  {#if executionResponse == null}
   <div class="center_align">
    <h3>No results yet</h3>

    <p>Run the query to see results here</p>
   </div>
  {:else}
   {#await executionResponse}
    <!-- While execution is running, a loader is shown -->
    <div class="center_align">
     <SpinningLoader />
    </div>
   {:then value}
    <!-- If successful execution data can be visualized -->

    <TextOutput>{value}</TextOutput>
    <!-- <Plot type="bar" data={example_data} /> -->
    <!-- <Plot type="line" data={example_data} /> -->
   {:catch error}
    <!-- If execution fails, an error message is shown together with the error message -->
    <div class="center_align" style="flex-shrink: 3">
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
</style>
