<template>
  <div class="flex flex-col gap-3">
    <div
      v-for="slot in dyeableSlots"
      :key="slot.slot"
      class="flex flex-col gap-1.5">
      <div class="flex items-center justify-between">
        <span class="text-xs font-medium text-text-secondary">{{ slot.slot }}</span>
        <span class="text-[10px] text-text-muted uppercase tracking-wide">
          {{ channelLabel(slot.dye_channels) }} dye
        </span>
      </div>

      <!-- One dropdown per dye channel, listing every in-game dye -->
      <div
        v-for="ch in slot.dye_channels"
        :key="ch"
        class="flex items-center gap-2">
        <span class="text-[10px] text-text-muted w-3 text-center shrink-0">{{ ch }}</span>
        <span
          class="w-6 h-6 rounded border border-white/15 shrink-0"
          :style="{ backgroundColor: displayHex(slot, ch) }"
          :title="displayHex(slot, ch)" />
        <select
          class="input text-xs flex-1 min-w-0"
          :value="selectedValue(slot, ch)"
          @change="onSelect(slot, ch, $event)">
          <option value="">Default ({{ defaultHex(slot, ch) }})</option>
          <option v-for="d in store.dyes" :key="d.name" :value="normalize(d.color)">
            {{ d.name }}
          </option>
        </select>
      </div>
    </div>

    <div v-if="dyeableSlots.length === 0" class="text-xs text-text-muted">
      {{
        store.viewMode === "character"
          ? "No dyeable pieces equipped."
          : notDyeable
            ? "This item is not dyeable."
            : "No dye channels."
      }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useModelViewerStore } from "../../../stores/modelViewerStore";
import type { ResolvedSlot } from "../../../types/modelViewer";

const store = useModelViewerStore();

const emit = defineEmits<{
  (e: "dye", slot: string, channel: number, hex: string): void;
}>();

// Item mode: the active item's dye channels. Character mode: every dyeable
// piece across the whole equipped loadout, so the outfit is dyeable in one
// place without re-selecting each slot.
const dyeableSlots = computed<ResolvedSlot[]>(() => {
  const source =
    store.viewMode === "character"
      ? Object.values(store.resolvedLoadout).flatMap((a) => a.slots)
      : (store.resolved?.slots ?? []);
  return source.filter((s) => s.dyeable && s.dye_channels > 0);
});

const notDyeable = computed(
  () => store.resolved?.not_dyeable_keyword ?? false,
);

function channelLabel(n: number): string {
  return n === 1 ? "Single" : n === 2 ? "Double" : n === 3 ? "Triple" : `${n}`;
}

/** Game DyeColor values are bare uppercase hex ("00BFFF") → "#00bfff". */
function normalize(hex: string): string {
  return `#${hex.replace(/^#/, "").toLowerCase()}`;
}

function defaultHex(slot: ResolvedSlot, channel: number): string {
  const c = slot.default_colors?.[`_Color${channel}`];
  if (!c) return "#808080";
  const to = (v: number) =>
    Math.max(0, Math.min(255, Math.round(v * 255)))
      .toString(16)
      .padStart(2, "0");
  return `#${to(c[0])}${to(c[1])}${to(c[2])}`;
}

/** The swatch chip: the user's stored dye for this slot/channel, else default. */
function displayHex(slot: ResolvedSlot, channel: number): string {
  return store.getDye(slot.slot, channel) ?? defaultHex(slot, channel);
}

/** Dropdown selection: stored dye hex if it matches a game dye, else Default. */
function selectedValue(slot: ResolvedSlot, channel: number): string {
  const stored = store.getDye(slot.slot, channel);
  if (!stored) return "";
  const hex = normalize(stored);
  return store.dyes.some((d) => normalize(d.color) === hex) ? hex : "";
}

function onSelect(slot: ResolvedSlot, channel: number, ev: Event) {
  const value = (ev.target as HTMLSelectElement).value;
  // "" = revert to the material's own default color for this channel.
  emit("dye", slot.slot, channel, value || defaultHex(slot, channel));
}
</script>
