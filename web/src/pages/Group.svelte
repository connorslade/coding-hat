<script>
  import { router } from "tinro";
  import { Bolt } from "svelte-heros-v2";
  import Content from "../lib/Content.svelte";
  import Header from "../lib/Header.svelte";

  const BADGE_TEXT = ["Not Started", "In Progress", "Complete"];
  const BADGE_COLORS = [
    ["#475569", "#F1F5F9"],
    ["#CA8A04", "#FEF9C3"],
    ["#059669", "#D1FAE5"],
  ];

  function statusBadgeColor(status) {
    let color = BADGE_COLORS[status];
    return `background: ${color[0]};color: ${color[1]};`;
  }

  function complete() {
    let done = group.problems.filter((x) => x.status > 1).length;
    let total = group.problems.length;

    if (done === total) return "Complete";
    return `${done}/${total}`;
  }

  export let id;
  let group = {
    problems: [
      {
        id: "201517",
        name: "Even A",
        language: "java",
        status: 0,
      },
      {
        id: "69",
        name: "Test Problem",
        language: "java",
        status: 2,
      },
    ],
  };
</script>

<Header page={"group"} />

<Content>
  <!-- DESCRIPTION -->

  <h1 class="text-2xl">Test Group</h1>
  <p>This is a test group</p>

  <!-- END DESCRIPTION -->

  <br />
  <p class="text-2xl">
    Problems ({complete()})
  </p>
  <hr class="mb-3" />

  {#each group.problems as p}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="entry" on:click={() => router.goto(`/p/${p.id}`)}>
      <div
        class="flex items-center bg-slate-800 hover:bg-slate-700 rounded p-1.5 cursor-pointer"
      >
        <div>
          <p class="text-lg">{p.name}</p>
          <div class="flex items-center text-slate-400">
            <Bolt size="16" variation="solid" class="mr-2" />
            <p>{p.language}</p>
          </div>
        </div>
        <div
          class="ml-auto text-xs py-1 px-2.5 font-bold bg-slate-600 text-slate-100 rounded"
          style={statusBadgeColor(p.status)}
        >
          {BADGE_TEXT[p.status]}
        </div>
      </div>
      <div class="border border-slate-700 rounded m-1" />
    </div>
  {/each}
</Content>

<style>
  .entry:last-child .border {
    display: none;
  }
</style>
