<script lang="ts">
 import type * as Monaco from "monaco-editor";
 import * as monaco from "monaco-editor";
 import { onDestroy, onMount } from "svelte";
 import { debounce } from "$lib/helpers/debounce";
 import { validate_input } from "$lib/helpers/validate_input";

 import SpinningLoader from "./SpinningLoader.svelte";

 export let value: string = "";
 export let editor: Monaco.editor.IStandaloneCodeEditor | null = null;
 export let enableValidation: boolean = false;
 export let options: Monaco.editor.IStandaloneEditorConstructionOptions = {};

 let model: Monaco.editor.ITextModel | null = null;
 let editorHasLoaded = false;
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

 onMount(async () => {
  editor = monaco.editor.create(editorElement, {
   ...defaultOptions,
   ...options,
  });
  model = monaco.editor.createModel(value, "gpgcl");

  editor.setModel(model);
  editorHasLoaded = true;

  if (enableValidation) {
   const debouncedHandleUpdate = debounce(() => {
    if (model != null && editor != null) {
     value = model.getValue() || "";
     validate_input(value, monaco, model);
    }
   }, 800);

   model.onDidChangeContent(() => {
    debouncedHandleUpdate();
   });
  }
 });

 const debounceUpdateEditorSize = debounce(() => {
  editor?.layout({ width: 0, height: 0 });

  window.requestAnimationFrame(() => {
   const rect = editorElement.getBoundingClientRect();

   editor?.layout({ width: rect.width, height: rect.height });
  });
 }, 100);
 window.addEventListener("resize", debounceUpdateEditorSize);
 debounceUpdateEditorSize();

 onDestroy(() => {
  monaco?.editor.getModels().forEach((model) => model.dispose());
  window.removeEventListener("resize", debounceUpdateEditorSize);
 });
</script>

<div bind:this={editorElement} class="editor">
 {#if !editorHasLoaded}
  <SpinningLoader />
 {/if}
</div>

<style>
 .editor {
  height: 100%;
  display: grid;
  place-items: center;
 }
</style>
