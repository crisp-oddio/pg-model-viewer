//! Item catalog state + the two CDN-backed commands the model viewer needs:
//! `ensure_game_data` (lazy-load `items.json` into memory) and `get_icon_path`
//! (fetch/cache an item icon). This is the standalone's tiny replacement for
//! glogger's much larger `cdn_commands` — items + icons only.

use crate::cdn;
use crate::game_data::{items, ItemInfo};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use tokio::sync::RwLock;

/// In-memory item catalog resolved from the PG CDN.
#[derive(Default)]
pub struct GameData {
    pub version: Option<u32>,
    pub items: HashMap<u32, ItemInfo>,
}

impl GameData {
    /// Resolve a loose item reference — a numeric id, an `item_<id>` key, or an
    /// exact display name — to its `ItemInfo`.
    pub fn resolve_item(&self, reference: &str) -> Option<&ItemInfo> {
        if let Ok(id) = reference.parse::<u32>() {
            return self.items.get(&id);
        }
        if let Some(id) = reference.rsplit('_').next().and_then(|s| s.parse::<u32>().ok()) {
            if let Some(it) = self.items.get(&id) {
                return Some(it);
            }
        }
        self.items.values().find(|i| i.name == reference)
    }
}

/// Tauri managed state: the item catalog behind an async RwLock (model_assets
/// reads it via `state.read().await`).
pub type GameDataState = RwLock<GameData>;

/// App data root (`%APPDATA%/<identifier>/`), created if missing.
fn cache_root(app: &AppHandle) -> Result<PathBuf, String> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Cannot resolve app data dir: {e}"))?;
    std::fs::create_dir_all(&base).map_err(|e| format!("Cannot create app data dir: {e}"))?;
    Ok(base)
}

/// Ensure the item catalog is loaded (fetching + caching `items.json` on first
/// run). Idempotent — returns the item count. Called by the frontend on mount
/// before it lists appearance items.
#[tauri::command]
pub async fn ensure_game_data(
    app: AppHandle,
    state: State<'_, GameDataState>,
) -> Result<usize, String> {
    {
        let g = state.read().await;
        if !g.items.is_empty() {
            return Ok(g.items.len());
        }
    }

    let dir = cache_root(&app)?;
    // Prefer the live version; fall back to whatever we cached before (offline).
    let version = match cdn::fetch_remote_version().await {
        Ok(v) => Some(v),
        Err(_) => cdn::read_cached_version(&dir).await,
    };

    let items_path = dir.join("items.json");
    if !items_path.exists() {
        let v = version.ok_or_else(|| {
            "Could not reach the Project: Gorgon CDN and no cached data exists".to_string()
        })?;
        cdn::download_data_file(v, "items", &dir).await?;
        cdn::write_cached_version(&dir, v).await.ok();
    }

    let text = tokio::fs::read_to_string(&items_path)
        .await
        .map_err(|e| format!("Cannot read items.json: {e}"))?;
    let parsed = items::parse(&text)?;
    let n = parsed.len();

    let mut g = state.write().await;
    g.version = version;
    g.items = parsed;
    Ok(n)
}

/// Fetch (and cache) an item icon, returning its local file path for
/// `convertFileSrc`. Requires the catalog to be loaded (for the CDN version).
#[tauri::command]
pub async fn get_icon_path(
    app: AppHandle,
    state: State<'_, GameDataState>,
    icon_id: u32,
) -> Result<String, String> {
    let version = state
        .read()
        .await
        .version
        .ok_or_else(|| "Game data not loaded yet".to_string())?;
    let icon_dir = cache_root(&app)?.join("icons");
    std::fs::create_dir_all(&icon_dir).map_err(|e| format!("Cannot create icon dir: {e}"))?;
    let path = cdn::get_or_fetch_icon(version, icon_id, &icon_dir).await?;
    Ok(path.to_string_lossy().to_string())
}
