<script lang="ts">
 import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

 import { all_examples } from "$lib/examples/example_programs";
 import { execute } from "$lib/helpers/server_functions";
 import { example_data } from "$lib/examples/example_data";

 import Editor from "./ui/Editor.svelte";
 import Select from "./ui/Select.svelte";
 import Button from "./ui/Button.svelte";
 import PlaygroundLayout from "./ui/PlaygroundLayout.svelte";
 import TextField from "./ui/TextField.svelte";
 import SpinningLoader from "./ui/SpinningLoader.svelte";

 import Plot from "./visualizations/Plot.svelte";
 import TextOutput from "./visualizations/TextOutput.svelte";
 import { toast } from "@zerodevx/svelte-toast";

 let editor: Monaco.editor.IStandaloneCodeEditor | null = null; // The editor instance

 let isProgramValid: boolean; // Has the entered program been validated on the server?
 let editorValue: string = all_examples[0].value; // The value of the editor
 let executionArgs: string = all_examples[0].args; // Extra arguments for execution of the tool
 let executionResponse: Promise<any> | null = null; // The response from the server

 function handleSelectExample(target: EventTarget | null) {
  let program = (target as HTMLSelectElement).value;
  let selectedExample = all_examples.find((e) => e.value === program);
  if (selectedExample == undefined) {
   toast.push("Something went wrong when selecting example");
   return;
  }

  editor?.getModel()?.setValue(selectedExample.value);
  executionArgs = selectedExample?.args ?? "";
 }

 function handleExecute() {
  // TODO: Accept additional parameters for server execution here.
  executionResponse = execute(editorValue, { executionArgs });
 }
</script>

<PlaygroundLayout>
 <!-- Editor section -->
 <span slot="source" class="slot_wrapper">
  <Select
   options={all_examples}
   on:change={(selected) => handleSelectExample(selected.target)}
  />
  <Editor
   bind:value={editorValue}
   bind:editor
   bind:valid={isProgramValid}
   enableValidation
  />
 </span>

 <span slot="title">
  <Button on:click={handleExecute} disabled={!isProgramValid}>Execute</Button>
 </span>

 <span slot="output" class="slot_wrapper">
  <div>
   <p>Execution arguments:</p>
   <div class="output_controls">
    <!-- <Select options={[{ label: "Additional arguments", value: "" }]} /> -->
    <TextField bind:value={executionArgs} placeholder="Execution args" />
   </div>
  </div>

  <!--  Visuzalisation results -->
  {#if executionResponse == null}
   <!-- Empty state -->
   <div class="center_align">
    <h3>No results yet</h3>

    <p>Run the query to see results here</p>
   </div>
  {:else}
   {#await executionResponse}
    <!-- Loading while executing -->
    <div class="center_align">
     <SpinningLoader />
    </div>
   {:then executionResult}
    <!-- If successful execution, data can be visualized -->
    <!-- TODO: Implement relevant visualization -->

    <TextOutput>{executionResult}</TextOutput>
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
