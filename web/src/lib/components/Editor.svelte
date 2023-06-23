<script lang="ts">
 import { configuration, language, tokenProvider } from "../config/language";
 import { onDestroy, onMount } from "svelte";
 import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";
 import loader from "@monaco-editor/loader";
 import { debounce } from "$lib/helpers/debounce";
 import { validate_input } from "$lib/helpers/validate_input";
 import SpinningLoader from "./SpinningLoader.svelte";

 export let value: string;
 export let editor: Monaco.editor.IStandaloneCodeEditor | null = null;
 export let model: Monaco.editor.ITextModel | null = null;
 export let monaco: typeof Monaco | null = null;

 let editorHasLoaded = false;
 let editorElement: HTMLElement;
 const options: Monaco.editor.IStandaloneEditorConstructionOptions = {
  readOnly: false,
  minimap: { enabled: false },
  scrollbar: {
   vertical: "hidden",
  },
  folding: false,
 };

 onMount(async () => {
  loader.config({ paths: { vs: "/node_modules/monaco-editor/min/vs" } });

  monaco = await loader.init();

  monaco.languages.register(language);
  monaco.languages.setLanguageConfiguration("gpgcl", configuration);
  monaco.languages.setMonarchTokensProvider("gpgcl", tokenProvider);

  editor = monaco.editor.create(editorElement, options);
  model = monaco.editor.createModel(value, "gpgcl");

  model.onDidChangeContent(() => {
   value = model?.getValue() || "";
  });

  editor.setModel(model);
  editorHasLoaded = true;
 });

 const updateEditorSize = () => {
  editor?.layout({ width: 0, height: 0 });

  window.requestAnimationFrame(() => {
   const rect = editorElement.getBoundingClientRect();

   editor?.layout({ width: rect.width, height: rect.height });
  });
 };

 const debounceUpdateEditorSize = debounce(updateEditorSize, 100);
 window.addEventListener("resize", debounceUpdateEditorSize);

 onDestroy(() => {
  monaco?.editor.getModels().forEach((model) => model.dispose());
  window.removeEventListener("resize", debounceUpdateEditorSize);
 });

 $: debouncedHandleUpdate(value, monaco, model);

 const debouncedHandleUpdate = debounce(validate_input, 800);
</script>

<div bind:this={editorElement} class="editor">
 {#if !editorHasLoaded}
  <SpinningLoader />
 {/if}
</div>

<!-- {#if !editorHasLoaded}{/if} -->

<style>
 .editor {
  height: 100%;
  display: grid;
  place-items: center;
 }
</style>
