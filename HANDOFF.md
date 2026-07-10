# PG Model Viewer — Session Handoff

**Date:** 2026-07-09/10 (Session 1 — extraction from glogger, paper-doll polish, v0.1.1 → v0.1.5 shipped)
**Machine:** Windows 11 (primary dev box)
**Branch:** `main` (no branching yet) — everything committed + pushed. Repo is **public**.
**Status:** ✅ `cargo test --lib` 18 pass; `vue-tsc` + `vite build` clean; **v0.1.5 released** (manifest verified serving 0.1.5). User visually verified male + female paper dolls, dye, loadouts, icons, and the Umrad/Court armor fixes. ⏳ **Auto-updater's live click-through (v0.1.4 → v0.1.5 in the title bar) is the one unverified leg** — user was about to test it.

## TL;DR — Session 1

This app is glogger's 3D Model Viewer, extracted into its own repo (and purged from glogger's history — see glogger's Session-33 handoff). Read auto-memory `project_model_viewer.md` first: it holds the deep facts (bundle layout, dye shader, EquipAppearance grammar, coverage, orientation rules).

### App shape
- **Backend** (`src-tauri/src/`): `cdn.rs` + `cdn_commands.rs` (minimal PG CDN layer — `items.json` + icons + `list_dyes`; `GameData` state), `game_data/appearance.rs` (EquipAppearance parser, 14 tests), `game_data/items.rs` (trimmed ItemInfo incl. `dye_color`), `model_assets.rs` (install detection, catalog, `resolve_item_appearance`, `resolve_base_body` [4 tests], extraction command, `EXPECTED_CATALOG_SCHEMA`).
- **Frontend** (`src/`): `components/Character/ModelViewer/*` (screen, TurntableViewer with `buildItem`/`buildCharacter`, DyeControls), `useGorgonDyeMaterial` (dye shader), `modelViewerStore` (loadout + `dyeBySlot` + localStorage presets), `settingsStore` (font + UI scale), `useGameIcon`/`useViewPrefs` (standalone rewrites).
- **Extractor** `tools/model_extractor/extract.py` (+ `build_sidecar.sh` → PyInstaller sidecar in releases).

### Shipped this session
- **v0.1.1** — first release; proved the 3-platform pipeline (exec-bit on `build_sidecar.sh` was the only stumble; bump job is idempotent for re-releases).
- **v0.1.2** — dye dropdowns (all **101 game dyes** from items.json `DyeColor` via `list_dyes`), dye persistence + **saved loadouts**, whole-outfit dye panel in Character mode, settings (font, 50–200% scale), slot-icon fill, and the extractor fixes below.
- **v0.1.3** — **model grouping** (items sharing an appearance string collapse into one row with a ×N badge + member tooltip; chest 314 items → 39 models; search matches any member; rep prefers display names; `BrowsableItem.appearance_key`) and the **auto-updater** (below).
- **v0.1.4** — **bind-pose correction** (extractor fix #4 below; fixes the Winter/Summer Court "skintight" coats rendering sideways with floating hands, and the 2.5×-oversized female Umrad Coat; `CATALOG_SCHEMA → 4`) and **texture wrapping** (mirrored gear parts — e.g. the Umrad Coat's left pauldron/skirt — tile UVs past 1.0 and rendered flat black under three.js's default clamp-to-edge; `RepeatWrapping` on all textures in `loadTexture`).
- **v0.1.5** — **first-run flow fix**: after "Extract models" the viewer appeared but the item list/base body only loaded in `onMounted`, forcing a relaunch; `runExtraction` now awaits the command and runs the full init (refreshStatus + fetchBaseBody + selectSlot; progress events are display-only). Plus the extraction screen no longer says "glogger's cache".

### Auto-updater (v0.1.3+)
Mirrors glogger: `tauri-plugin-updater` + `tauri-plugin-process`; checks `releases/latest/download/latest.json` 5s after startup + hourly (`updateStore.ts`); titlebar "Update to vX.Y.Z" button → download w/ progress → install (Windows `installMode: passive`) → relaunch. `bundle.createUpdaterArtifacts: true`; capabilities `updater:default` + `process:allow-restart`. **Signing**: keypair at `C:\Users\bwfre\.tauri\pg-model-viewer.key(.pub)` — **no password, BACK IT UP** (lose it = existing installs can't verify future updates); private key is the repo secret `TAURI_SIGNING_PRIVATE_KEY`; pubkey baked into `tauri.conf.json`. The workflow signs builds, collects `.sig` + macOS `.app.tar.gz`, and generates/attaches `latest.json` (windows-x86_64 = nsis exe, darwin-aarch64 = app.tar.gz, linux-x86_64 = AppImage; `.deb` never auto-updates — Tauri limitation). v0.1.1/v0.1.2 users need one final manual download of v0.1.3.

### Extractor fixes (the meat) — `CATALOG_SCHEMA = 4`
1. **Face submesh slicing** (`BASE_FACE_RE`): head/eyes meshes carry brow/lash submeshes that rendered with the wrong material (dark band across the brow). Sliced to submesh 0.
2. **GameObject-transform baking** (now unskinned meshes only): parts like web hats carry real placement transforms. `char = R_std⁻¹·(R_go·v + T_go)`, conjugated by the OBJ exporter's X-flip; no-ops for the two standard forms (identity / -90°X, zero pos). ⚠️ **Cast back to float32** — the f64 bake matrix silently promoted the arrays and the glb buffer (declared FLOAT) became garbage geometry (the "wall of noise" bug).
3. **Schema-driven re-extraction**: old-schema cache ⇒ `model_viewer_status.cache_ready=false` ⇒ UI shows the one-click re-extract; the extractor force-refreshes on schema mismatch (must bump `CATALOG_SCHEMA` in extract.py AND `EXPECTED_CATALOG_SCHEMA` in model_assets.rs together).
4. **Bind-pose correction** (skinned meshes — supersedes #2 for them): some gear was skinned against a rotated/offset/scaled skeleton export (skintight chests = Rz(90°); f2 head = Rz(90°) + ~1m; f2 highelf chest = 0.39× uniform scale). At runtime the shared avatar's bone matrices cancel this; statically we bake `C = inv(refBind[bone]) · meshBind[bone]` where ref = same-sex base body (union of chest/legs/hands/feet bind maps), clustered across bones, identity-skipped. ⚠️ **Known limit**: gear with genuinely non-rigid per-bone binds (m2 highelf/Umrad chest: 12 distinct clusters, ~11cm plate offsets) can't be fixed rigidly — needs real vertex-weight skinning (the deliberate Session-31 rabbit-hole, still skipped).

### ⚠️ Dev-environment gotcha (cost half the session — see memory `project_msix_sandbox_gotcha.md`)
The AI-agent shell is **MSIX-virtualized**: its `%APPDATA%\Roaming` writes shadow into Claude's `LocalCache`, and **GUI apps launched from it inherit the container** — Tauri's asset protocol then canonicalizes into the shadow path, falls outside the `$APPDATA/**` scope, and **403s every icon/mesh** while `invoke()` works. Fixes: `dev-detached.cmd` + scheduled task **`pgmv-dev`** (`schtasks /Run /TN pgmv-dev`) launches dev un-virtualized with CDP on **:9223** (node has native WebSocket — great for reading webview console/probing `asset.localhost` URLs). Real-Roaming writes from the sandbox: write to a plain dir then robocopy via a detached scheduled task.

### Release process
Actions → **Release** → Run workflow → `patch`/`minor`/`major` or `x.y.z`. Bumps 5 version files (`tools/bump_version.py`), tags, builds Win `setup.exe` / macOS `dmg` (arm64) / Linux `deb`+`AppImage` each with the frozen sidecar (`externalBin` injected release-only via inline `--config`), publishes with generated notes. Watch runs with `gh run watch --exit-status` **not piped** (a pipe eats the exit code — bit us once).

### Next up (paper-doll backlog, in rough order)
1. **Weapons in hand** — Character mode skips `is_weapon`; right-hand anchor ≈ hands-0 bbox (x≈+0.55, z≈1.06) + orient + grip offset.
2. **Hair + appearance flags** (`Hair=Off`, `Bra=off`, `Ears=off`) — hair meshes exist in the bundle (`eq-x-f2-hair-*`).
3. **Race selector** — race-letter skin materials already extracted; base-body directives take a race prefix trivially.
4. **Non-rigid skinned gear** (m2 highelf/Umrad chest class) — needs vertex-weight skinning in the extractor; rigid bake shifts some plates ~11cm.
5. Small: web-hat items (`eq-*-head-web-01`) bake to odd positions (a third rig convention, 2 cosmetic items); unsigned binaries (SmartScreen/Gatekeeper warnings).
