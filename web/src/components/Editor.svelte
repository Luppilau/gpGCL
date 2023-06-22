<script lang="ts">
 import { configuration, language, tokenProvider } from "./language";
 import loader from "@monaco-editor/loader";
 import { onDestroy, onMount } from "svelte";
 import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

 export let value: string;
 export let editor: Monaco.editor.IStandaloneCodeEditor | null = null;
 export let model: Monaco.editor.ITextModel | null = null;

 let monaco: typeof Monaco;
 let editorElement: HTMLElement;
 const options: Monaco.editor.IStandaloneEditorConstructionOptions = {
  readOnly: false,
  minimap: { enabled: false },
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
 });

 onDestroy(() => {
  monaco?.editor.getModels().forEach((model) => model.dispose());
 });
</script>

<div bind:this={editorElement} class="editor" />

<style>
 .editor {
  height: 100%;
  min-height: 300px;
 }
</style>
