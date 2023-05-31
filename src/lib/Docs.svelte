<script lang="ts">
  import { marked } from 'marked';
  export let readme: string;
</script>

<div class="readme">
  {@html marked.parse(readme)}
</div>

<style lang="scss">
.readme {
  max-width: 900px;
  margin: 1rem auto;
  padding: 1rem;
  border-radius: 0.5rem;
  background: var(--bg-2);
  box-shadow: 1px 3px 3px #00000080;
  overflow: hidden;
  :global(p),
  :global(ul),
  :global(details),
  :global(code),
  :global(h1),
  :global(h2),
  :global(h3),
  :global(h4),
  :global(h5) {
    color: var(--fg);
  }
  :global(h1),
  :global(h2),
  :global(h3),
  :global(h4) {
    font-family: 'RobotoSlab';
  }
  :global(a) {
    color: var(--primary);
  }
  :global(img) {
    max-width: 100%;
  }
  :global(pre) {
    background: var(--bg-3);
    padding: 0.5rem;
    border-radius: 0.25rem;
    overflow: auto;
  }
  :global(code) {
    background: var(--bg-3);
    padding: 1px 4px;
    border-radius: 4px;
    font-family: 'JetBrains Mono';
  }
}
</style>
