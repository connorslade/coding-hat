<script>
  import Content from "./lib/Content.svelte";
  import Header from "./lib/Header.svelte";
</script>

<Header page="not-found" />

<Content>
  <h1 class="font-bold underline">
    The page <code>{location.pathname}</code> was not found.
  </h1>
</Content>
