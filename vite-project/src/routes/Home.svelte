<script>
  import { Button, ContextMenu, ContextMenuDivider, ContextMenuGroup, ContextMenuOption, InlineLoading, Loading, OverflowMenu, OverflowMenuItem, Popover } from "carbon-components-svelte";
  import { flip } from "svelte/animate";
  import { fade, fly } from "svelte/transition";
  import IcnTest from "$icons/icn_test.svelte";
  import ServerScan from "$lib/ServerScan.svelte";
  import LanguageSelect from "$lib/LanguageSelect.svelte";

  let open = false;
  let ref = null;

  let folders = [];
  let folderRefs = [];
  let selected = [];
  let hidden = [];

  function UpdateSelectdIndex(idx) {
    selectedIndex = idx;
    selectedIndex = folders.findIndex((obj) => obj.id == selectedIndex);
  }

  let idx = 0;
  function handleADD() {
    idx++;
    folders.push({ id: idx, name: "newItem" });
    folders = [...folders];
  }

  let selectedIndex;
  function handleRemove() {
    if (selectedIndex == undefined) return;
    folders = folders.filter((item) => item.id != selectedIndex);
    selectedIndex = "";
  }

  function handleAddSelection(idx) {
    const index = selected.indexOf(idx);
    if (index !== -1) {
      selected.splice(index, 1);
      selected = [...selected];
      console.log(selected);
      return;
    }

    selected.push(idx);
    selected = [...selected];
  }

  function handleInvertSelectAll() {
    folders.forEach((folder) => {
      handleAddSelection(folder.id);
    });
  }

  function handleSelectAll() {}

  function handleDeleteSelected() {
    if (selectedIndex == undefined) return;
    folders.splice(selectedIndex, 1);
    folders = [...folders];
  }

  function handleHide() {
    if (selectedIndex == undefined) return;
    hidden.push(folders[selectedIndex]);
    folders.splice(selectedIndex, 1);
    folders = [...folders];
    hidden = [...hidden];
  }

  function handleUnhide(item) {
    folders.push(item);
    folders = [...folders];
    hidden = hidden.filter((hiddenitem) => hiddenitem.id != item.id);
    hidden = [...hidden];
  }

  function handleUnhideAll() {
    hidden.forEach((element) => {
      handleUnhide(element);
    });
  }

  let time;

  function updateClock() {
    const currentTime = new Date();
    const hours = currentTime.getHours();
    const minutes = currentTime.getMinutes();
    const seconds = currentTime.getSeconds();
    const formattedTime = `${hours % 12}:${minutes < 10 ? "0" : ""}${minutes}:${seconds < 10 ? "0" : ""}${seconds} ${hours >= 12 ? "PM" : "AM"}`;

    // Replace "clock" with the ID or class of the element where you want to display the time
    time = formattedTime;
  }
  // Update the clock every second (1000 milliseconds)
  setInterval(updateClock, 1000);
  // Call it once initially to avoid a delay in the first update
  updateClock();
</script>

<div class="">
  <input class="border text-black rounded mt-2" type="number" bind:value={selectedIndex} />
  <button class="border w-20 rounded press" on:click={handleRemove}> Remove </button>
  <button class="border w-20 rounded press" on:click={handleADD}> Add </button>
  <button class="border w-20 rounded press" on:click={handleUnhideAll}>unhide all</button>

  <br />
  <div class="flex justify-end mt-2 border rounded p-1">
    <input class="mx-1" type="checkbox" on:click={() => handleSelectAll()} /> select all
    <input class="mx-1" type="checkbox" on:click={() => handleInvertSelectAll()} /> invert selection
  </div>
  <div class="mt-2 flex flex-wrap">
    {#each folders as folder (folder.id)}
      <button
        bind:this={folderRefs[folder.id]}
        id={folder.id}
        class="relative border border-gray-200 press rounded w-20 mr-1 mb-1"
        animate:flip={{ duration: 1000 }}
        in:fly={{ x: 50 }}
        out:fly={{ x: 50 }}>
        <div class="absolute top-0.5 left-0.5">
          <input type="checkbox" checked={selected.includes(folder.id)} on:change={() => handleAddSelection(folder.id)} />
        </div>

        <div id={folder.id} class="flex justify-center">
          <img id={folder.id} src="icons/folder.svg" class="w-14" alt="" />
        </div>
        <p id={folder.id} class="truncate text-xs bg-[#c2ff00] text-black">
          {folder.id}.
          {folder.name}
        </p>
      </button>
    {/each}
  </div>
  <p class="t3ext-xs">
    selected: {selected.toString()}
    <br />
    hidden: <br />
    {#each hidden as item}
      {JSON.stringify(item)}
      <button
        class="press border rounded"
        on:click={() => {
          handleUnhide(item);
        }}>unhide</button> <br />
    {/each}

    <br />
    selectedidx: {selectedIndex}
  </p>

  <ContextMenu target={folderRefs} on:open={(e) => UpdateSelectdIndex(e.detail.id)}>
    <ContextMenuOption indented labelText="Copy" shortcutText="Ctrl + C" icon={IcnTest} />
    <ContextMenuOption indented labelText="Hide" shortcutText="Ctrl + H" icon={IcnTest} on:click={handleHide} />
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
    <ContextMenuOption indented kind="danger" labelText="Delete" on:click={handleDeleteSelected} />
  </ContextMenu>

  <!-- 
  <ContextMenu on:open={(e) => UpdateSelectdIndex(e.detail.id)}>
  <ContextMenuOption indented labelText="Unhide All" shortcutText="Ctrl + U" icon={IcnTest} on:click={handleUnhideAll} />
  </ContextMenu> 
-->

  <!-- <Loading small /> -->

  <div class=" absolute inset-x-16 bottom-1 ml-12 flex justify-between mt-2 rounded p-1">
    <div class="rounded border px-2">
      <ServerScan />
    </div>
    <!-- <div class="bg-red-500 w-8 rounded px-3">2</div>
    <div class="bg-red-500 w-8 rounded px-3">3</div>
    <div class="bg-red-500 w-8 rounded px-3">4</div> -->
    <div class=" w-52 rounded px-3 text-xs border flex">
      <div class="mr-4 mt-2">
        {new Date().getMonth() + 1 + "/" + new Date().getDate() + "/" + new Date().getFullYear()} -
        {time}
      </div>

      <div bind:this={ref} style:position="relative">
        <!-- <button class="press" on:click={() => (open = !open)}>
          <img src="1x1/English.svg" class="rounded-full w-3 m-2" alt="" />
        </button> -->
        <!-- <Popover
          bind:open
          align="top"
          on:click:outside={({ detail }) => {
            console.log("on:click:outside");
            open = ref.contains(detail.target);
          }}>
          <div style="padding: var(--cds-spacing-01)">
            <button class="w-24 hover:bg-blue-600 hover:text-white py-1 press flex px-2 rounded my-1">
              <img src="1x1/English.svg" class="rounded-full w-4 mr-4" alt="" />
              <div>English</div>
            </button>
            <button class="w-24 hover:bg-blue-600 hover:text-white py-1 press flex px-2 rounded my-1">
              <img src="1x1/Français.svg" class="rounded-full w-4 mr-4" alt="" />
              <div>Français</div>
            </button>
            <button class="w-24 hover:bg-blue-600 hover:text-white py-1 press flex px-2 rounded my-1">
              <img src="1x1/Español.svg" class="rounded-full w-4 mr-4" alt="" />
              <div>Español</div>
            </button>
            <button class="w-24 hover:bg-blue-600 hover:text-white py-1 press flex px-2 rounded my-1">
              <img src="/public/1x1/sa.svg" class="rounded-full w-4 mr-4" alt="" />
              <div>Arabic</div>
            </button>
            <button class="w-24 hover:bg-blue-600 hover:text-white py-1 press flex px-2 rounded my-1">
              <img src="1x1/cn.svg" class="rounded-full w-4 mr-4" alt="" />
              <div>Chinese</div>
            </button>
          </div>
        </Popover> -->

        <LanguageSelect />
      </div>

      <!-- <OverflowMenu class="" flipped direction="top" size="sm">
        <div class="press flex p-2">
          <img src="1x1/English.svg" class="rounded-full w-5 mr-2" alt="" />
          <div>English</div>
        </div>
        <div class="press flex p-2">
          <img src="1x1/English.svg" class="rounded-full w-5 mr-2" alt="" />
          <div>English</div>
        </div>
        <div class="press flex p-2">
          <img src="1x1/English.svg" class="rounded-full w-5 mr-2" alt="" />
          <div>English</div>
        </div>
      </OverflowMenu> -->
    </div>
  </div>
</div>

<style>
  .border {
    border-style: solid;
    border-width: 1px;
  }
</style>
