<script>
  import { ContextMenu, ContextMenuDivider, ContextMenuGroup, ContextMenuOption } from "carbon-components-svelte";
  import { flip } from "svelte/animate";
  import { fade, fly } from "svelte/transition";
  import IcnTest from "$icons/icn_test.svelte";

  let folders = [];
  // let target1;
  // let target2;

  let deleteIndex;
  function handleRemove() {
    if (deleteIndex == undefined) return;
    folders = folders.filter((item) => item.id != deleteIndex);
    deleteIndex = "";
  }

  let idx = 0;
  function handleADD() {
    idx++;
    folders.push({ id: idx, name: "newItem" });
    folders = [...folders];
  }
</script>

<input class="text-black rounded mt-16" type="number" bind:value={deleteIndex} />
<button class="border w-20 rounded press" on:click={handleRemove}> Remove </button>
<button class="border w-20 rounded press" on:click={handleADD}> Add </button>

<div class="mt-2">
  {#each folders as thing (thing.id)}
    <div id=" {thing.id}" class="relative border press rounded w-20 mr-1 mb-1" animate:flip={{ duration: 1000 }} in:fly={{ x: 50 }} out:fly={{ x: 50 }}>
      <div class="absolute top-0 left-0"><input type="checkbox" /></div>

      <div class="flex justify-center">
        <img src="icons/folder.svg" class="w-14" alt="" />
      </div>
      <p class="truncate text-xs">
        {thing.id}.
        {thing.name}
      </p>
    </div>
  {/each}
</div>

<!-- <div class="flex">
    <div class="w-20 h-20 rounded bg-blue-400 mr-2" bind:this={target1}>target1</div>
    <div class="w-20 h-20 rounded bg-blue-400 mr-2" bind:this={target2}>target2</div>
  </div> -->

<ContextMenu target={[]} on:open={(e) => console.log(e.detail)}>
  <ContextMenuOption indented labelText="Copy" shortcutText="Ctrl + C" icon={IcnTest} />
  <ContextMenuOption indented labelText="Hide" shortcutText="Ctrl + H" icon={IcnTest} />
  <ContextMenuDivider />
  <ContextMenuOption indented labelText="Export as">
    <ContextMenuGroup labelText="Export options">
      <ContextMenuOption id="pdf" labelText="PDF" />
      <ContextMenuOption id="txt" labelText="TXT" />
      <ContextMenuOption id="mp3" labelText="MP3" />
    </ContextMenuGroup>
  </ContextMenuOption>
  <ContextMenuDivider />
  <ContextMenuOption selectable labelText="Remove metadata" />
  <ContextMenuDivider />
  <ContextMenuGroup labelText="Style options">
    <ContextMenuOption id="0" labelText="Font smoothing" selected />
    <ContextMenuOption id="1" labelText="Reduce noise" />
    <ContextMenuOption id="2" labelText="Auto-sharpen" />
  </ContextMenuGroup>
  <ContextMenuDivider />
  <ContextMenuOption indented kind="danger" labelText="Delete" />
</ContextMenu>

<style>
  .border {
    border-style: solid;
    border-width: 1px;
  }
</style>
