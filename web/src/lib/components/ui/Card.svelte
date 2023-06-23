<div class="card">
 <slot />
</div>

<style>
 .card {
  display: flex;
  flex-direction: column;
  gap: var(--size-2);
  height: 50%;
  background-color: var(--surface-2);
  border-radius: 4px;
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.05), 0 1px 3px 0 rgba(0, 0, 0, 0.1);
  padding: 16px;
 }
</style>
