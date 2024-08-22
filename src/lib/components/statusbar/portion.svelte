<script lang="ts">
  import { portion, highlights } from "$lib/stores";
  import Tooltip from "$lib/components/tooltip.svelte";

  import { forceAlignScrollWithCurentChunk } from "$lib/stores";
  import { shortcut } from "$lib/shortcut";

  let portionIndicator = "";
  $: {
    if ($portion <= 0) {
      portionIndicator = `1`;
    } else {
      portionIndicator = `${$portion + 1}`;
    }
  }
</script>

<Tooltip
  content={`Chunk ${portionIndicator} of ${$highlights.left.length} • Click to align scroll with current chunk (F5)`}
  alignment="center"
  offset={-0.25}
  position="statusbar"
  class="flex items-center justify-stretch min-w-20"
>
  <button
    class="hover:bg-gray-150 h-full w-full text-gray-700 text-sm"
    use:shortcut={{code:"F5"}}
    on:click={() => {
      $forceAlignScrollWithCurentChunk = {left: true, right: true};
    }}
  >
    <span>Pos. {portionIndicator}</span>
  </button>
</Tooltip>
