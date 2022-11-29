<script>
  // @ts-nocheck
  import { Route } from "tinro";
  import NotFound from "./NotFound.svelte";
  import Home from "./pages/Home.svelte";
</script>

<main>
  <Route path="/"><Home /></Route>
  <Route fallback><NotFound /></Route>
</main>
