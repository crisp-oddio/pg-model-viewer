<template>
  <div class="app-root font-mono text-text-primary">
    <header class="app-titlebar">
      <span class="text-accent-gold font-semibold text-sm">PG Model Viewer</span>
      <span class="text-text-dim text-[11px] hidden sm:inline">
        Project: Gorgon 3D item models &amp; live dye
      </span>
    </header>
    <main class="app-main">
      <ModelViewerScreen />
    </main>
  </div>
</template>

<script setup lang="ts">
import ModelViewerScreen from "./components/Character/ModelViewer/ModelViewerScreen.vue";
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
  align-items: baseline;
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
