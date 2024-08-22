<script lang="ts">
  import { onMount } from "svelte";

  export let content: string;
  export let position: "toolbar" | "statusbar";
  export let alignment: "left" | "center" | "right";
  export let offset: number = 0;

  let containerRef: HTMLSpanElement;

  let containerX: number;
  let containerY: number;
  let containerW: number;

  let tooltipW: number;

  let tooltipPositionX: number = 0;
  let tooltipPositionY: number = 0;

  let show = false;

  const computeTooltipPosition = () => {
    if (!containerRef) return;

    let cumulativeOffsetLeft = 0;
    let cumulativeOffsetTop = 0;

    let currentElement: HTMLElement = containerRef;

    while (currentElement && currentElement !== document.body) {
      const computedStyle = window.getComputedStyle(currentElement);
      const elementPosition = computedStyle.position;

      if (elementPosition === "absolute") {
        cumulativeOffsetLeft += parseFloat(computedStyle.left) || 0;
        cumulativeOffsetTop += parseFloat(computedStyle.top) || 0;
      } else if (elementPosition === "relative") {
        cumulativeOffsetLeft += currentElement.offsetLeft;
        cumulativeOffsetTop += currentElement.offsetTop;
        if (alignment === "right") {
          cumulativeOffsetLeft -= containerW;
        }
      }

      const parentElement = currentElement.parentElement;
      if (parentElement !== null) {
        currentElement = parentElement;
      } else {
        break;
      }
    }

    let finalLeft =
      containerX + cumulativeOffsetLeft + (offset - 1) * containerW;
    let finalTop = cumulativeOffsetTop;

    if (alignment === "right") {
      finalLeft -= tooltipW;
      finalLeft += containerW;
    }else if (alignment === "center"){
      finalLeft += containerW / 2;
      finalLeft -= tooltipW / 2;
    }

    if (position === "statusbar") {
      finalTop -= containerY + 4;
    }else{
      finalTop += containerY + 2;
    }
    
    tooltipPositionX = finalLeft;
    tooltipPositionY = finalTop;
  };

  onMount(() => {
    computeTooltipPosition();
  });
</script>

<span
  bind:offsetWidth={containerX}
  bind:offsetHeight={containerY}
  bind:clientWidth={containerW}
  bind:this={containerRef}
  on:mouseenter={() => {
    show = true;
    computeTooltipPosition();
  }}
  on:mouseleave={() => {
    show = false;
  }}
  {...$$restProps}
  role="button" aria-haspopup="true" aria-controls="toolipId" tabindex="0"><slot /></span
>
<span
  role="tooltip"
  bind:clientWidth={tooltipW}
  class="bg-gray-700 shadow-sm rounded-lg text-white opacity-0 text-xs p-2 pointer-events-none absolute text-left invisible whitespace-nowrap z-[100] transition-opacity duration-200 ease-in-out"
  class:show
  style={`left: ${show? tooltipPositionX : 0}px; top: ${show? tooltipPositionY : 0}px;`}>{@html content}</span
>

<style lang="postcss">
  .show {
    @apply opacity-100 visible whitespace-normal;
  }
</style>
