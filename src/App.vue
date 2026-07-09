<template>
  <div class="app-root font-mono text-text-primary">
    <header class="app-titlebar">
      <div class="flex items-baseline gap-3 min-w-0">
        <span class="text-accent-gold font-semibold text-sm shrink-0">PG Model Viewer</span>
        <span class="text-text-dim text-[11px] hidden sm:inline truncate">
          Project: Gorgon 3D item models &amp; live dye
        </span>
      </div>
      <SettingsMenu />
    </header>
    <main class="app-main">
      <ModelViewerScreen />
    </main>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import ModelViewerScreen from "./components/Character/ModelViewer/ModelViewerScreen.vue";
import SettingsMenu from "./components/SettingsMenu.vue";
import { useSettingsStore } from "./stores/settingsStore";

// Apply persisted font + scale to the document on startup.
const settings = useSettingsStore();
onMounted(() => settings.applyAll());
</script>

<style>
:root {
  color-scheme: dark;
}
html,
body,
#app {
  height: 100%;
  margin: 0;
}
body {
  background: var(--color-surface-base, #1a1a1a);
}
.app-root {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.app-titlebar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.4rem 0.75rem;
  border-bottom: 1px solid var(--color-border-default, #333);
  flex: 0 0 auto;
}
.app-main {
  flex: 1 1 auto;
  min-height: 0;
}
</style>
