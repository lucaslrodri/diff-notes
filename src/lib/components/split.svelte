<script lang="ts">
  import Editor from "$lib/editor.svelte";

  import { layout, panelSize, settings } from "$lib/stores";

  type SplitterPointerEvent = PointerEvent & {
    currentTarget: EventTarget & HTMLDivElement;
  };

  const MIN_HEIGHT = 240;
  const SPLITER_SIZE = 8;
  const MIN_WIDTH = 480;

  let splitNode: HTMLDivElement;
  let leftNode: HTMLDivElement;

  // let percent: number | undefined = undefined;

  // ------- Dinamic styles -------//
  $: minClientSize = $layout === "horizontal" ? MIN_WIDTH : MIN_HEIGHT;
  $: leftSize =
    $panelSize !== null ? `minmax(${minClientSize}px, ${$panelSize}%)` : "1fr";
  $: rightSize = `minmax(${minClientSize}px, 1fr)`;
  $: spliterSize = `${dragging ? SPLITER_SIZE : SPLITER_SIZE}px`;
  $: gridTemplateStyle = `${$layout === "horizontal" ? "grid-template-columns" : "grid-template-rows"}: ${leftSize} ${spliterSize} ${rightSize};`;

  // ----- Drag tracking ----- //

  let startPosition: number;
  let startLeftSize: number;
  let dragging = false;
  let position: number;

  $: leftClientSize =
    $layout === "horizontal"
      ? leftNode?.getBoundingClientRect().width
      : leftNode?.getBoundingClientRect().height;
  $: splitClientSize =
    $layout === "horizontal"
      ? splitNode?.getBoundingClientRect().width
      : splitNode?.getBoundingClientRect().height;

  // ----- Event handlers ----- //
  const onPointerDown = (event: SplitterPointerEvent) => {
    leftClientSize =
      $layout === "horizontal"
        ? leftNode.getBoundingClientRect().width
        : leftNode.getBoundingClientRect().height;

    event.currentTarget.setPointerCapture(event.pointerId);
    startPosition = $layout === "horizontal" ? event.clientX : event.clientY;
    startLeftSize = leftClientSize;
    dragging = true;
  };

  const onPointerMove = (event: SplitterPointerEvent) => {
    splitClientSize =
      $layout === "horizontal"
        ? splitNode.getBoundingClientRect().width
        : splitNode.getBoundingClientRect().height;

    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      position = $layout === "horizontal" ? event.clientX : event.clientY;
      let newPrimarySize = startLeftSize + (position - startPosition);
      newPrimarySize = Math.max(0, Math.min(newPrimarySize, splitClientSize));
      const newPercent = (newPrimarySize / splitClientSize) * 100;

      $panelSize = newPercent;
    }
  };

  const onSavePanelPosition = async () => {
    await $settings.set("panelPosition", $panelSize);
    await $settings.save();
  };

  const onPointerUp = async (event: SplitterPointerEvent) => {
    event.currentTarget.releasePointerCapture(event.pointerId);
    dragging = false;
    onSavePanelPosition();
  };

  const onDoubleClick = async () => {
    $panelSize = null;
    onSavePanelPosition();
  };
</script>

<div
  class="grid w-full bg-white flex-1"
  class:flex-col={$layout == "vertical"}
  style={gridTemplateStyle}
  bind:this={splitNode}
>
  <div
    class="w-full flex flex-col"
    class:h-full={$layout == "vertical"}
    class:flex-row={$layout == "vertical"}
    bind:this={leftNode}
  >
    <Editor side="left" />
  </div>

  <div
    class="bg-gray-100 transition-colors flex flex-row items-center justify-center group"
    class:cursor-ew-resize={$layout == "horizontal"}
    class:cursor-ns-resize={$layout == "vertical"}
    role="separator"
    on:pointerdown|preventDefault|stopPropagation={onPointerDown}
    on:pointermove={onPointerMove}
    on:dblclick={onDoubleClick}
    on:pointerup={onPointerUp}
  >
    {#if $layout === "horizontal"}
      <!-- material-symbols:drag-indicator -->
      <svg
        class="w-full p-[2px] text-gray-250 group-hover:text-gray-400 transition-colors"
        version="1.1"
        viewBox="0 0 6 48"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="m1 1v46"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-width="2"
        />
        <path
          d="m5 1v46"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-width="2"
        />
      </svg>
    {:else}
      <svg
        class="h-full p-[2px] text-gray-250 group-hover:text-gray-400 transition-colors"
        version="1.1"
        viewBox="0 0 48 6"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="m1 5h46"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-width="2"
        />
        <path
          d="m1 1h46"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-width="2"
        />
      </svg>
    {/if}
  </div>

  <div
    class="w-full flex flex-col"
    class:h-full={$layout == "vertical"}
    class:flex-row={$layout == "vertical"}
  >
    <Editor side="right" />
  </div>
</div>
