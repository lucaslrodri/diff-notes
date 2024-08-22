<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import { content, first, last } from "$lib/stores";
  import { shortcut } from "$lib/shortcut";

  import Tooltip from "$lib/components/tooltip.svelte";


  const onDelete = async () => {
    $content.left = "";
    $content.right = "";
    $last = true;
    $first = false;
    await invoke("push", {
      state: { left: "", right: "", caret: 0, side: 'l' },
    });
  };


</script>

<Tooltip
  content="Discard all texts"
  alignment="right"
  position="statusbar"
  offset={0.8}
  class="flex item-strech"
>
  <button
    class="pr-2.5 pl-2 text-sm flex items-center justify-center bg-app-dark-red hover:bg-app-dark-red/90"
    on:click={onDelete}
    use:shortcut={{ctrl: true, code: "KeyD"}}
  >
    <!--Trash-->
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="size-4 text-white"
      viewBox="0 0 24 24"
    >
      <path
        fill="currentColor"
        d="M9 3v1H4v2h1v13a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V6h1V4h-5V3zm0 5h2v9H9zm4 0h2v9h-2z"
      />
    </svg>
  </button>
</Tooltip>
