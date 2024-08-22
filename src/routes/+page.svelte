<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  // import { Split, DefaultSplitter } from "@geoffcox/svelte-splitter"
  
  import Split from "$lib/components/split.svelte";
  import Loader from "$lib/components/loader.svelte";
  import Job from "$lib/components/statusbar/job.svelte";
  import Mode from "$lib/components/statusbar/mode.svelte";
  import Delete from "$lib/components/statusbar/delete.svelte";
  import Layout from "$lib/components/statusbar/layout.svelte";
  import Autosave from "$lib/components/statusbar/autosave.svelte";
  import Portion from "$lib/components/statusbar/portion.svelte";
  import History from "$lib/components/statusbar/history.svelte";
  import About from "$lib/components/statusbar/about.svelte";

  import { mode, content, highlights } from "$lib/stores";
  import { textsChangesHighlightsSchema } from "$lib/schemas";

  let isWorking = false;

  $: {
    isWorking = true;
    invoke("get_highlights_of_text_changes", {
      textLeft: $content.left,
      textRight: $content.right,
      mode: $mode,
    }).then((res) => {
      $highlights = textsChangesHighlightsSchema.parse(res);
      isWorking = false;
    });
  }
</script>

<Loader>
  <!-- Editor -->
  <Split/>
  <!-- Toolbar -->
  <div
    class="border-t border-gray-200 bg-gray-50 h-8 flex w-screen flex-row justify-between items-stretch gap-2"
  >
    <!-- Left Toolbar -->
    <div class="flex flex-row pr-2 items-stretch">
      <History />
      <!-- <div class="divider-statusbar" /> -->
      <Autosave />
      <Job bind:isWorking />
    </div>
    <!-- Right Toolbar -->
    <div class="flex flex-row pl-2">
      <Mode />
      <Portion />
      <Layout />
      <About />
      <Delete />
    </div>
  </div>
</Loader>
