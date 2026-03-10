use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::io::Read;
use crate::db::{load_skill_database, SkillDatabase};

#[derive(Deserialize, Debug, Clone)]
pub struct TrackMetadata {
    pub track_id: i32,
    pub surface_id: i32,
    pub dist_type_id: i32,
    pub distance: i32,
    pub direction_key: String,
}

#[derive(Deserialize, Debug)]
pub struct EventMetadata {
    pub event_id: i32,
    pub course_ref: String,
    pub date_label: String,
}

#[derive(Deserialize, Debug)]
pub struct MasterIndex {
    pub events: HashMap<String, EventMetadata>,
    pub courses: HashMap<String, TrackMetadata>,
}

pub fn fetch_index() -> Option<MasterIndex> {
    let url = format!("{}/index.json", env!("BASE_URL"));

    let response = ureq::get(&url).call().ok()?;

    let mut reader = response.into_body().into_reader();
    let mut json_string = String::new();
    reader.read_to_string(&mut json_string).ok()?;

    serde_json::from_str(&json_string).ok()
}

pub fn download_scores(file_name: &str) -> bool {
    let url = format!("{}/{}", env!("BASE_URL"), file_name);

    if let Ok(response) = ureq::get(&url).call() {
        let mut reader = response.into_body().into_reader();
        let mut json_string = String::new();

        if reader.read_to_string(&mut json_string).is_ok() {
            let mut path = crate::db::get_hachimi_base_dir().unwrap_or_default();
            path.push("plugins");
            std::fs::create_dir_all(&path).ok();
            path.push("scores.json");

            if std::fs::write(&path, &json_string).is_ok() {
                let raw_map: std::collections::HashMap<String, crate::data::SkillScoreMatrix> = serde_json::from_str(&json_string).unwrap_or_default();
                let parsed_map: std::collections::HashMap<i32, crate::data::SkillScoreMatrix> = raw_map.into_iter()
                    .filter_map(|(k, v)| k.parse::<i32>().ok().map(|id| (id, v)))
                    .collect();

                let mut cache = crate::data::SKILL_SCORES.lock().unwrap();
                *cache = parsed_map;
                return true;
            }
        }
    }
    false
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SkillScoreMatrix {
    pub runner: Option<f64>,
    pub leader: Option<f64>,
    pub betweener: Option<f64>,
    pub chaser: Option<f64>,
}

impl SkillScoreMatrix {
    pub fn get_score(&self, strategy: i32) -> Option<f64> {
        match strategy {
            1 => self.runner,
            2 => self.leader,
            3 => self.betweener,
            4 => self.chaser,
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerState {
    pub target_strategy: i32,
    pub sort_descending: bool,
    pub sort_mode: i32,
    pub collected_skills: HashMap<i32, String>,
}

#[derive(Debug, Clone, Default)]
pub struct FamilyStats {
    pub total_score: f64,
    pub total_cost: i32,
}

pub static FAMILY_STATS_CACHE: Lazy<Mutex<HashMap<i32, FamilyStats>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn get_state_file_path() -> PathBuf {
    let mut p = crate::db::get_hachimi_base_dir().unwrap_or_default();
    p.push("plugins");
    p.push("skill_helper_state.json");
    p
}

pub fn save_state(state: &OptimizerState) {
    if let Ok(json) = serde_json::to_string(state) {
        let _ = std::fs::write(get_state_file_path(), json);
    }
}

pub fn load_state() -> Option<OptimizerState> {
    if let Ok(json) = std::fs::read_to_string(get_state_file_path()) {
        serde_json::from_str(&json).ok()
    } else {
        None
    }
}

pub static OPTIMIZER_STATE: Lazy<Mutex<OptimizerState>> = Lazy::new(|| {
    let state = load_state().unwrap_or_else(|| OptimizerState {
        target_strategy: 1,
        sort_descending: true,
        sort_mode: 0,
        collected_skills: HashMap::new(),
    });
    Mutex::new(state)
});

pub static SKILL_DB: Lazy<Option<SkillDatabase>> = Lazy::new(|| {
    load_skill_database()
});

pub fn load_scores_from_disk() -> HashMap<i32, SkillScoreMatrix> {
    let mut path = crate::db::get_hachimi_base_dir().unwrap_or_default();
    path.push("plugins");
    path.push("scores.json");

    let json_data = std::fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
    let raw_map: HashMap<String, SkillScoreMatrix> = serde_json::from_str(&json_data).unwrap_or_default();

    raw_map.into_iter()
        .filter_map(|(k, v)| k.parse::<i32>().ok().map(|id| (id, v)))
        .collect()
}

pub static SKILL_SCORES: Lazy<Mutex<HashMap<i32, SkillScoreMatrix>>> = Lazy::new(|| {
    Mutex::new(load_scores_from_disk())
});

pub fn reload_scores() {
    let mut cache = SKILL_SCORES.lock().unwrap();
    *cache = load_scores_from_disk();
}

pub fn get_skill_score_from_map(scores: &HashMap<i32, SkillScoreMatrix>, skill_id: i32, strategy: i32) -> Option<f64> {
    scores.get(&skill_id).and_then(|matrix| matrix.get_score(strategy))
}

pub fn get_skill_score(skill_id: i32, strategy: i32) -> Option<f64> {
    SKILL_SCORES.lock().unwrap().get(&skill_id).and_then(|matrix| matrix.get_score(strategy))
}