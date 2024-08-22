<script lang="ts">
  import { afterUpdate } from "svelte";

  import { settings, mode } from "$lib/stores";
  import Tooltip from "$lib/components/tooltip.svelte";

  const modes = [
    {
      identifier: "s",
      label: "Semantic",
      tooltip: "Highlight differences in human-readable chunks",
    },
    {
      identifier: "w",
      label: "Word",
      tooltip: "Highlight differences in word chunks",
    },
  ];

  afterUpdate(async () => {
    $settings.set("mode", $mode);
    await $settings.save();
  });
</script>

<div class="flex flex-row">
  {#each modes as mode_item (mode_item.identifier)}
    <Tooltip
      content={mode_item.tooltip}
      position="statusbar"
      offset={0}
      alignment="center"
      class="flex h-full items-stretch"
    >
      <button
        class="btn-group-statusbar"
        class:bg-gray-200={$mode == mode_item.identifier}
        class:hover:bg-gray-150={$mode != mode_item.identifier}
        on:click={() => ($mode = mode_item.identifier)}
        >{mode_item.label}</button
      >
    </Tooltip>
  {/each}
</div>
