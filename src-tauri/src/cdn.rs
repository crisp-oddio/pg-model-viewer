//! Minimal Project: Gorgon CDN access — just what the model viewer needs:
//! the live data version, the `items.json` catalog (for item → EquipAppearance),
//! and item icons. Ported from glogger's `cdn.rs` (the item/icon subset).
//!
//! Version check:  GET http://client.projectgorgon.com/fileversion.txt  → integer
//! Data files:     https://cdn.projectgorgon.com/v{ver}/data/{file}.json
//! Icons:          https://cdn.projectgorgon.com/v{ver}/icons/icon_{id}.png

use std::path::{Path, PathBuf};
use tokio::fs;

const VERSION_URL: &str = "http://client.projectgorgon.com/fileversion.txt";
const CDN_BASE: &str = "https://cdn.projectgorgon.com";

/// Fetch the current live data version from the game's version endpoint.
pub async fn fetch_remote_version() -> Result<u32, String> {
    let text = reqwest::get(VERSION_URL)
        .await
        .map_err(|e| format!("Version fetch failed: {e}"))?
        .text()
        .await
        .map_err(|e| format!("Version read failed: {e}"))?;
    text.trim()
        .parse::<u32>()
        .map_err(|e| format!("Version parse failed '{text}': {e}"))
}

/// Read the version number we last downloaded, if any.
pub async fn read_cached_version(cache_dir: &Path) -> Option<u32> {
    let text = fs::read_to_string(cache_dir.join("version.txt")).await.ok()?;
    text.trim().parse().ok()
}

/// Persist the version number we just downloaded.
pub async fn write_cached_version(cache_dir: &Path, version: u32) -> Result<(), String> {
    fs::write(cache_dir.join("version.txt"), version.to_string())
        .await
        .map_err(|e| format!("Failed to write cached version: {e}"))
}

/// Download one JSON data file to `cache_dir/{name}.json`.
pub async fn download_data_file(version: u32, name: &str, cache_dir: &Path) -> Result<(), String> {
    let url = format!("{CDN_BASE}/v{version}/data/{name}.json");
    let bytes = reqwest::get(&url)
        .await
        .map_err(|e| format!("Download failed for {name}: {e}"))?
        .bytes()
        .await
        .map_err(|e| format!("Read failed for {name}: {e}"))?;
    fs::write(cache_dir.join(format!("{name}.json")), &bytes)
        .await
        .map_err(|e| format!("Write failed for {name}: {e}"))
}

/// Local path for a cached icon.
pub fn icon_path(icon_dir: &Path, icon_id: u32) -> PathBuf {
    icon_dir.join(format!("icon_{icon_id}.png"))
}

/// Local path for an icon, fetching + caching it first if needed.
pub async fn get_or_fetch_icon(
    version: u32,
    icon_id: u32,
    icon_dir: &Path,
) -> Result<PathBuf, String> {
    let path = icon_path(icon_dir, icon_id);
    if path.exists() {
        return Ok(path);
    }
    let url = format!("{CDN_BASE}/v{version}/icons/icon_{icon_id}.png");
    let bytes = reqwest::get(&url)
        .await
        .map_err(|e| format!("Icon fetch failed for {icon_id}: {e}"))?
        .bytes()
        .await
        .map_err(|e| format!("Icon read failed for {icon_id}: {e}"))?;
    fs::write(&path, &bytes)
        .await
        .map_err(|e| format!("Icon write failed for {icon_id}: {e}"))?;
    Ok(path)
}
