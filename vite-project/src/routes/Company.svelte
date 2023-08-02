<script>
  import { Breadcrumb, BreadcrumbItem } from "carbon-components-svelte";
  import { Accordion, AccordionItem } from "carbon-components-svelte";
  import { Folder } from "carbon-icons-svelte";
  import { flip } from "svelte/animate";
  import { fly } from "svelte/transition";

  import Save from "carbon-icons-svelte/lib/Save.svelte";
  import Reset from "carbon-icons-svelte/lib/Reset.svelte";
  import FolderAdd from "carbon-icons-svelte/lib/FolderAdd.svelte";
  import FolderOff from "carbon-icons-svelte/lib/FolderOff.svelte";
  import TrashCan from "carbon-icons-svelte/lib/TrashCan.svelte";
  import Settings from "carbon-icons-svelte/lib/Settings.svelte";
  import ListBoxes from "carbon-icons-svelte/lib/ListBoxes.svelte";
  import Grid from "carbon-icons-svelte/lib/Grid.svelte";

  import UserMultiple from "carbon-icons-svelte/lib/UserMultiple.svelte";
  import Delivery from "carbon-icons-svelte/lib/Delivery.svelte";

  const items = [
    { href: "/", text: "Dashboard" },
    { href: "/reports", text: "Annual reports" },
    { href: "/reports/2019", text: "2019" },
    { href: "/reports/2019/Dot", text: "Dot" },
    { href: "/reports/2019/Dot", text: "Dot" },
  ];

  let iconSize = 50;
  let folderRefs = [];
  let folders = [
    { id: "0", name: "Clients" },
    { id: "1", name: "Suppliers" },
    { id: "2", name: "Commercial" },
    { id: "3", name: "Delivary" },
    { id: "4", name: "Buyers" },
    { id: "5", name: "Employees" },
  ];

  let folders2 = [
    { id: "0", name: "Purchases" },
    { id: "1", name: "Sales" },
    { id: "2", name: "Producttion" },
    { id: "3", name: "Distibution" },
    { id: "4", name: "Assembly" },
    { id: "5", name: "After Sale" },
  ];

  let folders3 = [
    { id: "0", name: "Products" },
    { id: "1", name: "Lots" },
    { id: "2", name: "Deposit" },
    { id: "3", name: "Offers" },
  ];

  let folders4 = [
    { id: "0", name: "Payments" },
    { id: "1", name: "Deadlines" },
    { id: "2", name: "Charges" },
    { id: "3", name: "Banks" },
    { id: "4", name: "Accounts" },
  ];

  let folders5 = [
    { id: "0", name: "Stock" },
    { id: "1", name: "Sales" },
    { id: "2", name: "Purchases" },
    { id: "3", name: "Tiers" },
    { id: "4", name: "Finances" },
  ];
</script>

<div class="w-full border rounded p-2">
  <!-- <h1>{@html $t("greeting")}</h1> -->

  <Breadcrumb noTrailingSlash>
    {#each items as item, i}
      <BreadcrumbItem href={item.href} isCurrentPage={i === items.length - 1}>
        {item.text}
      </BreadcrumbItem>
    {/each}
  </Breadcrumb>
</div>

<div class="flex justify-between w-full mt-2">
  <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white">Create <FolderAdd /></button>
  <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white">Restore <Reset /></button>
  <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white">Save <Save /> </button>
  <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white">Closure <FolderOff /></button>
  <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white">Delete <TrashCan /></button>
  <button class="rounded h-8 w-28 border press flex justify-evenly pt-1.5 bg-blue-500 text-white">Settings <Settings /></button>
</div>

<Accordion>
  <AccordionItem open title="Tiers">
    <div class="mt-2 flex flex-wrap">
      {#each folders as folder (folder.id)}
        <a href="#/Company" class="relative press rounded mr-1 mb-1" in:fly={{ x: 50 }} out:fly={{ x: 50 }}>
          <!-- <div class="absolute z-10 top-0.5 left-0.5">
            <input type="checkbox" />
          </div> -->

          <div class="flex justify-center relative">
            <img src="/public/icons/folder.svg" style="width: {iconSize}px; height: {iconSize}px;" alt="" />
            <div class="absolute top-5">
              <Delivery size={24} />
            </div>
          </div>
          <p class="truncate text-xs rounded text-black">
            <!-- {folder.id}. -->
            {folder.name}
          </p>
        </a>
      {/each}
    </div>
  </AccordionItem>

  <AccordionItem open title="Activity">
    <div class="mt-2 flex flex-wrap">
      {#each folders2 as folder (folder.id)}
        <a href="#/Company" class="relative border border-gray-200 press rounded mr-1 mb-1" in:fly={{ x: 50 }} out:fly={{ x: 50 }}>
          <div class="absolute top-0.5 left-0.5">
            <input type="checkbox" />
          </div>

          <div class="flex justify-center relative">
            <img src="/public/icons/folder.svg" style="width: {iconSize}px; height: {iconSize}px;" alt="" />
            <div class="absolute top-5">
              <Delivery size={24} />
            </div>
          </div>
          <p class="truncate text-xs bg-[#c2ff00] rounded text-black">
            <!-- {folder.id}. -->
            {folder.name}
          </p>
        </a>
      {/each}
    </div>
  </AccordionItem>

  <AccordionItem open title="Products">
    <div class="mt-2 flex flex-wrap">
      {#each folders3 as folder (folder.id)}
        <a href="#/Company" class="relative border border-gray-200 press rounded mr-1 mb-1" in:fly={{ x: 50 }} out:fly={{ x: 50 }}>
          <div class="absolute top-0.5 left-0.5">
            <input type="checkbox" />
          </div>

          <div class="flex justify-center relative">
            <img src="/public/icons/folder.svg" style="width: {iconSize}px; height: {iconSize}px;" alt="" />
            <div class="absolute top-5">
              <Delivery size={24} />
            </div>
          </div>
          <p class="truncate text-xs bg-[#c2ff00] rounded text-black">
            <!-- {folder.id}. -->
            {folder.name}
          </p>
        </a>
      {/each}
    </div>
  </AccordionItem>

  <AccordionItem open title="Finances">
    <div class="mt-2 flex flex-wrap">
      {#each folders4 as folder (folder.id)}
        <a href="#/Company" class="relative border border-gray-200 press rounded mr-1 mb-1" in:fly={{ x: 50 }} out:fly={{ x: 50 }}>
          <div class="absolute top-0.5 left-0.5">
            <input type="checkbox" />
          </div>

          <div class="flex justify-center relative">
            <img src="/public/icons/folder.svg" style="width: {iconSize}px; height: {iconSize}px;" alt="" />
            <div class="absolute top-5">
              <Delivery size={24} />
            </div>
          </div>
          <p class="truncate text-xs bg-[#c2ff00] rounded text-black">
            <!-- {folder.id}. -->
            {folder.name}
          </p>
        </a>
      {/each}
    </div>
  </AccordionItem>

  <AccordionItem open title="Reports">
    <div class="mt-2 flex flex-wrap">
      {#each folders5 as folder (folder.id)}
        <a href="#/Company" class="relative border border-gray-200 press rounded mr-1 mb-1" in:fly={{ x: 50 }} out:fly={{ x: 50 }}>
          <div class="absolute top-0.5 left-0.5">
            <input type="checkbox" />
          </div>

          <div class="flex justify-center relative">
            <img src="/public/icons/folder.svg" style="width: {iconSize}px; height: {iconSize}px;" alt="" />
            <div class="absolute top-5">
              <Delivery size={24} />
            </div>
          </div>
          <p class="truncate text-xs bg-[#c2ff00] rounded text-black">
            <!-- {folder.id}. -->
            {folder.name}
          </p>
        </a>
      {/each}
    </div>
  </AccordionItem>
</Accordion>

<style>
  .border {
    border-style: solid;
    border-width: 0.5px;
  }
</style>
