<script lang="ts">
  import Tooltip from "$lib/components/tooltip.svelte";

  import { layout } from "$lib/stores";
  import { shortcut } from "$lib/shortcut";

  import { type SideSchema } from "$lib/schemas";

  export let side: SideSchema;
  export let onPaste;
</script>

<Tooltip
  content={`Overwrite this view with clipboard content (F${side === "left" ? "1" : "2"})`}
  offset={$layout === "horizontal" && side === "right" ? 0.75 : 0.25}
  alignment={side === "right" && $layout === "horizontal" ? "right" : "left"}
  position="toolbar"
>
  <button
    class="btn-toolbar"
    on:click={onPaste}
    use:shortcut={{
      code: `F${side === "left" ? "1" : "2"}`,
    }}
  >
    <svg xmlns="http://www.w3.org/2000/svg" class="h-full" viewBox="0 0 16 16">
      <path
        fill="currentColor"
        d="M4.085 2H3.5A1.5 1.5 0 0 0 2 3.5v10A1.5 1.5 0 0 0 3.5 15h2.612v-1H3.5a.5.5 0 0 1-.5-.5v-10a.5.5 0 0 1 .5-.5h.585A1.5 1.5 0 0 0 5.5 4h3a1.5 1.5 0 0 0 1.415-1h.585a.5.5 0 0 1 .5.5V5h1V3.5A1.5 1.5 0 0 0 10.5 2h-.585A1.5 1.5 0 0 0 8.5 1h-3a1.5 1.5 0 0 0-1.415 1M5 2.5a.5.5 0 0 1 .5-.5h3a.5.5 0 0 1 0 1h-3a.5.5 0 0 1-.5-.5M8.5 6A1.5 1.5 0 0 0 7 7.5v6A1.5 1.5 0 0 0 8.5 15h4a1.5 1.5 0 0 0 1.5-1.5v-6A1.5 1.5 0 0 0 12.5 6zM8 7.5a.5.5 0 0 1 .5-.5h4a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-.5.5h-4a.5.5 0 0 1-.5-.5z"
      />
    </svg>
  </button>
</Tooltip>
