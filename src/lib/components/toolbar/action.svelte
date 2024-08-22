<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import Tooltip from "$lib/components/tooltip.svelte";
  import { shortcut } from "$lib/shortcut";

  import {
    layout,
    portionType,
    textAreaFocused,
    content,
    highlights,
    portion,
  } from "$lib/stores";

  import { type SideSchema } from "$lib/schemas";

  export let side: SideSchema;
  export let onPerformAction: (caretPos: number) => void;

  let tooltipLabel = "";
  let shortcutLabel = "";

  $: {
    if (!$textAreaFocused["left"] && !$textAreaFocused["right"]) {
      tooltipLabel = "You need to focus on a text area to perform an action";
    } else if ($portionType === "U") {
      tooltipLabel = "No action available for this chunk";
    } else if ($portionType === "A") {
      if ($textAreaFocused[side]) {
        tooltipLabel = "Remove the chunk on this side";
      } else {
        tooltipLabel = `Add the chunk from the other side to this side`;
      }
    } else {
      tooltipLabel =
        "Replace the chunk on this side with the one from the other side";
    }
  }

  $: {
    shortcutLabel = ` (Alt + ${side === "left" ? "Left" : "Right"})`;
  }

  const onAction = () => {
    const otherSide = side === "left" ? "right" : "left";
    if ($portionType === "A") {
      if ($textAreaFocused[side]) {
        //Remove the chunk on this side
        const startIndexToRemove = $highlights[side][$portion].start;
        const endIndexToRemove = $highlights[side][$portion].end;

        $content[side] =
          $content[side].slice(0, startIndexToRemove) +
          $content[side].slice(endIndexToRemove);

        onPerformAction(startIndexToRemove);
      } else {
        //Add the chunk from the other side to this side
        if ($portion < $highlights[otherSide].length - 1) {
          const indexToAdd = $highlights[side][$portion + 1].start;

          $content[side] =
            $content[side].slice(0, indexToAdd) +
            $highlights[otherSide][$portion].content +
            $content[side].slice(indexToAdd);

          onPerformAction(
            indexToAdd + $highlights[otherSide][$portion].content.length - 1,
          );
        } else {
          $content[side] += $highlights[otherSide][$portion].content;

          onPerformAction($content[side].length - 1);
        }
      }
    } else if ($portionType === "M") {
      //Replace the chunk on this side with the one from the other side;
      const startIndexToReplace = $highlights[side][$portion].start;
      const endIndexToReplace = $highlights[side][$portion].end;

      $content[side] =
        $content[side].slice(0, startIndexToReplace) +
        $highlights[otherSide][$portion].content +
        $content[side].slice(endIndexToReplace);

      onPerformAction(
        startIndexToReplace +
          $highlights[otherSide][$portion].content.length -
          1,
      );
    }
  };

  const handler = (e: KeyboardEvent) => {
    if (!side) return;
    if (e.code === `Arrow${side === 'left'? 'Left' : 'Right'}` && e.altKey) {
      e.preventDefault();
      onAction();
    }
  };
  const removeHandler = () => window.removeEventListener('keydown', handler);
  const setHandler = () => {
        removeHandler();
        window.addEventListener('keydown', handler);
    };

  onMount(() => {
    setHandler();
  });

  onDestroy(() => {
    removeHandler();
  });
</script>

<Tooltip
  content={`${tooltipLabel}${$portionType === "U" || (!$textAreaFocused["left"] && !$textAreaFocused["right"]) ? "" : shortcutLabel}`}
  offset={$layout === "horizontal" && side === "right" ? 0.2 : 0.8}
  alignment={$layout === "horizontal" && side === "right" ? "left" : "right"}
  position="toolbar"
>
  {#if !$textAreaFocused["left"] && !$textAreaFocused["right"]}
    <button class="label-toolbar">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="size-5 text-gray-300"
        viewBox="0 0 16 16"
      >
        <path
          fill="currentColor"
          d="M4 8a4 4 0 1 1 8 0a4 4 0 0 1-8 0m4-2.5a2.5 2.5 0 1 0 0 5a2.5 2.5 0 0 0 0-5"
        />
      </svg>
    </button>
  {:else if $textAreaFocused[side]}
    <button
      class={$portionType === "U" ? "label-toolbar" : "btn-toolbar"}
      on:mousedown|preventDefault={() => onAction()}
      use:shortcut={{
        code: `F${side === "left" ? "6" : "7"}`,
        callback: onAction,
      }}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="size-5"
        class:text-app-dark-red={$portionType === "A"}
        class:text-app-dark-yellow={$portionType === "M"}
        class:text-gray-400={$portionType === "U"}
        viewBox="0 0 16 16"
      >
        <path fill="currentColor" d="M8 4a4 4 0 1 1 0 8a4 4 0 0 1 0-8" />
      </svg>
    </button>
  {:else}
    <button
      class={$portionType === "U" ? "label-toolbar" : "btn-toolbar"}
      on:mousedown|preventDefault={onAction}
      on:mouseup|preventDefault
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="size-5"
        class:text-app-dark-green={$portionType === "A"}
        class:text-app-dark-yellow={$portionType === "M"}
        class:text-gray-400={$portionType === "U"}
        viewBox="0 0 16 16"
      >
        <path
          fill="currentColor"
          d="M4 8a4 4 0 1 1 8 0a4 4 0 0 1-8 0m4-2.5a2.5 2.5 0 1 0 0 5a2.5 2.5 0 0 0 0-5"
        />
      </svg>
    </button>
  {/if}
</Tooltip>
