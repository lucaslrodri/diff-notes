import { writable } from "svelte/store";
import { Store } from "tauri-plugin-store-api";

import { type TextsChangesHighlightsSchema, type LayoutSchema, type SideSchema, type ContentSchema } from "$lib/schemas";

export const settings = writable(new Store(".settings.json"));
export const data = writable(new Store(".data.json"));

export const mode = writable("s");
export const layout = writable<LayoutSchema>("horizontal");
export const autosave = writable(false);
export const content = writable<ContentSchema>({ left: "", right: "" });
export const title = writable<ContentSchema>({ left: "", right: "" });
export const panelSize = writable<number | null>(null);

export const forceCalculateCaretPosition = writable(false);
export const forceSetCaretPosition = writable<{ side: SideSchema, caretPos: number } | null>(null);
export const forceAlignScrollWithCurentChunk = writable({left: false, right: false});
export const forceChangeState = writable<"undo"|"push"|null>(null);

export const portion = writable<number>(0);
export const portionType = writable<"U" | "M" | "A" | "R">("U");

export const textAreaFocused = writable<{ left: boolean, right: boolean }>({ left: false, right: false });

export const highlights = writable<TextsChangesHighlightsSchema>({ left: [], right: [], raw: [] });

export const first = writable(true);
export const last = writable(true);

