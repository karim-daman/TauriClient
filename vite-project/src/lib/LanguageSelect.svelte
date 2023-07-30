<script>
  import { Popover } from "carbon-components-svelte";
  import { t, locale, locales } from "../locale/i18n";

  let open = false;
  let ref = null;
</script>

<div bind:this={ref} style:position="relative">
  <button
    class="press"
    on:click={() => {
      open = !open;
    }}>
    <img src="1x1/{$locale}.svg" class="rounded-full w-3 m-2" alt="" />
  </button>
  <Popover
    bind:open
    align="top"
    on:click:outside={({ detail }) => {
      open = ref.contains(detail.target);
    }}>
    <div style="padding: var(--cds-spacing-01)">
      {#each locales as l}
        <button
          on:click={() => {
            $locale = l;
            open = !open;
          }}
          class="w-24 hover:bg-blue-600 hover:text-white py-1 press flex px-2 rounded my-1">
          <img src="1x1/{l}.svg" class="rounded-full w-4 mr-4" alt="" />
          <div>{l}</div>
        </button>
      {/each}
    </div>
  </Popover>
</div>
