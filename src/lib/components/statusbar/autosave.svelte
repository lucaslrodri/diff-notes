<script lang="ts">
  import { afterUpdate } from "svelte";

  import { settings, autosave } from "$lib/stores";
  import Tooltip from "$lib/components/tooltip.svelte";

  afterUpdate(async () => {
    $settings.set("autosave", $autosave);
    await $settings.save();
  });
</script>

<Tooltip
  content={$autosave
    ? "Automatically save texts when the app closes"
    : "Texts are discarded when the app closes"}
  position="statusbar"
  offset={0.1}
  alignment="center"
  class="flex h-full items-stretch px-2 hover:bg-gray-100"
>
  <label class="inline-flex items-center cursor-pointer">
    <input type="checkbox" bind:checked={$autosave} class="sr-only peer" />
    <div
      class="relative w-9 h-5 bg-gray-200 peer-focus:outline-none peer-focus:ring-none rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-gray-500"
    ></div>
    <span class="ms-2 text-sm text-gray-700">AutoSave</span>
  </label>
</Tooltip>
