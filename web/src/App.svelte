<script>
  // @ts-nocheck
  import { Route } from "tinro";
</script>

<main>
  <nav>
    <a href="/">Home</a>
    <a href="/portfolio">Portfolio</a>
    <a href="/contacts">Contacts</a>
  </nav>

  <Route path="/">
    <h1>This is the main page</h1>
  </Route>
  <Route fallback>Page not found!?</Route>
</main>
