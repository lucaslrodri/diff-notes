<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  import { content, forceSetCaretPosition, first, last, forceChangeState } from "$lib/stores";

  import { type StackCurrentSchema, stackCurrentSchema } from "$lib/schemas";

  import Tooltip from "$lib/components/tooltip.svelte";

  const onStackChange = (current: StackCurrentSchema) => {
    $content["left"] = current.state.left;
    $content["right"] = current.state.right;
    $forceSetCaretPosition = {
      caretPos: current.state.caret,
      side: current.state.side === "l" ? "left" : "right",
    };
    $first = current.first;
    $last = current.last;
  };

  const onUndo = async () => {
    $forceChangeState = "push";
  };

  const onRedo = async () => {
    invoke("redo").then((res) => {
      const current = stackCurrentSchema.parse(res);
      onStackChange(current);
    });
  };
  
  $: {
    if ($forceChangeState == "undo"){
      invoke("undo").then((res) => {
        const current = stackCurrentSchema.parse(res);
        onStackChange(current);
        $forceChangeState = null;
      });
    }
  }

  const handler = (e: KeyboardEvent) => {
    if (e.code === "KeyZ" && e.ctrlKey) {
      e.preventDefault();
      onUndo();
    } else if (e.code === "KeyY" && e.ctrlKey) {
      e.preventDefault();
      onRedo();
    }
  };
  const removeHandler = () => window.removeEventListener("keydown", handler);
  const setHandler = () => {
    removeHandler();
    window.addEventListener("keydown", handler);
  };

  onMount(() => {
    setHandler();
  });

  onDestroy(() => {
    removeHandler();
  });
</script>

<Tooltip
  content="Undo (Ctrl+Z)"
  alignment="left"
  position="statusbar"
  offset={0.25}
  class="flex item-strech"
>
  <!-- Undo:fluent:arrow-undo-16-filled -->
  <button
    class="text-sm pl-1.5 pr-1 flex items-center justify-center text-gray-500 w-[2.125rem] hover:bg-gray-150 disabled:hover:bg-transparent"
    disabled={$first}
    on:click={onUndo}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="h-full"
      class:text-gray-300={$first}
      viewBox="0 0 24 24"
      ><path
        fill="currentColor"
        d="M7.404 18v-1h7.254q1.556 0 2.65-1.067q1.096-1.067 1.096-2.606t-1.095-2.596q-1.096-1.058-2.651-1.058H6.916l2.965 2.965l-.708.708L5 9.173L9.173 5l.708.708l-2.965 2.965h7.742q1.963 0 3.355 1.354q1.39 1.354 1.39 3.3t-1.39 3.31T14.657 18z"
      /></svg
    >
  </button>
</Tooltip>
<Tooltip
  content="Redo (Ctrl+Y)"
  alignment="center"
  offset={0.25}
  position="statusbar"
  class="flex item-strech"
>
  <!-- Undo:fluent:arrow-undo-16-filled -->
  <button
    class="text-sm px-1 flex items-center justify-center text-gray-500 w-8 hover:bg-gray-150 disabled:hover:bg-transparent"
    disabled={$last}
    on:click={onRedo}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="h-full"
      class:text-gray-300={$last}
      viewBox="0 0 24 24"
      ><path
        fill="currentColor"
        d="M9.342 18q-1.963 0-3.355-1.364t-1.39-3.309t1.39-3.3Q7.38 8.673 9.343 8.673h7.743L14.12 5.708L14.828 5L19 9.173l-4.173 4.173l-.708-.707l2.966-2.966H9.342q-1.556 0-2.65 1.058q-1.096 1.058-1.096 2.596t1.095 2.606Q7.787 17 9.342 17h7.254v1z"
      /></svg
    >
  </button>
</Tooltip>
<div class="divider-statusbar" />
