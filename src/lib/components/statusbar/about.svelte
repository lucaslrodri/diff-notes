<script>
  import Tooltip from "$lib/components/tooltip.svelte";

  import { getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";

  let version = "";
  onMount(async () => {
    version = await getVersion();
    version = ` | v${version}`;
  });
</script>

<Tooltip
  content={`Made with <span style='color: rgb(255 105 97); font-size: 0.75rem;'>♥</span> by <b>@lucaslrodri</b> ${version}`}
  alignment="right"
  offset={1.25}
  position="statusbar"
  class="flex items-stretch"
>
  <a class="btn-statusbar w-[2.3rem] hover:bg-gray-150"
    href="https://github.com/lucaslrodri"
    target="_blank"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      class="h-full text-gray-500"
    >
      <path
        fill="currentColor"
        d="M11.963 2.382C.554 2.621-1.82 17.93 8.852 21.602c.498.093.684-.219.684-.478v-1.68c-2.79.601-3.38-1.317-3.38-1.317a2.603 2.603 0 0 0-1.121-1.442c-.902-.612.072-.602.072-.602a2.074 2.074 0 0 1 1.536 1.038a2.167 2.167 0 0 0 2.924.819c.052-.5.275-.965.633-1.317c-2.23-.25-4.564-1.1-4.564-4.875a3.755 3.755 0 0 1 1.038-2.645a3.464 3.464 0 0 1 .103-2.634s.84-.26 2.76 1.037a9.584 9.584 0 0 1 5.02 0c1.908-1.276 2.748-1.038 2.748-1.038c.365.828.398 1.763.093 2.614a3.754 3.754 0 0 1 1.037 2.645c0 3.786-2.344 4.626-4.574 4.865c1.038.55.602 4.086.664 4.522c0 .259.176.57.695.477c10.642-3.64 8.152-18.97-3.257-19.209"
      />
    </svg>
  </a>
</Tooltip>
