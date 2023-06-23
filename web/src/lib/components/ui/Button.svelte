<button on:click>
 <slot />
</button>

<style>
 button {
  padding-block: 0.75ch;
  padding-inline: var(--size-relative-4);
  border-radius: var(--radius-2);
  background-color: #fff;
 }

 button:active {
  background-color: var(--surface-1);
 }

 @media (prefers-color-scheme: dark) {
  button {
   background-color: var(--surface-3);
  }
  button:active {
   background-color: var(--surface-2);
  }
 }
</style>
