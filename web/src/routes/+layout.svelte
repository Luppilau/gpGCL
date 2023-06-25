<script>
 import Playground from "$lib/components/Playground.svelte";
 import { SvelteToast } from "@zerodevx/svelte-toast";
 import {
  gpgclConfiguration,
  gpgclLanguage,
  gpgclTokenProvider,
 } from "$lib/config/language";
 import * as monaco from "monaco-editor";

 // Global instatiation of Monaco editor
 monaco.languages.register(gpgclLanguage);
 monaco.languages.setLanguageConfiguration("gpgcl", gpgclConfiguration);
 monaco.languages.setMonarchTokensProvider("gpgcl", gpgclTokenProvider);
</script>

<main>
 <div class="content">
  <slot />
 </div>

 <div class="playground">
  <Playground />
 </div>
</main>

<div class="wrap">
 <SvelteToast target="top" options={{ intro: { y: -64 } }} />
</div>
<SvelteToast />

<style>
 main {
  overflow: hidden;
  box-sizing: border-box;
  height: 100%;
  width: 100%;
  display: grid;
  grid-template-columns: 3fr 2fr;
  gap: var(--size-2);
  padding: var(--size-5);
 }

 @media (max-width: 1050px) {
  main {
   overflow: auto;
   grid-template-columns: 1fr;
  }

  .content {
   overflow: unset !important;
  }
 }

 .playground {
  display: flex;
  flex-direction: column;
  gap: var(--size-5);
 }

 .content {
  display: flex;
  flex-direction: column;
  gap: var(--size-1);
  padding-right: var(--size-3);
  overflow: auto;
 }

 .content :global(code) {
  padding: var(--size-1);
  font-size: var(--font-size-1);
 }

 .content :global(table) {
  border-collapse: collapse;
  margin: var(--size-4) 0;
  min-width: 400px;
  box-shadow: var(--shadow-2);
 }

 .content :global(table thead tr) {
  background-color: var(--blue-6);
  text-align: left;
 }
 .content :global(table th),
 .content :global(table td) {
  padding: var(--size-2) var(--size-4);
 }
 .content :global(table tbody tr) {
  border-bottom: 1px solid var(--blue-6);
 }

 .content :global(table tbody tr:nth-of-type(even)) {
  background-color: var(--surface-2);
 }

 .content :global(table tbody tr:last-of-type) {
  border-bottom: 2px solid var(--blue-6);
 }

 .wrap {
  --toastContainerTop: 0.5rem;
  --toastContainerRight: 0.5rem;
  --toastContainerBottom: auto;
  --toastContainerLeft: 0.5rem;
  --toastWidth: 100%;
  --toastMinHeight: 2rem;
  --toastPadding: 0 0.5rem;
  font-size: 0.875rem;
 }
 @media (min-width: 40rem) {
  .wrap {
   --toastContainerRight: auto;
   --toastContainerLeft: calc(50vw - 20rem);
   --toastWidth: 40rem;
  }
 }

 :global(*) {
  max-inline-size: unset !important;
 }
</style>
