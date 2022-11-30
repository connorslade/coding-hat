<script>
  import Content from "../lib/Content.svelte";
  import Header from "../lib/Header.svelte";
  import { randStr, state } from "../state";

  let user = null;
  state.getUser().then((u) => (user = u));
</script>

<Header page={"home"} />

<Content>
  {#if !user}
    <p>Welcome to Coding Hat!</p>
    <p>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      You can
      <span
        class="link"
        on:click={() => (document.location = `/auth/redirect?${randStr()}`)}
        >login with google</span
      >
      to get started!
    </p>
  {:else}
    <h1>
      {user.new
        ? `Welcome to Codeing Hat, ${user.name}`
        : `Welcome back, ${user.name}`}
    </h1>
    <p>Why not try an <a class="link" href="/p/200079">example problem</a>!</p>
  {/if}

  <br />
  <p>This homepage is currently unfinished.</p>
</Content>

<style>
  .link {
    color: rgb(14 165 233);
    text-decoration: underline;
    cursor: pointer;
  }
</style>
