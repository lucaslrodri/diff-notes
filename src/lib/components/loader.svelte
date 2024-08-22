<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import {
    data,
    settings,
    mode,
    layout,
    autosave,
    content,
    title,
    panelSize,
  } from "$lib/stores";

  const fetchSettings = async () => {
    try{
      const mode_res = await $settings.get("mode");
      $mode = mode_res === "w" ? "w" : "s";
      // console.log("Loaded mode");
  
      const layout_res = await $settings.get("layout");
      $layout = layout_res === "vertical" ? "vertical" : "horizontal";
      // console.log("Loaded layout");
  
      const panelPosition_res = await $settings.get("panelPosition");
      if (typeof panelPosition_res !== "number" || panelPosition_res < 0 || panelPosition_res > 100) {
        $panelSize = null;
      }else {
        $panelSize = panelPosition_res;
      }
      // console.log("Loaded panel size");
  
      const autosave_res = await $settings.get("autosave");
      $autosave = autosave_res ? true : false;
      // console.log("Loaded autosave");
  
      if ($autosave) {
        const content_res = await $data.get("content");
        if (typeof content_res === 'object' && content_res !== null && 'left' in content_res && 'right' in content_res) {
          $content.left = typeof content_res.left === 'string' ? content_res.left : "";
          $content.right = typeof content_res.right === 'string' ? content_res.right : "";
          // console.log("Loaded content");
        }else{
          $content.left = "";
          $content.right = "";
          // console.log("No content found");
        }

        await invoke("reset", {
          state: {
            left: $content.left,
            right: $content.right,
            caret: 0,
            side: "l",
          },
        });
  
        const title_res = await $data.get("title");
        if (typeof title_res === 'object' && title_res !== null && 'left' in title_res && 'right' in title_res) {
          $title.left = typeof title_res.left === 'string' ? title_res.left : "";
          $title.right = typeof title_res.right === 'string' ? title_res.right : "";
          // console.log("Loaded title");
        }else{
          $title.left = "";
          $title.right = "";
          // console.log("No title found");
        }
      } else {
        $content.left = "";
        $content.right = "";
        $title.left = "";
        $title.right = "";
        // console.log("Autosave disabled");
      }
    }catch(error){
      console.log("Error fetching settings: ", error);
      $mode = "s";
      $layout = "horizontal";
      $panelSize = 50;
      $autosave = false;
      $content.left = "";
      $content.right = "";
      $title.left = "";
      $title.right = "";
    }finally{
      return true;
    }
  }
  let loadedPromess = fetchSettings();
</script>

<div class="h-screen w-screen flex flex-col">
  {#await loadedPromess then loadedFinised}
    {#if loadedFinised}
      <slot />
    {/if}
  {:catch error}
    <div class="flex flex-col items-center justify-center h-full w-full">
      <p class="text-red-500">Error on loading data</p>
      <p class="text-red-500">{error.message}</p>
    </div>
  {/await}
</div>
