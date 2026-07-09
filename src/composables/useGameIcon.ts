import { ref } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";

/** Load a PG item icon by id via the backend `get_icon_path` (fetch + cache). */
export function useGameIcon() {
  const iconSrc = ref<string | null>(null);
  const iconLoading = ref(false);
  let loadedIconId: number | null = null;

  async function loadIcon(iconId: number | null | undefined) {
    if (iconId == null) return;
    if (iconId === loadedIconId) return;
    loadedIconId = iconId;

    iconLoading.value = true;
    try {
      const path = await invoke<string>("get_icon_path", { iconId });
      iconSrc.value = convertFileSrc(path);
    } catch (e) {
      console.warn(`Icon load failed for id ${iconId}:`, e);
      iconSrc.value = null;
    } finally {
      iconLoading.value = false;
    }
  }

  return { iconSrc, iconLoading, loadIcon };
}
