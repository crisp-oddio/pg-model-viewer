import { defineStore, acceptHMRUpdate } from "pinia";
import { ref } from "vue";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

const CHECK_INTERVAL_MS = 60 * 60 * 1000; // re-check hourly

/**
 * Auto-updater (mirrors glogger's): checks GitHub's latest.json shortly after
 * startup and hourly after that; the titlebar shows an "Update to vX.Y.Z"
 * button that downloads, installs, and relaunches.
 */
export const useUpdateStore = defineStore("update", () => {
  const updateAvailable = ref(false);
  const latestVersion = ref("");
  const dismissed = ref(false);

  const installing = ref(false);
  const downloadProgress = ref(0); // 0-100
  const installError = ref<string | null>(null);

  let intervalId: ReturnType<typeof setInterval> | null = null;
  let pendingUpdate: Update | null = null;

  async function checkForUpdate() {
    try {
      const update = await check();
      if (update) {
        updateAvailable.value = true;
        latestVersion.value = update.version;
        pendingUpdate = update;
      }
    } catch {
      // Silently ignore — offline, rate-limited, dev build, etc.
    }
  }

  async function downloadAndInstall() {
    if (!pendingUpdate) return;
    installing.value = true;
    installError.value = null;
    downloadProgress.value = 0;
    let downloaded = 0;
    let total = 0;

    try {
      await pendingUpdate.downloadAndInstall((event) => {
        if (event.event === "Started") {
          total = event.data.contentLength ?? 0;
        } else if (event.event === "Progress") {
          downloaded += event.data.chunkLength;
          if (total > 0) downloadProgress.value = Math.round((downloaded / total) * 100);
        } else if (event.event === "Finished") {
          downloadProgress.value = 100;
        }
      });
      await relaunch();
    } catch (e) {
      installError.value = String(e);
      installing.value = false;
    }
  }

  function startPolling() {
    if (intervalId) return;
    setTimeout(() => checkForUpdate(), 5000);
    intervalId = setInterval(() => checkForUpdate(), CHECK_INTERVAL_MS);
  }

  function dismiss() {
    dismissed.value = true;
  }

  return {
    updateAvailable,
    latestVersion,
    dismissed,
    installing,
    downloadProgress,
    installError,
    checkForUpdate,
    downloadAndInstall,
    startPolling,
    dismiss,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useUpdateStore, import.meta.hot));
}
