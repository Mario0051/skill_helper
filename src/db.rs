use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;
use std::path::PathBuf;
use std::ffi::CStr;
use std::sync::{atomic::Ordering, Mutex};
use once_cell::sync::Lazy;
use crate::{HACHIMI_VERSION, VTABLE_PTR};
use crate::data::TrackMetadata;
use crate::plugin_api::VtableV3;

pub const DICT_CAT_RACE: &str = "33";
pub const DICT_CAT_TRACK: &str = "35";
pub const DICT_CAT_SKILL_NAME: &str = "47";
pub const DICT_CAT_SURFACE_DIST: &str = "147";
pub const DICT_CAT_LOH: &str = "274";

pub const STRAT_ID_RUNNER: i32 = 1;
pub const STRAT_ID_LEADER: i32 = 2;
pub const STRAT_ID_BETWEENER: i32 = 3;
pub const STRAT_ID_CHASER: i32 = 4;

pub const STRAT_TEXT_IDX_RUNNER: &str = "2101";
pub const STRAT_TEXT_IDX_LEADER: &str = "2201";
pub const STRAT_TEXT_IDX_BETWEENER: &str = "2301";
pub const STRAT_TEXT_IDX_CHASER: &str = "2401";

pub static TEXT_DATA_DICT: Lazy<Mutex<HashMap<String, HashMap<String, String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static LOCALIZE_DICT: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct SkillTier {
    pub id: i32,
    pub group_id: i32,
    pub group_rate: i32,
    pub name: Option<String>,
}

pub struct SkillDatabase {
    pub grouped_skills: HashMap<i32, Vec<SkillTier>>,
    pub skill_to_group: HashMap<i32, i32>,
}

pub fn get_hachimi_base_dir() -> Option<PathBuf> {
    if let Ok(env_dir) = std::env::var("HACHIMI_BASE_DIR") {
        return Some(PathBuf::from(env_dir));
    }

    let version = HACHIMI_VERSION.load(Ordering::Relaxed);
    let ptr = VTABLE_PTR.load(Ordering::Relaxed);

    if version >= 3 && !ptr.is_null() {
        let vtable_v3 = unsafe { &*(ptr as *const VtableV3) };
        let c_path = unsafe { CStr::from_ptr((vtable_v3.hachimi_get_base_dir)()) };
        Some(PathBuf::from(c_path.to_str().unwrap_or("")))
    } else {
        let mut p = std::env::current_exe().unwrap_or_default();
        p.pop();
        p.push("hachimi");
        Some(p)
    }
}

pub fn get_hachimi_data_path() -> Option<PathBuf> {
    if let Ok(env_dir) = std::env::var("HACHIMI_DATA_DIR") {
        return Some(PathBuf::from(env_dir));
    }

    let version = HACHIMI_VERSION.load(Ordering::Relaxed);
    let ptr = VTABLE_PTR.load(Ordering::Relaxed);

    if version >= 3 && !ptr.is_null() {
        let vtable_v3 = unsafe { &*(ptr as *const VtableV3) };
        let c_path = unsafe { CStr::from_ptr((vtable_v3.hachimi_get_data_path)()) };
        return Some(PathBuf::from(c_path.to_str().unwrap_or("")));
    }

    let user_profile = std::env::var("USERPROFILE").unwrap_or_default();

    let fallback_paths = vec![
        PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps\common\UmamusumePrettyDerby_Jpn\UmamusumePrettyDerby_Jpn_Data\Persistent"),
        PathBuf::from(&user_profile).join(r"AppData\LocalLow\Cygames\umamusume"),
        PathBuf::from(&user_profile).join(r"Umamusume\umamusume_Data\Persistent"),
        PathBuf::from(&user_profile).join(r"AppData\LocalLow\Cygames\UmamusumePrettyDerby_Jpn"),
    ];

    for path in fallback_paths {
        if path.exists() {
            return Some(path);
        }
    }

    None
}

pub fn get_master_db_path() -> Option<PathBuf> {
    if let Some(mut path) = get_hachimi_data_path() {
        path.push("master");
        path.push("master.mdb");
        if path.exists() {
            return Some(path);
        }
    }

    None
}

pub fn load_skill_database() -> Option<SkillDatabase> {
    let path = get_master_db_path()?;
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY).ok()?;
    let mut translations: HashMap<String, String> = HashMap::new();

    if let Some(base_dir) = get_hachimi_base_dir() {
        let localized_dir = base_dir.join("localized_data");

        if let Ok(data) = std::fs::read_to_string(localized_dir.join("text_data_dict.json")) {
            if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&data) {
                if let Ok(nested_map) = serde_json::from_value::<HashMap<String, HashMap<String, String>>>(json_val.clone()) {
                    *TEXT_DATA_DICT.lock().unwrap() = nested_map;
                }

                if let Some(cat_47) = json_val.get(DICT_CAT_SKILL_NAME).and_then(|v| v.as_object()) {
                    for (k, v) in cat_47 {
                        if let Some(s) = v.as_str() {
                            translations.insert(k.clone(), s.to_string());
                        }
                    }
                }
            }
        }

        if let Ok(data) = std::fs::read_to_string(localized_dir.join("localize_dict.json")) {
            if let Ok(flat_map) = serde_json::from_str::<HashMap<String, String>>(&data) {
                *LOCALIZE_DICT.lock().unwrap() = flat_map;
            }
        }
    }

    let query = format!("SELECT s.id, s.group_id, s.group_rate, t.text FROM skill_data s LEFT JOIN text_data t ON s.id = t.[index] AND t.category = {}", DICT_CAT_SKILL_NAME);
    let mut stmt = conn.prepare(&query).ok()?;

    let skill_iter = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let mut name: Option<String> = row.get(3)?;

        if let Some(translated) = translations.get(&id.to_string()) {
            name = Some(translated.clone());
        }

        Ok(SkillTier {
            id,
            group_id: row.get(1)?,
            group_rate: row.get(2)?,
            name,
        })
    }).ok()?;

    let mut grouped_skills: HashMap<i32, Vec<SkillTier>> = HashMap::new();
    let mut skill_to_group: HashMap<i32, i32> = HashMap::new();

    for skill in skill_iter.flatten() {
        skill_to_group.insert(skill.id, skill.group_id);
        grouped_skills.entry(skill.group_id).or_default().push(skill);
    }

    for group in grouped_skills.values_mut() {
        group.sort_by_key(|s| s.group_rate);
    }

    Some(SkillDatabase { grouped_skills, skill_to_group })
}

pub fn build_track_name(meta: &TrackMetadata) -> String {
    let game_dict = TEXT_DATA_DICT.lock().unwrap();
    let local_dict = LOCALIZE_DICT.lock().unwrap();

    let track = game_dict.get(DICT_CAT_TRACK)
        .and_then(|cat: &HashMap<String, String>| cat.get(&meta.track_id.to_string()))
        .map(|s| s.as_str())
        .unwrap_or("Unknown");

    let surface = game_dict.get(DICT_CAT_SURFACE_DIST)
        .and_then(|cat: &HashMap<String, String>| cat.get(&meta.surface_id.to_string()))
        .map(|s| s.as_str())
        .unwrap_or("");

    let dist_type = game_dict.get(DICT_CAT_SURFACE_DIST)
        .and_then(|cat| cat.get(&meta.dist_type_id.to_string()))
        .map(|s| format!("({})", s))
        .unwrap_or_default();

    let direction = local_dict.get(&meta.direction_key)
        .map(|s| s.as_str())
        .unwrap_or("");

    format!("{} {} {}m {} {}", track, surface, meta.distance, dist_type, direction)
        .replace("  ", " ")
        .trim()
        .to_string()
}

pub fn build_event_name(event_key: &str, event_id: i32, date_label: &str, meta: &TrackMetadata) -> String {
    let event_name = {
        let game_dict = TEXT_DATA_DICT.lock().unwrap();
        let category = if event_key.contains("chm") { DICT_CAT_RACE } else { DICT_CAT_LOH };

        game_dict.get(category)
            .and_then(|cat| cat.get(&event_id.to_string()))
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Event".to_string())
    };

    let track_details = build_track_name(meta);

    format!("【{}】{} ({})", date_label, event_name, track_details)
}