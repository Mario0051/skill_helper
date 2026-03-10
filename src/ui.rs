use once_cell::sync::Lazy;
use std::ffi::{c_char, c_void, CString};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use crate::plugin_api::{VtableV2, VtableV3};

pub static STRATEGY_LABELS: Lazy<Mutex<Vec<CString>>> = Lazy::new(|| {
    let mut runner = String::from("逃げ");
    let mut leader = String::from("先行");
    let mut betweener = String::from("差し");
    let mut chaser = String::from("追込");

    if let Some(mdb_path) = crate::db::get_master_db_path() {
        if let Ok(conn) = rusqlite::Connection::open_with_flags(&mdb_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY) {
            let query = format!(
                "SELECT `index`, `text` FROM text_data WHERE category = {} AND `index` IN ({}, {}, {}, {})",
                crate::db::DICT_CAT_SURFACE_DIST, 
                crate::db::STRAT_TEXT_IDX_RUNNER, 
                crate::db::STRAT_TEXT_IDX_LEADER, 
                crate::db::STRAT_TEXT_IDX_BETWEENER, 
                crate::db::STRAT_TEXT_IDX_CHASER
            );
            if let Ok(mut stmt) = conn.prepare(&query) {
                if let Ok(mut rows) = stmt.query([]) {
                    while let Ok(Some(row)) = rows.next() {
                        let idx: i32 = row.get(0).unwrap_or(0);
                        let text: String = row.get(1).unwrap_or_default();
                        match idx {
                            2101 => runner = text,
                            2201 => leader = text,
                            2301 => betweener = text,
                            2401 => chaser = text,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    let mut dict_path = crate::db::get_hachimi_base_dir().unwrap_or_default();
    dict_path.push("localized_data");
    dict_path.push("text_data_dict.json");

    if let Ok(json_str) = std::fs::read_to_string(dict_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
            if let Some(category_147) = json.get(crate::db::DICT_CAT_SURFACE_DIST) {
                if let Some(text) = category_147.get(crate::db::STRAT_TEXT_IDX_RUNNER).and_then(|v| v.as_str()) { runner = text.to_string(); }
                if let Some(text) = category_147.get(crate::db::STRAT_TEXT_IDX_LEADER).and_then(|v| v.as_str()) { leader = text.to_string(); }
                if let Some(text) = category_147.get(crate::db::STRAT_TEXT_IDX_BETWEENER).and_then(|v| v.as_str()) { betweener = text.to_string(); }
                if let Some(text) = category_147.get(crate::db::STRAT_TEXT_IDX_CHASER).and_then(|v| v.as_str()) { chaser = text.to_string(); }
            }
        }
    }

    Mutex::new(vec![
        CString::new(runner).unwrap_or_default(),
        CString::new(leader).unwrap_or_default(),
        CString::new(betweener).unwrap_or_default(),
        CString::new(chaser).unwrap_or_default(),
    ])
});

pub struct TrackData {
    pub files: Vec<String>,
    pub names: Vec<String>,
    pub c_labels: Vec<CString>,
    pub values: Vec<i32>,
    pub base_width: f32,
}

pub static TRACK_DATA: Lazy<Mutex<TrackData>> = Lazy::new(|| {
    Mutex::new(TrackData {
        files: Vec::new(),
        names: Vec::new(),
        c_labels: Vec::new(),
        values: Vec::new(),
        base_width: 200.0,
    })
});

pub static TRACK_INDEX: Mutex<i32> = Mutex::new(0);

pub static IS_NETWORKING: AtomicBool = AtomicBool::new(false);

extern "C" fn render_v2_cycler_horizontal(ui: *mut c_void, _userdata: *mut c_void) {
    let vtable_ptr = crate::VTABLE_PTR.load(Ordering::Relaxed);
    if vtable_ptr.is_null() { return; }

    let vtable_v2 = unsafe { &*(vtable_ptr as *const VtableV2) };
    let t_data = TRACK_DATA.lock().unwrap();
    let mut idx = TRACK_INDEX.lock().unwrap();

    if t_data.files.is_empty() { return; }

    unsafe {
        if (vtable_v2.gui_ui_button)(ui, c"< Prev".as_ptr()) {
            *idx = if *idx == 0 { (t_data.files.len() - 1) as i32 } else { *idx - 1 };
        }

        let current_name = CString::new(format!(" Target: {} ", t_data.names[*idx as usize])).unwrap_or_default();
        (vtable_v2.gui_ui_label)(ui, current_name.as_ptr());

        if (vtable_v2.gui_ui_button)(ui, c"Next >".as_ptr()) {
            *idx = (*idx + 1) % (t_data.files.len() as i32);
        }
    }
}

extern "C" fn render_cloud_meta_heading_horizontal(ui: *mut c_void, _userdata: *mut c_void) {
    let vtable_ptr = crate::VTABLE_PTR.load(Ordering::Relaxed);
    if vtable_ptr.is_null() { return; }

    let vtable_v2 = unsafe { &*(vtable_ptr as *const VtableV2) };
    unsafe {
        (vtable_v2.gui_ui_heading)(ui, c"Cloud Meta\nDownloader".as_ptr());
    }
}

pub extern "C" fn render_optimizer_ui(ui: *mut c_void, _userdata: *mut c_void) {
    let vtable_ptr = crate::VTABLE_PTR.load(Ordering::Relaxed);
    if vtable_ptr.is_null() {
        return;
    }

    let version = crate::HACHIMI_VERSION.load(Ordering::Relaxed);
    let vtable_v2 = unsafe { &*(vtable_ptr as *const VtableV2) };

    let mut state = crate::data::OPTIMIZER_STATE.lock().unwrap();
    let mut state_changed = false;

    unsafe {
        (vtable_v2.gui_ui_heading)(ui, c"Skill Helper".as_ptr());

        (vtable_v2.gui_ui_label)(ui, c"Target Strategy:".as_ptr());

        let labels_lock = STRATEGY_LABELS.lock().unwrap();

        if version >= 3 {
            let vtable_v3 = &*(vtable_ptr as *const VtableV3);
            let strategy_ptrs: Vec<*const c_char> = vec![
                labels_lock[0].as_ptr(),
                labels_lock[1].as_ptr(),
                labels_lock[2].as_ptr(),
                labels_lock[3].as_ptr(),
            ];
            let strategy_values = [
                crate::db::STRAT_ID_RUNNER, 
                crate::db::STRAT_ID_LEADER, 
                crate::db::STRAT_ID_BETWEENER, 
                crate::db::STRAT_ID_CHASER
            ];
            let mut current_strat = state.target_strategy;

            (vtable_v3.gui_ui_searchable_combobox)(
                ui,
                c"strategy_selector".as_ptr(),
                &mut current_strat,
                strategy_values.as_ptr(),
                strategy_ptrs.as_ptr(),
                4,
            );

            if current_strat != state.target_strategy {
                state.target_strategy = current_strat;
                state_changed = true;
            }
        } else {
            let mut is_runner = state.target_strategy == crate::db::STRAT_ID_RUNNER;
            let mut is_leader = state.target_strategy == crate::db::STRAT_ID_LEADER;
            let mut is_betweener = state.target_strategy == crate::db::STRAT_ID_BETWEENER;
            let mut is_chaser = state.target_strategy == crate::db::STRAT_ID_CHASER;

            if (vtable_v2.gui_ui_checkbox)(ui, labels_lock[0].as_ptr(), &mut is_runner) && is_runner {
                state.target_strategy = 1; state_changed = true;
            }
            if (vtable_v2.gui_ui_checkbox)(ui, labels_lock[1].as_ptr(), &mut is_leader) && is_leader {
                state.target_strategy = 2; state_changed = true;
            }
            if (vtable_v2.gui_ui_checkbox)(ui, labels_lock[2].as_ptr(), &mut is_betweener) && is_betweener {
                state.target_strategy = 3; state_changed = true;
            }
            if (vtable_v2.gui_ui_checkbox)(ui, labels_lock[3].as_ptr(), &mut is_chaser) && is_chaser {
                state.target_strategy = 4; state_changed = true;
            }
        }

        drop(labels_lock);

        (vtable_v2.gui_ui_separator)(ui);

        (vtable_v2.gui_ui_label)(ui, c"Sort Mode:".as_ptr());

        if version >= 3 {
            let vtable_v3 = &*(vtable_ptr as *const VtableV3);

            let sort_ptrs = [
                c"Total Score".as_ptr(),
                c"Point Cost".as_ptr(),
                c"Score Efficiency".as_ptr(),
            ];
            let sort_values = [0, 1, 2];
            let mut current_sort = state.sort_mode;

            (vtable_v3.gui_ui_searchable_combobox)(
                ui,
                c"sort_mode_selector".as_ptr(),
                &mut current_sort,
                sort_values.as_ptr(),
                sort_ptrs.as_ptr(),
                3,
            );

            if current_sort != state.sort_mode {
                state.sort_mode = current_sort;
                state_changed = true;
            }
        } else {
            let mut is_score = state.sort_mode == 0;
            let mut is_cost = state.sort_mode == 1;
            let mut is_eff = state.sort_mode == 2;

            if (vtable_v2.gui_ui_checkbox)(ui, c"Total Score".as_ptr(), &mut is_score) && is_score {
                state.sort_mode = 0; state_changed = true;
            }
            if (vtable_v2.gui_ui_checkbox)(ui, c"Point Cost".as_ptr(), &mut is_cost) && is_cost {
                state.sort_mode = 1; state_changed = true;
            }
            if (vtable_v2.gui_ui_checkbox)(ui, c"Efficiency".as_ptr(), &mut is_eff) && is_eff {
                state.sort_mode = 2; state_changed = true;
            }
        }

        let mut desc = state.sort_descending;
        (vtable_v2.gui_ui_checkbox)(ui, c"Sort Descending".as_ptr(), &mut desc);
        if desc != state.sort_descending {
            state.sort_descending = desc;
            state_changed = true;
        }

        (vtable_v2.gui_ui_separator)(ui);

        if (vtable_v2.gui_ui_button)(ui, c"Clear Processed Skills Cache".as_ptr()) {
            state_changed = true;
        }

        if (vtable_v2.gui_ui_button)(ui, c"Reload Local scores.json".as_ptr()) {
            crate::data::reload_scores();
            crate::hooks::trigger_list_refresh();
        }

        (vtable_v2.gui_ui_separator)(ui);

        (vtable_v2.gui_ui_horizontal)(ui, Some(render_cloud_meta_heading_horizontal), std::ptr::null_mut());

        let is_busy = IS_NETWORKING.load(Ordering::Relaxed);

        if is_busy {
            (vtable_v2.gui_ui_colored_label)(ui, 150, 150, 150, 255, c"Processing Network Request...".as_ptr());
        } else {
            if (vtable_v2.gui_ui_button)(ui, c"1. Fetch Available Tracks".as_ptr()) {
                if version >= 3 {
                    let vtable_v3 = &*(vtable_ptr as *const VtableV3);

                    let current_w = (vtable_v3.gui_get_menu_width)();
                    TRACK_DATA.lock().unwrap().base_width = current_w;
                }

                IS_NETWORKING.store(true, Ordering::Relaxed);
                std::thread::spawn(|| {
                    if let Some(index) = crate::data::fetch_index() {
                        let mut t_data = TRACK_DATA.lock().unwrap();
                        t_data.names.clear();
                        t_data.files.clear();
                        t_data.c_labels.clear();

                        for (key, e_meta) in &index.events {
                            if let Some(c_meta) = index.courses.get(&e_meta.course_ref) {
                                let display = crate::db::build_event_name(key, e_meta.event_id, &e_meta.date_label, c_meta);

                                t_data.names.push(display.clone());
                                let safe_display = display.replace('\0', "");
                                t_data.c_labels.push(std::ffi::CString::new(safe_display).unwrap_or_default());
                                t_data.files.push(format!("event_{}.json", key));
                                let current_len = t_data.values.len() as i32;
                                t_data.values.push(current_len);
                            }
                        }

                        for (id, c_meta) in &index.courses {
                            let display = crate::db::build_track_name(c_meta);

                            t_data.names.push(display.clone());
                            let safe_display = display.replace('\0', "");t_data.c_labels.push(std::ffi::CString::new(safe_display).unwrap_or_default());
                            t_data.files.push(format!("course_{}.json", id));
                            let current_len = t_data.values.len() as i32;
                            t_data.values.push(current_len);
                        }
                    }
                    IS_NETWORKING.store(false, Ordering::Relaxed);
                });
            }
        }

        let has_files = !TRACK_DATA.lock().unwrap().files.is_empty();

        if has_files {
            if version >= 3 {
                let vtable_v3 = &*(vtable_ptr as *const VtableV3);
                let t_data = TRACK_DATA.lock().unwrap();
                let mut idx_lock = TRACK_INDEX.lock().unwrap();

                let label_ptrs: Vec<*const c_char> = t_data.c_labels.iter().map(|c| c.as_ptr()).collect();
                let mut current_idx = *idx_lock;

                let control_w = t_data.base_width;

                (vtable_v3.gui_ui_searchable_combobox)(
                    ui,
                    c"track_selector".as_ptr(),
                    &mut current_idx,
                    t_data.values.as_ptr(),
                    label_ptrs.as_ptr(),
                    t_data.values.len()
                );

                if current_idx != *idx_lock {
                    *idx_lock = current_idx;
                    (vtable_v3.gui_set_menu_width)(control_w);
                }

            } else {
                (vtable_v2.gui_ui_horizontal)(ui, Some(render_v2_cycler_horizontal), std::ptr::null_mut());
            }

            if !is_busy {
                if (vtable_v2.gui_ui_button)(ui, c"2. Download & Apply Meta".as_ptr()) {
                    let file_to_dl = {
                        let t_data = TRACK_DATA.lock().unwrap();
                        let idx = TRACK_INDEX.lock().unwrap();
                        t_data.files[*idx as usize].clone()
                    };

                    IS_NETWORKING.store(true, Ordering::Relaxed);
                    std::thread::spawn(move || {
                        if crate::data::download_scores(&file_to_dl) {
                            crate::hooks::trigger_list_refresh();
                        }
                        IS_NETWORKING.store(false, Ordering::Relaxed);
                    });
                }
            }
        }
    }

    if state_changed {
        crate::data::save_state(&state);
        crate::hooks::trigger_list_refresh();
    }
}