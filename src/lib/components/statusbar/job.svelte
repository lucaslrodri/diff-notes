<script lang="ts">
  import Spinner from "$lib/components/spinner.svelte";
  import Tooltip from "$lib/components/tooltip.svelte";

  export let isWorking;
</script>

<Tooltip
  content={isWorking ? "Differentiation algorithm is currently processing" : "Differentiation process has completed"}
  position="statusbar"
  offset={0.2}
  alignment="center"
  class="label-statusbar"
>
  {#if isWorking}
    <!-- svg-spinners:3-dots-bounce -->
    <Spinner class="size-4 mr-2 text-gray-500" />
    <span>In Progress...</span>
  {:else}
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="size-6 text-app-dark-green mr-2"
      viewBox="0 0 24 24"
    >
      <path
        fill="currentColor"
        d="m9.55 18l-5.7-5.7l1.425-1.425L9.55 15.15l9.175-9.175L20.15 7.4z"
      />
    </svg>
    <span>Done</span>
  {/if}
</Tooltip>
