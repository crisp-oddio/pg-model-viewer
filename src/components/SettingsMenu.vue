<template>
  <div ref="root" class="relative">
    <button
      class="px-2 py-1 text-xs rounded border border-border-default text-text-muted hover:bg-white/5 hover:text-text-secondary cursor-pointer flex items-center gap-1"
      :class="{ 'bg-white/5 text-text-secondary': open }"
      title="Settings"
      @click="open = !open">
      <span class="text-sm leading-none">⚙</span>
      <span class="hidden sm:inline">Settings</span>
    </button>

    <div
      v-if="open"
      class="absolute right-0 top-full mt-1 w-72 z-50 rounded border border-border-default bg-surface-elevated shadow-lg p-3 flex flex-col gap-4">
      <!-- Font family -->
      <div>
        <label class="block text-text-secondary mb-1.5 text-xs">UI Font</label>
        <select
          class="input text-xs w-full"
          :value="settings.uiFontFamily"
          @change="settings.setFontFamily(($event.target as HTMLSelectElement).value)">
          <option v-for="f in FONT_FAMILY_OPTIONS" :key="f.value" :value="f.value">
            {{ f.label }}
          </option>
        </select>
        <div
          class="mt-1.5 text-[11px] text-text-muted truncate"
          :style="{ fontFamily: settings.uiFontFamily }">
          The quick brown fox — 0123456789
        </div>
      </div>

      <!-- UI scale -->
      <div>
        <div class="flex items-center justify-between mb-1.5">
          <label for="ui-scale" class="text-text-secondary text-xs">UI Scale</label>
          <span class="text-[11px] text-text-muted">{{ settings.uiScale }}%</span>
        </div>
        <div class="flex items-center gap-2">
          <input
            id="ui-scale"
            type="range"
            class="flex-1 cursor-pointer"
            :min="MIN_UI_SCALE"
            :max="MAX_UI_SCALE"
            step="5"
            :value="settings.uiScale"
            @input="settings.previewUiScale(sliderVal($event))"
            @change="settings.setUiScale(sliderVal($event))" />
          <button
            class="btn-secondary text-[11px] px-2 py-0.5 whitespace-nowrap"
            @click="settings.resetUiScale()">
            Reset
          </button>
        </div>
        <p class="mt-1 text-[10px] text-text-dim leading-snug">
          Zooms the whole app ({{ MIN_UI_SCALE }}–{{ MAX_UI_SCALE }}%). Useful on
          high-DPI / 4K displays.
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import {
  useSettingsStore,
  FONT_FAMILY_OPTIONS,
  MIN_UI_SCALE,
  MAX_UI_SCALE,
} from "../stores/settingsStore";

const settings = useSettingsStore();
const open = ref(false);
const root = ref<HTMLElement | null>(null);

function sliderVal(e: Event): number {
  return Number((e.target as HTMLInputElement).value);
}

function onDocClick(e: MouseEvent) {
  if (open.value && root.value && !root.value.contains(e.target as Node)) {
    open.value = false;
  }
}

onMounted(() => document.addEventListener("mousedown", onDocClick));
onBeforeUnmount(() => document.removeEventListener("mousedown", onDocClick));
</script>
