<script>
  import Content from "../lib/Content.svelte";
  import Header from "../lib/Header.svelte";

  export let id;
  export let pid;
  console.log(id);
  let problem = null;
  // onMount(async () => {
  //   let res = await fetch(`/api/problem/${meta.params.id}`);
  //   let data = await res.json();
  //   console.log(data);
  //   problem = data;
  // });

  fetch(`/api/problem/${pid}`)
    .then((res) => res.json())
    .then((data) => {
      console.log(data);
      problem = data;
    });
</script>

<Header page={"problem"} />

<Content>
  {#if problem}
    <h1 class="font-bold underline text-2xl">{problem?.name}</h1>
    <p>{@html problem?.text}</p>
    <br />
    <hr />

    <p>TODO: Editor</p>
  {:else}
    <p>Loading...</p>
  {/if}
</Content>
