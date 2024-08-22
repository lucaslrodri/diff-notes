<script lang="ts">
  import Copy from "$lib/components/toolbar/copy.svelte";
  import Paste from "$lib/components/toolbar/paste.svelte";
  import Title from "$lib/components/toolbar/title.svelte";
  import Action from "$lib/components/toolbar/action.svelte";
  import Move from "$lib/components/toolbar/move.svelte";

  import { readText } from "@tauri-apps/api/clipboard";
  import { onMount, onDestroy } from "svelte";

  import { invoke } from "@tauri-apps/api/tauri";

  import {
    layout,
    content,
    autosave,
    data,
    highlights,
    portion,
    forceCalculateCaretPosition,
    portionType,
    textAreaFocused,
    forceSetCaretPosition,
    forceAlignScrollWithCurentChunk,
    first,
    last,
    forceChangeState,
  } from "$lib/stores";

  import { type SideSchema } from "$lib/schemas";

  export let side: SideSchema;

  let textArea: HTMLDivElement;
  let caretPos: number = 0;

  let highlightsContainer: HTMLDivElement;
  let removeHighlightsContainer: HTMLDivElement;

  let scrollInterval: ReturnType<typeof setInterval>;
  let caretPosTimeout: ReturnType<typeof setTimeout>;
  let debounceTypingTimeout: ReturnType<typeof setTimeout> | null = null;

  let otherSide: SideSchema = "left";
  $: otherSide = side === "left" ? "right" : "left";

  const syncScroll = () => {
    if (!textArea || !highlightsContainer || !removeHighlightsContainer) return;
    highlightsContainer.scrollTop = textArea.scrollTop;
    removeHighlightsContainer.scrollTop = textArea.scrollTop;
  };

  const onPaste = async () => {
    const text = await readText();
    if (text) {
      $content[side] = text;
      textArea.focus();
      $last = true;
      $first = false;
      setCaretPosition(text.length, async () => {
        await onAddStack(text.length, side);
      });
    }
  };

  const onMove = async () => {
    if ($content[side]) {
      $content[otherSide] = $content[side];
      await onAddStack($content[otherSide].length, otherSide);
    }
  };

  const getCarretPosition = (): void => {
    if (!textAreaFocused) return;

    const doc = textArea.ownerDocument.defaultView;
    if (!doc) return;

    const selection = doc.getSelection();
    if (!selection) return;

    if (selection.rangeCount > 0) {
      const range = selection.getRangeAt(0);
      const preCaretRange = range.cloneRange();
      preCaretRange.selectNodeContents(textArea);
      preCaretRange.setEnd(range.endContainer, range.endOffset);
      caretPos = preCaretRange.toString().length;

      $portion = $highlights[side].findIndex((item) => {
        return item.start <= caretPos && item.end >= caretPos;
      });

      if ($portion >= $highlights[side].length) return;

      if ($highlights[side][$portion]?.op === "M") {
        $portionType = "M";
      } else if ($highlights[side][$portion]?.op === "A") {
        $portionType = "A";
      } else if ($highlights[side][$portion]?.op === "U") {
        $portionType = "U";
      }
    }
  };

  const setCaretPosition = (pos: number, callback: () => void = () => {}) => {
    textArea.focus();
    caretPosTimeout = setTimeout(() => {
      // Create a range and selection object
      const range = document.createRange();
      const selection = window.getSelection();

      // Assuming the content is simple and the first child is a text node
      const textNode = textArea.firstChild || textArea;
      if (!textNode) return;

      const textNodeLength = textNode.textContent?.length || 0;

      // Clamp the position to ensure it's within the text node length
      const clampedPosition = Math.min(pos, textNodeLength);

      // Set the start and end of the range to the desired position
      range.setStart(textNode, clampedPosition);
      range.setEnd(textNode, clampedPosition);

      // Clear any existing selections
      selection?.removeAllRanges();

      // Add the new range, which sets the caret position
      selection?.addRange(range);

      getCarretPosition();
      callback();
    }, 100);
  };

  const onAddStack = async (caretPos: number, side: SideSchema) => {
    await invoke("push", {
      state: {
        left: $content.left,
        right: $content.right,
        side: side === "left" ? "l" : "r",
        caret: caretPos,
      },
    });
  };

  $: {
    $data.set("content", $autosave ? $content : { left: "", right: "" });
    syncScroll();
  }

  $: {
    if ($forceCalculateCaretPosition) {
      getCarretPosition();
      $forceCalculateCaretPosition = false;
    }
  }

  $: {
    if ($forceSetCaretPosition) {
      if ($forceSetCaretPosition.side === side) {
        setCaretPosition($forceSetCaretPosition.caretPos);
        $forceSetCaretPosition = null;
      }
    }
  }

  $: {
    if ($forceChangeState === "push") {
      onAddStack(caretPos, side).then(() => {
        $forceChangeState = "undo";
      });
    }
  }

  const alignScrollWithChunk = () => {
    let currentPortionNode = highlightsContainer.querySelector(".current");
    if (!currentPortionNode) {
      currentPortionNode = removeHighlightsContainer.querySelector(".current");
    }
    if (!currentPortionNode) return;

    const nodeTop =
      currentPortionNode.getBoundingClientRect().top + textArea.scrollTop;
    const textAreaTop = textArea.getBoundingClientRect().top;
    textArea.scrollTo({ top: nodeTop - textAreaTop - 5, behavior: "smooth" });
  };

  $: {
    if ($forceAlignScrollWithCurentChunk[side]) {
      alignScrollWithChunk();
      syncScroll();
      $forceAlignScrollWithCurentChunk[side] = false;
    }
  }

  onMount(() => {
    scrollInterval = setInterval(() => {
      syncScroll();
    }, 100);
  });

  onDestroy(() => {
    if (scrollInterval) {
      clearInterval(scrollInterval);
    }
    if (caretPosTimeout) {
      clearTimeout(caretPosTimeout);
    }
  });
</script>

<!--Toolbar-->
<div
  class="flex flex-row justify-between items-center px-2 py-2 border-b border-gray-200 bg-gray-50 z-40"
  class:bg-gray-150={textAreaFocused}
  class:shadow-gray-200={textAreaFocused}
  class:shadow-toolbar={textAreaFocused}
  class:flex-row-reverse={side == "right" && $layout == "horizontal"}
>
  <div class="flex flex-row">
    <Copy {side} />
    <Paste {side} {onPaste} />
  </div>
  <Title {side} />
  <div
    class="flex flex-row flex-none"
    class:flex-row-reverse={side == "right" && $layout == "horizontal"}
  >
    <Action
      {side}
      onPerformAction={(caretPos) => {
        $last = true;
        $first = false;
        setCaretPosition(caretPos, () => onAddStack(caretPos, side));
      }}
    />
    <Move {side} {onMove} />
  </div>
</div>

<!--Editor-->
<div class="w-full flex-1 bg-white relative">
  <div
    class="absolute w-full h-full py-2 px-3 text-base font-mono whitespace-pre-wrap break-words overflow-y-auto z-20"
    bind:this={highlightsContainer}
  >
    {#each $highlights[side] as highlight, index}
      {#if highlight.op === "A"}
        <mark
          class="bg-app-green text-black/0 text-base font-mono"
          class:current={index === $portion}
          class:added={index === $portion}>{highlight.content}</mark
        >
      {:else if highlight.op === "M"}
        <mark
          class="bg-app-yellow text-black/0 text-base font-mono"
          class:current={index === $portion}
          class:changed={index === $portion}>{highlight.content}</mark
        >
      {:else if highlight.op === "U"}
        <span
          class="text-black/0 text-base font-mono"
          class:current={index === $portion}
          class:bg-gray-200={index === $portion}>{highlight.content}</span
        >
      {/if}
    {/each}
  </div>
  <div
    class="absolute w-full h-full py-2 px-3 text-base font-mono text-black/0 whitespace-pre-wrap break-words overflow-y-auto z-10"
    bind:this={removeHighlightsContainer}
  >
    {#each $highlights[side] as highlight, index}
      {#if highlight.op === "R"}
        <mark
          class="w-0 !ml--0.5 bg-app-red overflow-show opacity-80"
          class:current={index === $portion}
          class:removed={index === $portion}
          ><span class="w-0.5 text-black/0 font-sans">.</span></mark
        >
      {:else}
        {highlight.content}
      {/if}
    {/each}
  </div>
  <div
    role="textbox"
    id={`editor-${side}`}
    aria-multiline="true"
    tabindex="0"
    contenteditable="plaintext-only"
    spellcheck="false"
    class="absolute w-full h-full p-2 px-3 z-30 text-base whitespace-pre-wrap break-words font-mono overflow-y-auto text-gray-700 focus:text-gray-900 bg-transparent border-0 focus:outline-none"
    bind:textContent={$content[side]}
    bind:this={textArea}
    on:focus={() => ($textAreaFocused[side] = true)}
    on:blur={() => {
      $textAreaFocused[side] = false;
    }}
    on:paste={() => onAddStack(caretPos, side)}
    on:scroll={(_e) => syncScroll()}
    on:input={() => {
      $last = true;
      $first = false;
    }}
    on:keydown={(e) => {
      //Support to TAB
      if (e.key === "Tab") {
        e.preventDefault();
        document.execCommand("insertHTML", false, "&#009");
      }
      getCarretPosition();
      if (debounceTypingTimeout !== null) {
        clearTimeout(debounceTypingTimeout);
      }
    }}
    on:keyup={() => {
      if (debounceTypingTimeout !== null) {
        clearTimeout(debounceTypingTimeout);
      }
      getCarretPosition();
      debounceTypingTimeout = setTimeout(() => onAddStack(caretPos, side), 500);
    }}
    on:mousedown={() => getCarretPosition()}
    on:mouseup={() => getCarretPosition()}
  />
</div>

<style lang="postcss">
  .changed {
    @apply bg-app-dark-yellow/60;
  }
  .removed {
    @apply bg-app-dark-red/80;
  }
  .added {
    @apply bg-app-dark-green/60;
  }
</style>
