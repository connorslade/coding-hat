<script>
  // @ts-nocheck
  import { Route } from "tinro";
  import NotFound from "./NotFound.svelte";
  import About from "./pages/About.svelte";
  import Home from "./pages/Home.svelte";
</script>

<main>
  <Route path="/"><Home /></Route>
  <Route path="/about"><About /></Route>
  <Route fallback><NotFound /></Route>
</main>
