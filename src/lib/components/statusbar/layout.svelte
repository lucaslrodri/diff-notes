<script lang="ts">
  import { settings, layout } from "$lib/stores";
  import { afterUpdate } from "svelte";
  import Tooltip from "../tooltip.svelte";

  import { type LayoutSchema } from "$lib/schemas";

  const layouts: LayoutSchema[] = ["vertical", "horizontal"];

  afterUpdate(async () => {
    $settings.set("layout", $layout);
    await $settings.save();
  });
</script>

<div class="flex flex-row">
  {#each layouts as layout_item (layout_item)}
    <Tooltip
      content={layout_item == "vertical"
        ? "Select vertical layout"
        : "Select horizontal layout"}
      class="flex items-stretch"
      position="statusbar"
      offset={-0.25}
      alignment="center"
    >
      <button
        class="btn-group-statusbar"
        class:bg-gray-200={$layout == layout_item}
        class:hover:bg-gray-150={$layout != layout_item}
        on:click={() => ($layout = layout_item)}
      >
        {#if layout_item == "vertical"}
          <!-- lucide:rows-2 -->
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="size-5 text-gray-500"
            viewBox="0 0 24 24"
          >
            <g
              fill="none"
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="1.35"
              ><rect width="18" height="18" x="3" y="3" rx="2" /><path
                d="M3 12h18"
              /></g
            >
          </svg>
        {:else}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="size-5 text-gray-500"
            viewBox="0 0 24 24"
          >
            <g
              fill="none"
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="1.35"
              ><rect width="18" height="18" x="3" y="3" rx="2" /><path
                d="M12 3v18"
              /></g
            >
          </svg>
        {/if}
      </button>
    </Tooltip>
  {/each}
</div>
