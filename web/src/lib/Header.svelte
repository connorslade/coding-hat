<script>
  import { ArrowRightOnRectangle, ArrowLeftOnRectangle } from "svelte-heros-v2";
  import { router } from "tinro";
  import favicon from "../assets/favicon-32x32.png";
  import { state } from "../state";

  export let page;
</script>

<nav
  class="bg-slate-800 m-6 h-12 mx-auto rounded flex justify-start items-center drop-shadow-lg"
>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div
    class="flex items-center cursor-pointer mr-1.5"
    on:click={() => router.goto("/")}
  >
    <img src={favicon} class="mx-3 inline-block" alt="coding hat logo" />
    <p class="text-xl inline-block">Coding Hat</p>
  </div>

  <div class="border border-slate-700 rounded m-1 h-1/2" />

  {#each ["home", "about"] as p}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div
      class="cursor-pointer bg-slate-800 hover:bg-slate-700 rounded p-1.5 px-2 ml-1.5"
      style="background-color: {page === p ? '#334155' : ''}"
      on:click={() => router.goto(p === "home" ? "/" : `/${p}`)}
    >
      <p>{p}</p>
    </div>
  {/each}

  {#if !state}
    <ArrowLeftOnRectangle
      size="36"
      name="log-in"
      title="Log In"
      class="log ml-auto cursor-pointer bg-slate-800 hover:bg-slate-700 rounded m-1.5 inline-block"
      on:click={() => router.goto("/auth/redirect")}
    />
  {:else}
    <div class="ml-auto flex mr-1.5">
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <img
        src={state.avatar}
        width="36px"
        height="auto"
        title="Your Profile"
        alt="icon"
        referrerpolicy="no-referrer"
        class="cursor-pointer bg-slate-800 hover:bg-slate-700 rounded mr-1.5"
        on:click={() => router.goto(`/profile/${state.id}`)}
      />

      <ArrowRightOnRectangle
        size="36"
        name="log-out"
        title="Log Out"
        class="log cursor-pointer bg-slate-800 hover:bg-slate-700 rounded"
        on:click={() => router.goto("/auth/logout")}
      />
    </div>
  {/if}
</nav>
