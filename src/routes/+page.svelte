<script>
  import Hero from '$lib/Hero.svelte';
  import Docs from '$lib/Docs.svelte';

  export let data;
</script>

<Hero />
<Docs readme={data.readme} />
