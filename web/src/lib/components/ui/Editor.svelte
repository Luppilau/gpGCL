<script lang="ts">
 import type * as Monaco from "monaco-editor";
 import * as monaco from "monaco-editor";
 import { onDestroy, onMount, tick } from "svelte";
 import { debounce } from "$lib/helpers/debounce";
 import { validate_input } from "$lib/helpers/server_functions";

 export let value: string = "";
 export let valid: boolean = true;
 export let editor: Monaco.editor.IStandaloneCodeEditor | null = null;
 export let enableValidation: boolean = false;
 export let options: Monaco.editor.IStandaloneEditorConstructionOptions = {};

 let model: Monaco.editor.ITextModel | null = null;
 let editorHasLoaded = false;
 let containerElement: HTMLElement;
 let editorElement: HTMLElement;

 const defaultOptions: Monaco.editor.IStandaloneEditorConstructionOptions = {
  readOnly: false,
  minimap: { enabled: false },
  scrollbar: {
   vertical: "hidden",
  },
  folding: false,
  hover: { enabled: true },
 };

 // Initialize editor
 onMount(async () => {
  editor = monaco.editor.create(editorElement, {
   ...defaultOptions,
   ...options,
  });

  model = monaco.editor.createModel(value, "gpgcl");
  editor.setModel(model);
  editorHasLoaded = true;

  // Validate input on change if not disabled
  if (enableValidation) {
   const debouncedHandleUpdate = debounce(async () => {
    if (model != null && editor != null) {
     value = model.getValue() || "";
     valid = await validate_input(value, monaco, model);
    }
   }, 800);

   model.onDidChangeContent(() => {
    valid = false;
    debouncedHandleUpdate();
   });
  }
 });

 // Update editor to fit space. Debounced for performance
 const debounceUpdateEditorSize = debounce(() => {
  editor?.layout({ width: 0, height: 0 });
  tick().then(() => {
   window.requestAnimationFrame(() => {
    const rect = containerElement.getBoundingClientRect();
    editor?.layout({
     width: rect.right - rect.left,
     height: rect.bottom - rect.top,
    });
   });
  });
 }, 30);
 window.addEventListener("resize", debounceUpdateEditorSize);

 // Cleanup
 onDestroy(() => {
  model?.dispose();
  window.removeEventListener("resize", debounceUpdateEditorSize);
 });
</script>

<div bind:this={containerElement} class="container">
 <div bind:this={editorElement} class="editor" />
</div>

<style>
 .editor {
  height: 100%;
 }

 .container {
  width: 100%;
  height: 100%;
 }
</style>
