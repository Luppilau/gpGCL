<div class="container">
 <pre> 
    <slot />
  </pre>
</div>

<style>
 .container {
  padding: var(--size-2);
  position: relative;
  overflow: auto;
  height: 100%;
  background-color: white;
 }

 pre {
  position: absolute;
  color: black;
 }
</style>
