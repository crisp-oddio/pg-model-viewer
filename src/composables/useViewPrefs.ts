import { ref, type Ref } from "vue";

/**
 * Per-screen view preferences persisted to localStorage (standalone
 * replacement for glogger's settings-store-backed version). Same signature:
 * a shared reactive object per `screenKey`, debounced-saved on update.
 */
const _sharedRefs = new Map<
  string,
  { prefs: Ref<Record<string, unknown>>; update: (partial: Record<string, unknown>) => void }
>();

export function useViewPrefs<T extends Record<string, unknown>>(
  screenKey: string,
  defaults: T,
): { prefs: Ref<T>; update: (partial: Partial<T>) => void } {
  const existing = _sharedRefs.get(screenKey);
  if (existing) {
    return existing as unknown as { prefs: Ref<T>; update: (partial: Partial<T>) => void };
  }

  const storageKey = `viewPrefs:${screenKey}`;
  let stored: Partial<T> = {};
  try {
    const raw = localStorage.getItem(storageKey);
    if (raw) stored = JSON.parse(raw) as Partial<T>;
  } catch {
    // Corrupt/absent — fall back to defaults.
  }

  const prefs = ref({ ...defaults, ...stored }) as Ref<T>;
  let debounce: ReturnType<typeof setTimeout> | null = null;

  function update(partial: Partial<T>) {
    prefs.value = { ...prefs.value, ...partial };
    if (debounce) clearTimeout(debounce);
    debounce = setTimeout(() => {
      try {
        localStorage.setItem(storageKey, JSON.stringify(prefs.value));
      } catch {
        // Storage full/unavailable — preferences just won't persist.
      }
    }, 400);
  }

  const entry = { prefs, update } as {
    prefs: Ref<Record<string, unknown>>;
    update: (partial: Record<string, unknown>) => void;
  };
  _sharedRefs.set(screenKey, entry);
  return { prefs, update };
}
