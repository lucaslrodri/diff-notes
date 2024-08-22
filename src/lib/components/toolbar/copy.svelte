<script lang="ts">
  import { writeText } from "@tauri-apps/api/clipboard";

  import { content, layout } from "$lib/stores";

  import Tooltip from "$lib/components/tooltip.svelte";
  import { shortcut } from "$lib/shortcut";

  import { type SideSchema } from "$lib/schemas";

  export let side: SideSchema;

  let copied = false;

  let timeOut: ReturnType<typeof setTimeout> | null = null;

  let onCopy = async () => {
    if ($content[side]?.length > 0 && !copied) {
      writeText($content[side])
        .then(() => {
          copied = true;
          timeOut = setTimeout(() => {
            copied = false;
          }, 500);
        })
        .catch(() => {
          copied = false;
          console.log("Failed to copy to clipboard");
        });
    }
  };
</script>

<Tooltip
  content={`Copy this view to clipboard (Ctrl+${side === "left" ? "1" : "2"})`}
  offset={$layout === "horizontal" && side === "right" ? 0.75 : 0.25}
  alignment={side === "right" && $layout === "horizontal" ? "right" : "left"}
  position="toolbar"
>
  <button
    class={copied ? "label-toolbar" : "btn-toolbar"}
    disabled={copied}
    use:shortcut={{ ctrl: true, code: `Digit${side === "left" ? "1" : "2"}` }}
    on:click={onCopy}
  >
    {#if copied}
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="w-full text-app-dark-green"
        viewBox="0 0 24 24"
      >
        <path
          fill="currentColor"
          d="m9.55 18l-5.7-5.7l1.425-1.425L9.55 15.15l9.175-9.175L20.15 7.4z"
        />
      </svg>
    {:else}
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-full"
        viewBox="0 0 16 16"
      >
        <path
          fill="currentColor"
          d="M6.999 1a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2H12a2 2 0 0 0 2-2V5.5h-.002V4.414a1.5 1.5 0 0 0-.439-1.06l-1.914-1.915A1.5 1.5 0 0 0 10.585 1zM12 12H6.999a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h3v1.5a1.5 1.5 0 0 0 1.5 1.5h1.5v1.061H13V11a1 1 0 0 1-1 1m.791-8h-1.293a.5.5 0 0 1-.5-.5V2.207zM3 4a1 1 0 0 1 1-1v8a3 3 0 0 0 3 3h5a1 1 0 0 1-1 1H6.79A3.79 3.79 0 0 1 3 11.21z"
        />
      </svg>
    {/if}
  </button>
</Tooltip>
