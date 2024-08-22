<script lang="ts">
  import { layout } from "$lib/stores";
  import Tooltip from "$lib/components/tooltip.svelte";
  
  import { shortcut } from "$lib/shortcut";
  
  import { type SideSchema } from "$lib/schemas";

  export let side: SideSchema;
  export let onMove;

  let tooltipViewSideName: string = "right";

  $: {
    if (side === "left" && $layout === "horizontal") {
      tooltipViewSideName = "right";
    } else if (side === "right" && $layout === "horizontal") {
      tooltipViewSideName = "left";
    } else if (side === "left" && $layout === "vertical") {
      tooltipViewSideName = "bottom";
    } else {
      tooltipViewSideName = "top";
    }
  }
</script>

<Tooltip
  content={`Clone this view to the ${tooltipViewSideName} side (Ctrl+Alt+${side === "left" ? "Right" : "Left"})`}
  offset={$layout === "horizontal" && side === "right" ? 0.25 : 0.75}
  alignment={side === "right" && $layout === "horizontal" ? "left" : "right"}
  position="toolbar"
>
  <button
    class="btn-toolbar flex items-center justify-center"
    use:shortcut={{alt: true, ctrl: true, code: `Arrow${side === 'left'? 'Right' : 'Left'}`}}
    on:click={onMove}
  >
    <svg
      class="size-6 text-gray-500 p-0.5"
      version="1.1"
      viewBox="0 0 20 20"
      xmlns="http://www.w3.org/2000/svg"
    >
      {#if side === "left" && $layout === "horizontal"}
        <path
          stroke="none"
          fill="currentColor"
          d="m16.5 9.2962-3.0875-3.2748c-0.26814-0.28458-0.67153-0.36977-1.022-0.21582l-0.06798 0.03381c-0.3133 0.17001-0.51046 0.51179-0.51055 0.88505l-2.6e-5 1.7881-4.7911-7.7e-5c-0.51782-1.2e-5 -0.9376 0.44521-0.93763 0.99444l-4.09e-5 0.98467 0.00234 0.0746c0.03679 0.51895 0.44458 0.92005 0.93525 0.9199l4.7911-4.21e-4 -2.7e-5 1.7883c1.6e-4 0.88579 1.0098 1.3294 1.6005 0.70312l3.0876-3.2747c0.36603-0.38833 0.36605-1.0178 2.9e-5 -1.4062"
          stroke-width=".48281"
        />
        <rect
          x="1"
          y="1"
          width="18"
          height="18"
          rx="2"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
        />
        <path
          stroke="none"
          fill="currentColor"
          d="m1 1v18h9v-6.0137h-2.9785c-1.3336 4.16e-4 -2.3418-1.0453-2.4316-2.3125l-0.00195-0.03125-0.00391-0.12695v-1.0078c5.6e-5 -1.3374 1.033-2.4942 2.4375-2.4941h2.9785v-6.0137z"
        />
      {:else if side === "right" && $layout === "horizontal"}
        <g transform="matrix(-1,0,0,1,20,0)">
          <path
            stroke="none"
            fill="currentColor"
            d="m16.5 9.2962-3.0875-3.2748c-0.26814-0.28458-0.67153-0.36977-1.022-0.21582l-0.06798 0.03381c-0.3133 0.17001-0.51046 0.51179-0.51055 0.88505l-2.6e-5 1.7881-4.7911-7.7e-5c-0.51782-1.2e-5 -0.9376 0.44521-0.93763 0.99444l-4.09e-5 0.98467 0.00234 0.0746c0.03679 0.51895 0.44458 0.92005 0.93525 0.9199l4.7911-4.21e-4 -2.7e-5 1.7883c1.6e-4 0.88579 1.0098 1.3294 1.6005 0.70312l3.0876-3.2747c0.36603-0.38833 0.36605-1.0178 2.9e-5 -1.4062"
            stroke-width=".48281"
          />
          <rect
            x="1"
            y="1"
            width="18"
            height="18"
            rx="2"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          />
          <path
            stroke="none"
            fill="currentColor"
            d="m1 1v18h9v-6.0137h-2.9785c-1.3336 4.16e-4 -2.3418-1.0453-2.4316-2.3125l-0.00195-0.03125-0.00391-0.12695v-1.0078c5.6e-5 -1.3374 1.033-2.4942 2.4375-2.4941h2.9785v-6.0137z"
          />
        </g>
      {:else if side === "left" && $layout === "vertical"}
        <g transform="rotate(90,10,10)">
          <path
            stroke="none"
            fill="currentColor"
            d="m16.5 9.2962-3.0875-3.2748c-0.26814-0.28458-0.67153-0.36977-1.022-0.21582l-0.06798 0.03381c-0.3133 0.17001-0.51046 0.51179-0.51055 0.88505l-2.6e-5 1.7881-4.7911-7.7e-5c-0.51782-1.2e-5 -0.9376 0.44521-0.93763 0.99444l-4.09e-5 0.98467 0.00234 0.0746c0.03679 0.51895 0.44458 0.92005 0.93525 0.9199l4.7911-4.21e-4 -2.7e-5 1.7883c1.6e-4 0.88579 1.0098 1.3294 1.6005 0.70312l3.0876-3.2747c0.36603-0.38833 0.36605-1.0178 2.9e-5 -1.4062"
            stroke-width=".48281"
          />
          <rect
            x="1"
            y="1"
            width="18"
            height="18"
            rx="2"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          />
          <path
            stroke="none"
            fill="currentColor"
            d="m1 1v18h9v-6.0137h-2.9785c-1.3336 4.16e-4 -2.3418-1.0453-2.4316-2.3125l-0.00195-0.03125-0.00391-0.12695v-1.0078c5.6e-5 -1.3374 1.033-2.4942 2.4375-2.4941h2.9785v-6.0137z"
          />
        </g>
      {:else}
        <g transform="matrix(0,-1,-1,0,20,20)">
          <path
            stroke="none"
            fill="currentColor"
            d="m16.5 9.2962-3.0875-3.2748c-0.26814-0.28458-0.67153-0.36977-1.022-0.21582l-0.06798 0.03381c-0.3133 0.17001-0.51046 0.51179-0.51055 0.88505l-2.6e-5 1.7881-4.7911-7.7e-5c-0.51782-1.2e-5 -0.9376 0.44521-0.93763 0.99444l-4.09e-5 0.98467 0.00234 0.0746c0.03679 0.51895 0.44458 0.92005 0.93525 0.9199l4.7911-4.21e-4 -2.7e-5 1.7883c1.6e-4 0.88579 1.0098 1.3294 1.6005 0.70312l3.0876-3.2747c0.36603-0.38833 0.36605-1.0178 2.9e-5 -1.4062"
            stroke-width=".48281"
          />
          <rect
            x="1"
            y="1"
            width="18"
            height="18"
            rx="2"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          />
          <path
            stroke="none"
            fill="currentColor"
            d="m1 1v18h9v-6.0137h-2.9785c-1.3336 4.16e-4 -2.3418-1.0453-2.4316-2.3125l-0.00195-0.03125-0.00391-0.12695v-1.0078c5.6e-5 -1.3374 1.033-2.4942 2.4375-2.4941h2.9785v-6.0137z"
          />
        </g>
      {/if}
    </svg>
  </button>
</Tooltip>
