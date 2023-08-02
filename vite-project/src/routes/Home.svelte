<script>
  import { shortcut } from "$lib/shortcut";
  import { t } from "../locale/i18n";

  import Save from "carbon-icons-svelte/lib/Save.svelte";
  import Reset from "carbon-icons-svelte/lib/Reset.svelte";
  import FolderAdd from "carbon-icons-svelte/lib/FolderAdd.svelte";
  import FolderOff from "carbon-icons-svelte/lib/FolderOff.svelte";
  import TrashCan from "carbon-icons-svelte/lib/TrashCan.svelte";
  import Settings from "carbon-icons-svelte/lib/Settings.svelte";
  import ListBoxes from "carbon-icons-svelte/lib/ListBoxes.svelte";
  import Grid from "carbon-icons-svelte/lib/Grid.svelte";

  import {
    Button,
    ButtonSet,
    ContextMenu,
    ContextMenuDivider,
    ContextMenuGroup,
    ContextMenuOption,
    InlineLoading,
    Loading,
    Modal,
    OverflowMenu,
    OverflowMenuItem,
    Popover,
    TextInput,
    TooltipDefinition,
  } from "carbon-components-svelte";
  import { flip } from "svelte/animate";
  import { fade, fly } from "svelte/transition";
  import IcnTest from "$icons/icn_test.svelte";
  import ServerScan from "$lib/ServerScan.svelte";
  import LanguageSelect from "$lib/LanguageSelect.svelte";
  import DateTime from "$lib/DateTime.svelte";

  let iconSize = 80;

  let createFolderModal = false;
  let renameFolderModal = false;
  let delteFolderModal = false;

  // let open = false;
  let ref = null;
  let searchItem;
  let selectedIndex;
  let folders = [];
  let folderRefs = [];
  let selected = [];
  let hidden = [];

  let filteredFolders = [];

  let renameTarget = "";

  function renameFolder() {
    renameTarget = folders[selectedIndex].name;
    renameFolderModal = true;
  }

  function confirmRename() {
    folders[selectedIndex].name = renameTarget;
    folders = [...folders];
    filteredFolders = [...folders];
    renameFolderModal = false;
  }

  function checkSearching() {
    filteredFolders = folders.filter((folder) => folder.name.toLowerCase().includes(searchItem.toLowerCase()));
  }

  function UpdateSelectdIndex(idx) {
    selectedIndex = idx;
    selectedIndex = folders.findIndex((obj) => obj.id == selectedIndex);
  }

  let folderName = "";

  let idx = 0;
  function handleADD(folderName) {
    if (folderName == "") folderName = "new folder";
    createFolderModal = false;
    idx++;
    folders.push({ id: idx, name: folderName });
    folders = [...folders];
    filteredFolders = [...folders];
  }

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

  function handleSelectAll() {
    if (selected.length == 0) {
      folders.forEach((element) => {
        selected.push(element.id);
      });
    } else selected = [];
    selected = [...selected];
  }

  function initFolderDelete() {
    delteFolderModal = true;
  }

  function handleDelete() {
    if (selectedIndex == undefined) return;
    folders.splice(selectedIndex, 1);
    folders = [...folders];
    filteredFolders = [...folders];
    delteFolderModal = false;
  }

  function handleHideSelected() {
    selected.forEach((idx) => {
      if (folders[idx - 1].id == idx) {
        hidden.push(folders[idx - 1]);
      }
    });

    // selected.forEach((idx) => {
    //   folders = folders.filter((folder) => folder.id != idx);
    // });

    // selected = [];

    hidden = [...hidden];
    // folders = [...folders];
  }
  function handleDeleteSelected() {
    selected.forEach((idx) => {
      folders = folders.filter((folder) => folder.id != idx);
    });
    selected = [];
  }

  function handleHide() {
    if (selectedIndex == undefined) return;
    hidden.push(folders[selectedIndex]);
    folders.splice(selectedIndex, 1);
    folders = [...folders];
    hidden = [...hidden];
    filteredFolders = [...folders];
  }

  function handleUnhide(item) {
    folders.push(item);
    folders = [...folders];
    filteredFolders = [...folders];
    hidden = hidden.filter((hiddenitem) => hiddenitem.id != item.id);
    hidden = [...hidden];
  }

  function handleUnhideAll() {
    hidden.forEach((element) => {
      handleUnhide(element);
    });
  }

  function handleDeleteAll() {
    folders = [];
    filteredFolders = [];
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

<div class="relative h-[50rem] overflow-auto rounded px-2">
  <h1>{@html $t("greeting")}</h1>

  <div class="flex justify-between w-full mt-2">
    <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white" on:click={() => (createFolderModal = true)}>Create <FolderAdd /></button>
    <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white">Restore <Reset /></button>
    <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white">Save <Save /> </button>
    <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white">Closure <FolderOff /></button>
    <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white">Delete <TrashCan /></button>
    <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white">Settings <Settings /></button>
  </div>
  <input class="border text-black h-8 rounded my-2 w-1/2" placeholder="Search..." bind:value={searchItem} on:input={checkSearching} />

  <!-- <button class="border w-20 rounded press" on:click={handleRemove}> Remove </button> -->
  <!-- <button class="border w-20 rounded press" on:click={() => (createFolderModal = true)}> Add </button> -->

  <Modal
    bind:open={renameFolderModal}
    modalHeading="Rename folder."
    primaryButtonText="Confirm"
    secondaryButtonText="Cancel"
    selectorPrimaryFocus="#rename"
    on:click:button--secondary={() => (renameFolderModal = false)}
    on:open
    on:close
    on:submit={confirmRename}>
    <br />
    <TextInput id="rename" labelText="New folder name" placeholder="Rename folder..." bind:value={renameTarget} />
  </Modal>

  <Modal
    bind:open={createFolderModal}
    modalHeading="Create a new folder."
    primaryButtonText="Confirm"
    secondaryButtonText="Cancel"
    selectorPrimaryFocus="#folder-name"
    on:click:button--secondary={() => (createFolderModal = false)}
    on:open
    on:close
    on:submit={() => {
      handleADD(folderName);
    }}>
    <br />
    <TextInput id="folder-name" bind:value={folderName} labelText="Folder name" placeholder="Enter folder name..." />
  </Modal>

  <Modal
    danger
    bind:open={delteFolderModal}
    modalHeading="Delete all instances"
    primaryButtonText="Delete"
    secondaryButtonText="Cancel"
    on:click:button--secondary={() => (delteFolderModal = false)}
    on:open
    on:close
    on:submit={handleDelete}>
    <p>This is a permanent action and cannot be undone.</p>
  </Modal>

  <button class="border w-20 rounded press" on:click={handleUnhideAll}>unhide all</button>
  <button class="border w-20 rounded press" on:click={handleDeleteAll}>Delete all</button>

  <button
    use:shortcut={{ control: true, code: "Period" }}
    on:click={() => {
      handleADD("new folder");
    }} />

  {#if selected.length > 0}
    <button class="border w-28 rounded press" on:click={handleDeleteSelected}>Delete Selection</button>
    <button class="border w-28 rounded press" on:click={handleHideSelected}>Hide Selection</button>
  {/if}

  <br />
  <div class="flex justify-end mt-2 border rounded p-1">
    <div>
      <input class="mx-1" type="checkbox" on:click={() => handleSelectAll()} /> select all
      <input class="mx-1" type="checkbox" on:click={() => handleInvertSelectAll()} /> invert selection
    </div>
  </div>
  <div class="mt-2 flex flex-wrap">
    {#each filteredFolders as folder (folder.id)}
      <a
        href="#/Company"
        bind:this={folderRefs[folder.id]}
        id={folder.id}
        class="relative border-gray-200 press rounded mr-1 mb-1"
        animate:flip={{ duration: 1000 }}
        in:fly={{ x: 50 }}
        out:fly={{ x: 50 }}>
        <div class="absolute top-0.5 left-0.5">
          <input type="checkbox" checked={selected.includes(folder.id)} on:change={() => handleAddSelection(folder.id)} />
        </div>

        <div id={folder.id} class="flex justify-center">
          <img id={folder.id} src="/public/icons/folder.svg" style="width: {iconSize}px; height: {iconSize}px;" alt="" />
        </div>
        <p id={folder.id} class="truncate text-xs bg-[#c2ff00] rounded text-black">
          <!-- {folder.id}. -->
          {folder.name}
        </p>
      </a>
    {/each}
  </div>
  <p class="text-xs">
    {#each hidden as item (item.id)}
      <button bind:this={folderRefs[item.id]} id={item.id} class="relative opacity-25 border-gray-200 press rounded w-20 mr-1 mb-1">
        <div class="absolute top-0.5 left-0.5">
          <input type="checkbox" checked={selected.includes(item.id)} on:change={() => handleAddSelection(item.id)} />
        </div>

        <div id={item.id} class="flex justify-center">
          <img id={item.id} src="/public/icons/folder.svg" class="w-14" alt="" />
        </div>
        <p id={item.id} class=" text-xs bg-[#c2ff00] rounded text-black">
          {item.id}.
          {item.name}
        </p>
      </button>

      {JSON.stringify(item)}
      <button
        class="press border rounded"
        on:click={() => {
          handleUnhide(item);
        }}>unhide</button> <br />
    {/each}

    <br />

    <!-- selectedidx: {selectedIndex} -->
  </p>

  <!-- <div class="border w-52 flex flex-wrap">
    <div class="border mr-2">
      folders
      {#each folders as item}
        <br /> {item.id}
      {/each}
    </div>

    <div class="border mr-2">
      hidden
      {#each hidden as item}
        <br /> {item.id}
      {/each}
    </div>
    <div class="border mr-2">
      selected
      {#each selected as item}
        <br /> {item}
      {/each}
    </div>
  </div> -->

  <ContextMenu target={folderRefs} on:open={(e) => UpdateSelectdIndex(e.detail.id)}>
    <ContextMenuOption indented labelText="Rename" shortcutText="Ctrl + R" icon={IcnTest} on:click={renameFolder} />
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
    <ContextMenuOption indented kind="danger" labelText="Delete" on:click={initFolderDelete} />
  </ContextMenu>

  <!-- 
  <ContextMenu on:open={(e) => UpdateSelectdIndex(e.detail.id)}>
  <ContextMenuOption indented labelText="Unhide All" shortcutText="Ctrl + U" icon={IcnTest} on:click={handleUnhideAll} />
  </ContextMenu> 
-->

  <!-- <Loading /> -->
</div>

<!-- absolute ml-16 inset-x-10 bottom-1 ml-8 -->
<div class="flex flex-col mt-2 rounded">
  <div class=" w-full flex justify-between text-xs py-2">
    <div>
      {folders.length}
      {folders.length > 1 ? "folders" : "folder"}
    </div>

    <div class="flex">
      <button class="mr-2 press border rounded"> <Grid /></button>
      <button class=" press border rounded"> <ListBoxes /></button>
    </div>
  </div>

  <div class="flex justify-between">
    <div class="rounded border px-2">
      <ServerScan />
    </div>

    <div class="  rounded text-xs border flex justify-between">
      {iconSize}
      <input type="range" class="ml-2" step="20" min="40" max="240" bind:value={iconSize} />

      <DateTime />
      <div bind:this={ref} style:position="relative">
        <LanguageSelect />
      </div>
    </div>
  </div>
</div>

<style>
  .border {
    border-style: solid;
    border-width: 0.5px;
  }
</style>
