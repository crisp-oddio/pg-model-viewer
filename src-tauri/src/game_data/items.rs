//! PG `items.json` → `ItemInfo`. Trimmed from glogger to the fields the model
//! viewer needs: identity, icon, equip slot, keywords, and the 3D
//! `EquipAppearance` directives.

use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// A single item definition. Only the fields the Model Viewer consumes are
/// promoted to typed fields; the rest of the CDN record is dropped.
#[derive(Debug, Serialize, Clone)]
pub struct ItemInfo {
    pub id: u32,
    pub name: String,
    pub icon_id: Option<u32>,
    pub internal_name: Option<String>,
    pub keywords: Vec<String>,
    pub equip_slot: Option<String>,

    // ── Equipped 3D appearance (the whole point of this app) ────────────
    // Raw PG appearance directives, e.g.
    //   "@Chest=@eq-{sex}-mage-chest-01(^Armor=mage-body%DYE%);Bra=off"
    // `equip_appearance2` is the modern (m2/f2) model set; prefer it when
    // present, falling back to `equip_appearance`. Parsed by
    // `game_data::appearance::parse_equip_appearance`.
    pub equip_appearance: Option<String>,
    pub equip_appearance2: Option<String>,
}

/// Parse the CDN `items.json` object (keys like `Item_1234`) into a map by id.
pub fn parse(json: &str) -> Result<HashMap<u32, ItemInfo>, String> {
    let raw: HashMap<String, Value> = serde_json::from_str(json)
        .map_err(|e| format!("items.json: parse error at line {}, col {}: {e}", e.line(), e.column()))?;

    let mut items = HashMap::with_capacity(raw.len());
    for (key, value) in raw {
        // Key is `Item_<id>`; take the trailing numeric segment.
        let id: u32 = match key.rsplit('_').next().and_then(|s| s.parse().ok()) {
            Some(id) => id,
            None => continue,
        };
        items.insert(
            id,
            ItemInfo {
                id,
                name: str_field(&value, "Name").unwrap_or_else(|| format!("Unknown Item {id}")),
                icon_id: u32_field(&value, "IconId"),
                internal_name: str_field(&value, "InternalName"),
                keywords: str_array_field(&value, "Keywords"),
                equip_slot: str_field(&value, "EquipSlot"),
                equip_appearance: str_field(&value, "EquipAppearance"),
                equip_appearance2: str_field(&value, "EquipAppearance2"),
            },
        );
    }
    Ok(items)
}

fn str_field(value: &Value, key: &str) -> Option<String> {
    value.get(key)?.as_str().map(|s| s.to_string())
}

fn u32_field(value: &Value, key: &str) -> Option<u32> {
    value.get(key)?.as_u64().map(|n| n as u32)
}

fn str_array_field(value: &Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default()
}
