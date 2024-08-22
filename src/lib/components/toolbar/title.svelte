<script lang="ts">
  import { data, layout, title, autosave } from "$lib/stores";

  import Tooltip from "$lib/components/tooltip.svelte";

  import { type SideSchema } from "$lib/schemas";

  export let side: SideSchema;

  let placeholder: string = "";

  $: {
    $data.set("title", $autosave ? $title : { left: "", right: "" });
  }

  $: {
    if ($layout === "vertical") {
      placeholder = side === "left" ? "Top" : "Bottom";
    } else {
      placeholder = side;
    }
  }
</script>

<Tooltip
  content="Edit title of this view"
  alignment="center"
  position="toolbar"
  class="flex-1 flex justify-center"
>
  <input
    class="text-gray-700 focus:bg-white border-gray-50 hover:border-gray-200 focus:border-gray-200 border w-full font-semibold text-center mx-2 py-1 px-2.5 bg-transparent focus:bg-transparent placeholder:capitalize focus:outline-none hover:bg-transparent rounded cursor-default title-input placeholder:text-gray-500 truncate"
    {placeholder}
    id={`title-${side}`}
    autocomplete="false"
    spellcheck="false"
    bind:value={$title[side]}
  />
</Tooltip>
