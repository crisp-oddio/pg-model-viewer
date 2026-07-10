# PG Model Viewer — Session Handoff

**Date:** 2026-07-09/10 (Session 1 — extraction from glogger, paper-doll polish, v0.1.1 + v0.1.2 shipped)
**Machine:** Windows 11 (primary dev box)
**Branch:** `main` (no branching yet) — everything committed + pushed. Repo is **public**.
**Status:** ✅ `cargo test --lib` 18 pass; `vue-tsc` + `vite build` clean; **v0.1.2 released** with all four installers; user visually verified male + female paper dolls, dye, loadouts, icons in the dev build.

## TL;DR — Session 1

This app is glogger's 3D Model Viewer, extracted into its own repo (and purged from glogger's history — see glogger's Session-33 handoff). Read auto-memory `project_model_viewer.md` first: it holds the deep facts (bundle layout, dye shader, EquipAppearance grammar, coverage, orientation rules).

### App shape
- **Backend** (`src-tauri/src/`): `cdn.rs` + `cdn_commands.rs` (minimal PG CDN layer — `items.json` + icons + `list_dyes`; `GameData` state), `game_data/appearance.rs` (EquipAppearance parser, 14 tests), `game_data/items.rs` (trimmed ItemInfo incl. `dye_color`), `model_assets.rs` (install detection, catalog, `resolve_item_appearance`, `resolve_base_body` [4 tests], extraction command, `EXPECTED_CATALOG_SCHEMA`).
- **Frontend** (`src/`): `components/Character/ModelViewer/*` (screen, TurntableViewer with `buildItem`/`buildCharacter`, DyeControls), `useGorgonDyeMaterial` (dye shader), `modelViewerStore` (loadout + `dyeBySlot` + localStorage presets), `settingsStore` (font + UI scale), `useGameIcon`/`useViewPrefs` (standalone rewrites).
- **Extractor** `tools/model_extractor/extract.py` (+ `build_sidecar.sh` → PyInstaller sidecar in releases).

### Shipped this session
- **v0.1.1** — first release; proved the 3-platform pipeline (exec-bit on `build_sidecar.sh` was the only stumble; bump job is idempotent for re-releases).
- **v0.1.2** — dye dropdowns (all **101 game dyes** from items.json `DyeColor` via `list_dyes`), dye persistence + **saved loadouts**, whole-outfit dye panel in Character mode, settings (font, 50–200% scale), slot-icon fill, and the extractor fixes below.

### Extractor fixes (the meat) — `CATALOG_SCHEMA = 3`
1. **Face submesh slicing** (`BASE_FACE_RE`): head/eyes meshes carry brow/lash submeshes that rendered with the wrong material (dark band across the brow). Sliced to submesh 0.
2. **GameObject-transform baking**: most newmodel parts are authored in bind-pose character space with an inert standard transform (identity or -90°X, zero pos) — but f2 head/teeth/eyelash, m2 teeth, web hats, and foliage armor carry **real placement transforms**; without baking, the female head renders at hip height. `char = R_std⁻¹·(R_go·v + T_go)`, conjugated by the OBJ exporter's X-flip; no-ops (byte-identical) for the standard forms. ⚠️ **Cast back to float32** — the f64 bake matrix silently promoted the arrays and the glb buffer (declared FLOAT) became garbage geometry (the "wall of noise" bug).
3. **Schema-driven re-extraction**: old-schema cache ⇒ `model_viewer_status.cache_ready=false` ⇒ UI shows the one-click re-extract; the extractor force-refreshes on schema mismatch. v0.1.1 users upgrade seamlessly this way.

### ⚠️ Dev-environment gotcha (cost half the session — see memory `project_msix_sandbox_gotcha.md`)
The AI-agent shell is **MSIX-virtualized**: its `%APPDATA%\Roaming` writes shadow into Claude's `LocalCache`, and **GUI apps launched from it inherit the container** — Tauri's asset protocol then canonicalizes into the shadow path, falls outside the `$APPDATA/**` scope, and **403s every icon/mesh** while `invoke()` works. Fixes: `dev-detached.cmd` + scheduled task **`pgmv-dev`** (`schtasks /Run /TN pgmv-dev`) launches dev un-virtualized with CDP on **:9223** (node has native WebSocket — great for reading webview console/probing `asset.localhost` URLs). Real-Roaming writes from the sandbox: write to a plain dir then robocopy via a detached scheduled task.

### Release process
Actions → **Release** → Run workflow → `patch`/`minor`/`major` or `x.y.z`. Bumps 5 version files (`tools/bump_version.py`), tags, builds Win `setup.exe` / macOS `dmg` (arm64) / Linux `deb`+`AppImage` each with the frozen sidecar (`externalBin` injected release-only via inline `--config`), publishes with generated notes. Watch runs with `gh run watch --exit-status` **not piped** (a pipe eats the exit code — bit us once).

### Next up (paper-doll backlog, in rough order)
1. **Weapons in hand** — Character mode skips `is_weapon`; right-hand anchor ≈ hands-0 bbox (x≈+0.55, z≈1.06) + orient + grip offset.
2. **Hair + appearance flags** (`Hair=Off`, `Bra=off`, `Ears=off`) — hair meshes exist in the bundle (`eq-x-f2-hair-*`).
3. **Race selector** — race-letter skin materials already extracted; base-body directives take a race prefix trivially.
4. Small: web-hat items (`eq-*-head-web-01`) bake to odd positions (a third rig convention, 2 cosmetic items); unsigned binaries (SmartScreen/Gatekeeper warnings).
