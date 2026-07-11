use std::ffi::{c_char, c_void, CString};
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::il2cpp_types::{Il2CppArray, Il2CppObject, Il2CppString};
use crate::plugin_api::VtableV2;
use crate::{get_real_target_addr, VTABLE_PTR};

#[cfg(target_os = "windows")]
pub static UPDATE_CURRENT_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
#[cfg(target_os = "android")]
pub static UPDATE_SKILL_NAME_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static SETUP_SCROLL_LIST_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static PLAY_OUT_VIEW_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static EVENT_SYSTEM_UPDATE_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static DIALOG_SKILL_HINT_OPEN_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
#[cfg(target_os = "windows")]
pub static PARTS_SKILL_LIST_ITEM_UPDATE_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
#[cfg(target_os = "android")]
pub static PARTS_SKILL_LIST_ITEM_SETUP_NEED_SKILL_POINT_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static PARTS_SKILL_LIST_CONTAINER_UPDATE_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static DECK_SKILL_ITEM_UPDATE_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static SETUP_SKILL_CONTENT_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static PARTS_SINGLE_MODE_SKILL_LIST_SETUP_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static FACTOR_LIST_ITEM_SETUP_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static GET_FILTERED_FACTOR_GROUP_LIST_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static FACTOR_SELECT_SHOW_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static FACTOR_SELECT_HIDE_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static FACTOR_SELECT_INSTANCE: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static IL2CPP_STRING_NEW: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

pub static IL2CPP_GCHANDLE_NEW: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static IL2CPP_GCHANDLE_GET_TARGET: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static IL2CPP_GCHANDLE_FREE: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

pub static VC_INSTANCE: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static SKILL_LIST_PTR: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static IS_OMISSION: AtomicBool = AtomicBool::new(false);
pub static NEEDS_REFRESH: AtomicBool = AtomicBool::new(false);

pub static TRACKED_LEARNING_ITEMS: Lazy<Mutex<Vec<u32>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static TRACKED_INNER_ITEMS: Lazy<Mutex<Vec<(u32, i32)>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static TRACKED_DECK_ITEMS: Lazy<Mutex<Vec<(u32, i32)>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub static ACTIVE_PARTS_LIST_HANDLE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
pub static ACTIVE_SETUP_PARAM_HANDLE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
pub static ACTIVE_RESOURCE_HASH: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1);

pub static LEARNING_LIST_ORIGINAL_ORDER: Lazy<Mutex<Vec<i32>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static CONTAINER_ORIGINAL_ORDER: Lazy<Mutex<HashMap<usize, Vec<i32>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static TRANSFORM_ORIGINAL_ORDER: Lazy<Mutex<HashMap<(usize, i32), (usize, i32, i32)>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static HINT_ORIGINAL_ORDER: Lazy<Mutex<HashMap<usize, Vec<i32>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static SETUP_LIST_ORIGINAL_ORDER: Lazy<Mutex<HashMap<usize, Vec<i32>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub unsafe fn get_valid_target(handle: u32) -> *mut Il2CppObject {
    let get_target_ptr = IL2CPP_GCHANDLE_GET_TARGET.load(Ordering::Relaxed);
    if get_target_ptr.is_null() { return std::ptr::null_mut(); }
    let get_target: extern "C" fn(u32) -> *mut Il2CppObject = std::mem::transmute(get_target_ptr);

    let obj = get_target(handle);
    if obj.is_null() { return std::ptr::null_mut(); }

    let cached_ptr = *(((obj as *mut u8).add(0x10)) as *const usize);
    if cached_ptr == 0 { return std::ptr::null_mut(); }

    obj
}

pub unsafe fn install(vtable: &VtableV2, interceptor: *mut c_void, image: *mut c_void) {
    IL2CPP_STRING_NEW.store((vtable.il2cpp_resolve_symbol)(c"il2cpp_string_new".as_ptr()), Ordering::SeqCst);
    IL2CPP_GCHANDLE_NEW.store((vtable.il2cpp_resolve_symbol)(c"il2cpp_gchandle_new".as_ptr()), Ordering::SeqCst);
    IL2CPP_GCHANDLE_GET_TARGET.store((vtable.il2cpp_resolve_symbol)(c"il2cpp_gchandle_get_target".as_ptr()), Ordering::SeqCst);
    IL2CPP_GCHANDLE_FREE.store((vtable.il2cpp_resolve_symbol)(c"il2cpp_gchandle_free".as_ptr()), Ordering::SeqCst);

    let class_learning_item = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"PartsSingleModeSkillLearningListItem".as_ptr());

    #[cfg(target_os = "windows")]
    {
        let update_current_addr = (vtable.il2cpp_get_method_addr)(class_learning_item, c"UpdateCurrent".as_ptr(), 0);
        if !update_current_addr.is_null() {
            UPDATE_CURRENT_ORIG.store((vtable.interceptor_hook)(
                interceptor, crate::get_real_target_addr(update_current_addr as *mut u8), update_current_hook as *mut c_void
            ), Ordering::SeqCst);
        }
    }

    #[cfg(target_os = "android")]
    {
        let update_name_addr = (vtable.il2cpp_get_method_addr)(class_learning_item, c"UpdateSkillName".as_ptr(), 1);
        if !update_name_addr.is_null() {
            UPDATE_SKILL_NAME_ORIG.store((vtable.interceptor_hook)(
                interceptor, crate::get_real_target_addr(update_name_addr as *mut u8), update_skill_name_hook as *mut c_void
            ), Ordering::SeqCst);
        }
    }

    let class_vc = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"SingleModeSkillLearningViewController".as_ptr());
    let setup_scroll_list_addr = (vtable.il2cpp_get_method_addr)(class_vc, c"SetupScrollList".as_ptr(), 3);
    SETUP_SCROLL_LIST_ORIG.store((vtable.interceptor_hook)(
        interceptor, get_real_target_addr(setup_scroll_list_addr as *mut u8), setup_scroll_list_hook as *mut c_void
    ), Ordering::SeqCst);

    let play_out_addr = (vtable.il2cpp_get_method_addr)(class_vc, c"PlayOutView".as_ptr(), 0);
    PLAY_OUT_VIEW_ORIG.store((vtable.interceptor_hook)(
        interceptor, get_real_target_addr(play_out_addr as *mut u8), play_out_view_hook as *mut c_void
    ), Ordering::SeqCst);

    let ui_image = (vtable.il2cpp_get_assembly_image)(c"UnityEngine.UI".as_ptr());
    let es_class = (vtable.il2cpp_get_class)(ui_image, c"UnityEngine.EventSystems".as_ptr(), c"EventSystem".as_ptr());
    let es_update_addr = (vtable.il2cpp_get_method_addr)(es_class, c"Update".as_ptr(), 0);
    if !es_update_addr.is_null() {
        EVENT_SYSTEM_UPDATE_ORIG.store((vtable.interceptor_hook)(
            interceptor, get_real_target_addr(es_update_addr as *mut u8), event_system_update_hook as *mut c_void
        ), Ordering::SeqCst);
    }

    let class_skill_hint = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"DialogSkillHint".as_ptr());
    let hint_open_addr = (vtable.il2cpp_get_method_addr)(class_skill_hint, c"Open".as_ptr(), 1);
    if !hint_open_addr.is_null() {
        DIALOG_SKILL_HINT_OPEN_ORIG.store((vtable.interceptor_hook)(
            interceptor, get_real_target_addr(hint_open_addr as *mut u8), dialog_skill_hint_open_hook as *mut c_void
        ), Ordering::SeqCst);
    }

    let class_skill_list_item = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"PartsSingleModeSkillListItem".as_ptr());

    #[cfg(target_os = "windows")]
    {
        let update_item_addr = (vtable.il2cpp_get_method_addr)(class_skill_list_item, c"UpdateItem".as_ptr(), 4);
        if !update_item_addr.is_null() {
            PARTS_SKILL_LIST_ITEM_UPDATE_ORIG.store((vtable.interceptor_hook)(
                interceptor, crate::get_real_target_addr(update_item_addr as *mut u8), parts_skill_list_item_update_hook as *mut c_void
            ), Ordering::SeqCst);
        }
    }

    #[cfg(target_os = "android")]
    {
        let setup_need_skill_point_addr = (vtable.il2cpp_get_method_addr)(class_skill_list_item, c"SetupNeedSkillPoint".as_ptr(), 0);
        if !setup_need_skill_point_addr.is_null() {
            PARTS_SKILL_LIST_ITEM_SETUP_NEED_SKILL_POINT_ORIG.store((vtable.interceptor_hook)(
                interceptor, crate::get_real_target_addr(setup_need_skill_point_addr as *mut u8), parts_skill_list_item_setup_need_skill_point_hook as *mut c_void
            ), Ordering::SeqCst);
        }
    }

    let class_skill_container = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"PartsSingleModeSkillListItemContainer".as_ptr());
    let update_container_addr = (vtable.il2cpp_get_method_addr)(class_skill_container, c"UpdateItem".as_ptr(), 2);
    if !update_container_addr.is_null() {
        PARTS_SKILL_LIST_CONTAINER_UPDATE_ORIG.store((vtable.interceptor_hook)(
            interceptor, crate::get_real_target_addr(update_container_addr as *mut u8), parts_skill_list_container_update_hook as *mut c_void
        ), Ordering::SeqCst);
    }

    let class_deck_skill_item = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"PartsSupportCardDeckSkillListItem".as_ptr());
    let deck_update_item_addr = (vtable.il2cpp_get_method_addr)(class_deck_skill_item, c"UpdateItem".as_ptr(), 2);
    if !deck_update_item_addr.is_null() {
        DECK_SKILL_ITEM_UPDATE_ORIG.store((vtable.interceptor_hook)(
            interceptor, crate::get_real_target_addr(deck_update_item_addr as *mut u8), deck_skill_item_update_hook as *mut c_void
        ), Ordering::SeqCst);
    }

    let class_deck_dialog = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"DialogSupportCardDeckEffectList".as_ptr());
    let setup_skill_content_addr = (vtable.il2cpp_get_method_addr)(class_deck_dialog, c"SetupSkillContent".as_ptr(), 2);
    if !setup_skill_content_addr.is_null() {
        SETUP_SKILL_CONTENT_ORIG.store((vtable.interceptor_hook)(
            interceptor, crate::get_real_target_addr(setup_skill_content_addr as *mut u8), setup_skill_content_hook as *mut c_void
        ), Ordering::SeqCst);
    }

    let class_single_mode_skill_list = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"PartsSingleModeSkillList".as_ptr());
    let setup_list_addr = (vtable.il2cpp_get_method_addr)(class_single_mode_skill_list, c"Setup".as_ptr(), 2);
    if !setup_list_addr.is_null() {
        PARTS_SINGLE_MODE_SKILL_LIST_SETUP_ORIG.store((vtable.interceptor_hook)(
            interceptor, crate::get_real_target_addr(setup_list_addr as *mut u8), parts_single_mode_skill_list_setup_hook as *mut c_void
        ), Ordering::SeqCst);
    }

    let class_factor_item = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"GenerateSuccessionCharaPriorityFactorGroupListItem".as_ptr());
    let factor_setup_addr = (vtable.il2cpp_get_method_addr)(class_factor_item, c"Setup".as_ptr(), 7);
    if !factor_setup_addr.is_null() {
        FACTOR_LIST_ITEM_SETUP_ORIG.store((vtable.interceptor_hook)(
            interceptor, crate::get_real_target_addr(factor_setup_addr as *mut u8), factor_list_item_setup_hook as *mut c_void
        ), Ordering::SeqCst);
    }

    let class_factor_model = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"GenerateSuccessionCharaPriorityFactorSelectModel".as_ptr());
    let get_filtered_list_addr = (vtable.il2cpp_get_method_addr)(class_factor_model, c"get_FilteredFactorGroupList".as_ptr(), 0);
    if !get_filtered_list_addr.is_null() {
        GET_FILTERED_FACTOR_GROUP_LIST_ORIG.store((vtable.interceptor_hook)(
            interceptor,
            crate::get_real_target_addr(get_filtered_list_addr as *mut u8),
            get_filtered_factor_group_list_hook as *mut c_void
        ), Ordering::SeqCst);
    }

    let class_factor_select = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"GenerateSuccessionCharaPriorityFactorSelect".as_ptr());
    if !class_factor_select.is_null() {
        let show_addr = (vtable.il2cpp_get_method_addr)(class_factor_select, c"Show".as_ptr(), 0);
        if !show_addr.is_null() {
            FACTOR_SELECT_SHOW_ORIG.store((vtable.interceptor_hook)(
                interceptor, crate::get_real_target_addr(show_addr as *mut u8), factor_select_show_hook as *mut c_void
            ), Ordering::SeqCst);
        }

        let hide_addr = (vtable.il2cpp_get_method_addr)(class_factor_select, c"Hide".as_ptr(), 1);
        if !hide_addr.is_null() {
            FACTOR_SELECT_HIDE_ORIG.store((vtable.interceptor_hook)(
                interceptor, crate::get_real_target_addr(hide_addr as *mut u8), factor_select_hide_hook as *mut c_void
            ), Ordering::SeqCst);
        }
    }
}

pub fn trigger_list_refresh() {
    NEEDS_REFRESH.store(true, Ordering::SeqCst);
}

unsafe fn apply_live_ui_updates() {
    let vtable = &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2);

    let state = { crate::data::OPTIMIZER_STATE.lock().unwrap().clone() };
    let strategy = state.target_strategy;
    let desc = state.sort_descending;
    let enable_scoring = state.enable_scoring;

    let free_handle: extern "C" fn(u32) = std::mem::transmute(IL2CPP_GCHANDLE_FREE.load(Ordering::Relaxed));

    let learning_items: Vec<usize> = {
        let mut guard = TRACKED_LEARNING_ITEMS.lock().unwrap();
        let mut valid = Vec::new();
        guard.retain(|&handle| {
            let obj = get_valid_target(handle);
            if obj.is_null() { free_handle(handle); return false; }
            valid.push(obj as usize);
            true
        });
        valid
    };

    let inner_items: Vec<(usize, i32)> = {
        let mut guard = TRACKED_INNER_ITEMS.lock().unwrap();
        let mut valid = Vec::new();
        guard.retain(|&(handle, skill_id)| {
            let obj = get_valid_target(handle);
            if obj.is_null() { free_handle(handle); return false; }
            valid.push((obj as usize, skill_id));
            true
        });
        valid
    };

    let deck_items: Vec<(usize, i32)> = {
        let mut guard = TRACKED_DECK_ITEMS.lock().unwrap();
        let mut valid = Vec::new();
        guard.retain(|&(handle, skill_id)| {
            let obj = get_valid_target(handle);
            if obj.is_null() { free_handle(handle); return false; }
            valid.push((obj as usize, skill_id));
            true
        });
        valid
    };

    for ptr in learning_items {
        update_learning_item_text(ptr as *mut Il2CppObject, vtable, &state);
    }

    update_and_sort_item_group(inner_items, strategy, desc, enable_scoring, vtable, c"PartsSingleModeSkillListItem", c"_nameText");

    update_and_sort_item_group(deck_items, strategy, desc, enable_scoring, vtable, c"PartsSupportCardDeckSkillListItem", c"_name");
}

unsafe fn update_and_sort_item_group(
    items: Vec<(usize, i32)>,
    strategy: i32,
    desc: bool,
    enable_scoring: bool,
    vtable: &crate::plugin_api::VtableV2,
    _class_name: &std::ffi::CStr,
    text_field_name: &std::ffi::CStr
) {
    if items.is_empty() { return; }

    let core_module = (vtable.il2cpp_get_assembly_image)(c"UnityEngine.CoreModule".as_ptr());
    let component_class = (vtable.il2cpp_get_class)(core_module, c"UnityEngine".as_ptr(), c"Component".as_ptr());
    let transform_class = (vtable.il2cpp_get_class)(core_module, c"UnityEngine".as_ptr(), c"Transform".as_ptr());
    let object_class = (vtable.il2cpp_get_class)(core_module, c"UnityEngine".as_ptr(), c"Object".as_ptr());

    let get_game_object_addr = (vtable.il2cpp_get_method_addr_cached)(component_class, c"get_gameObject".as_ptr(), 0);
    let get_transform_addr = (vtable.il2cpp_get_method_addr_cached)(component_class, c"get_transform".as_ptr(), 0);
    let get_parent_addr = (vtable.il2cpp_get_method_addr_cached)(transform_class, c"get_parent".as_ptr(), 0);
    let get_sibling_index_addr = (vtable.il2cpp_get_method_addr_cached)(transform_class, c"GetSiblingIndex".as_ptr(), 0);
    let set_sibling_index_addr = (vtable.il2cpp_get_method_addr_cached)(transform_class, c"SetSiblingIndex".as_ptr(), 1);
    let set_parent_addr = (vtable.il2cpp_get_method_addr_cached)(transform_class, c"SetParent".as_ptr(), 2);
    let get_name_addr = (vtable.il2cpp_get_method_addr_cached)(object_class, c"get_name".as_ptr(), 0);

    let ui_image = (vtable.il2cpp_get_assembly_image)(c"UnityEngine.UI".as_ptr());
    let text_class = (vtable.il2cpp_get_class)(ui_image, c"UnityEngine.UI".as_ptr(), c"Text".as_ptr());
    let get_text_addr = (vtable.il2cpp_get_method_addr_cached)(text_class, c"get_text".as_ptr(), 0);
    let set_text_addr = (vtable.il2cpp_get_method_addr_cached)(text_class, c"set_text".as_ptr(), 1);

    let string_new: extern "C" fn(*const std::ffi::c_char) -> *mut Il2CppString = std::mem::transmute(IL2CPP_STRING_NEW.load(std::sync::atomic::Ordering::Relaxed));

    if get_transform_addr.is_null() || get_parent_addr.is_null() || get_sibling_index_addr.is_null()
        || set_sibling_index_addr.is_null() || get_text_addr.is_null() || set_text_addr.is_null()
        || get_game_object_addr.is_null() || set_parent_addr.is_null()
        || get_name_addr.is_null() {
        return;
    }

    let get_game_object: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_game_object_addr);
    let get_transform: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_transform_addr);
    let get_parent: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_parent_addr);
    let get_sibling_index: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(get_sibling_index_addr);
    let set_sibling_index: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(set_sibling_index_addr);
    let set_parent: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject, bool) = std::mem::transmute(set_parent_addr);
    let get_name: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppString = std::mem::transmute(get_name_addr);
    let get_text: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppString = std::mem::transmute(get_text_addr);
    let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(set_text_addr);

    let is_inner = _class_name.to_bytes() == b"PartsSingleModeSkillListItem";
    let mut active_items = Vec::new();

    for &(item_val, skill_id) in items.iter() {
        let item_ptr = item_val as *mut Il2CppObject;

        let go = get_game_object(item_ptr);
        if go.is_null() { continue; }

        let score = crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0);

        let item_class = (*item_ptr).klass;
        let name_field = (vtable.il2cpp_get_field_from_name)(item_class, text_field_name.as_ptr());

        if !name_field.is_null() {
            let mut name_text_obj: *mut Il2CppObject = std::ptr::null_mut();
            (vtable.il2cpp_get_field_value)(item_ptr as *mut std::ffi::c_void, name_field, &mut name_text_obj as *const _ as *mut std::ffi::c_void);

            if !name_text_obj.is_null() {
                let current_str = (*get_text(name_text_obj)).as_string();
                let base_str = current_str.split("<color=").next().unwrap_or(&current_str).trim_end();

                let new_text = if score > 0.0 && enable_scoring {
                    format!("{} <color=#ffb000>[{:.2}pt]</color>", base_str, score)
                } else {
                    base_str.to_string()
                };

                if new_text != current_str {
                    if let Ok(c_str) = std::ffi::CString::new(new_text.replace('\0', "")) {
                        set_text(name_text_obj, string_new(c_str.as_ptr()));
                    }
                }
            }
        }

        let transform = get_transform(item_ptr);
        if !transform.is_null() {
            let parent = get_parent(transform);
            let grandparent = if !parent.is_null() { get_parent(parent) } else { std::ptr::null_mut() };

            let mut is_loop = false;
            let mut is_hint = false;
            let mut curr = parent;
            for _ in 0..8 {
                if curr.is_null() { break; }
                let curr_go = get_game_object(curr);
                if !curr_go.is_null() {
                    let name_str = get_name(curr_go);
                    if !name_str.is_null() {
                        let name = (*name_str).as_string();
                        if name.contains("Loop") { is_loop = true; }
                        if name.contains("DialogSkillHint") { is_hint = true; }
                        if name.contains("UpgradeSelect") { is_loop = true; }
                    }
                }
                curr = get_parent(curr);
            }

            if is_loop { continue; }

            let mut managed_by_global = false;
            if is_inner {
                let global_map = crate::hooks::SETUP_LIST_ORIGINAL_ORDER.lock().unwrap();
                for orig_order in global_map.values() {
                    if orig_order.contains(&skill_id) { managed_by_global = true; break; }
                }
            }
            if managed_by_global { continue; }

            let transform_ptr = transform as usize;
            let mut order_map = crate::hooks::TRANSFORM_ORIGINAL_ORDER.lock().unwrap();
            let map_key = (transform_ptr, skill_id);
            if !order_map.contains_key(&map_key) {
                let current_p_idx = get_sibling_index(parent);
                let current_t_idx = get_sibling_index(transform);
                order_map.insert(map_key, (parent as usize, current_p_idx, current_t_idx));
            }
            drop(order_map);

            active_items.push((item_ptr, transform, parent, grandparent, score, skill_id, is_hint));
        }
    }

    if is_inner {
        let mut roots: std::collections::HashMap<usize, Vec<(*mut Il2CppObject, *mut Il2CppObject, f64, i32, bool)>> = std::collections::HashMap::new();

        for (item_ptr, transform, parent, grandparent, score, skill_id, is_hint) in active_items {
            if parent.is_null() { continue; }
            let root = if !grandparent.is_null() { grandparent } else { parent };
            roots.entry(root as usize).or_default().push((item_ptr, transform, score, skill_id, is_hint));
        }

        for (_, mut group) in roots {
            if group.is_empty() { continue; }

            let mut available_slots = Vec::new();
            let order_map = crate::hooks::TRANSFORM_ORIGINAL_ORDER.lock().unwrap();
            for item in group.iter() {
                let transform_ptr = item.1 as usize;
                let skill_id = item.3;
                if let Some(&(orig_parent, p_idx, t_idx)) = order_map.get(&(transform_ptr, skill_id)) {
                    available_slots.push((orig_parent as *mut Il2CppObject, p_idx, t_idx));
                }
            }
            drop(order_map);

            available_slots.sort_by_key(|slot| (slot.1, slot.2));

            group.sort_by(|a, b| {
                if !enable_scoring {
                    let skill_a = a.3;
                    let skill_b = b.3;
                    let is_hint_a = a.4;
                    let is_hint_b = b.4;

                    let get_sort_key = |skill_id: i32, transform_ptr: usize, is_hint: bool| -> (usize, usize) {
                        if is_hint {
                            let hint_map = crate::hooks::HINT_ORIGINAL_ORDER.lock().unwrap();
                            for orig_order in hint_map.values() {
                                if let Some(pos) = orig_order.iter().position(|&id| id == skill_id) {
                                    return (pos, 0);
                                }
                            }
                        }

                        let order_map = crate::hooks::TRANSFORM_ORIGINAL_ORDER.lock().unwrap();
                        let &(orig_parent, p_idx, t_idx) = order_map.get(&(transform_ptr, skill_id)).unwrap_or(&(0, 0, 0));
                        drop(order_map);

                        let container_map = crate::hooks::CONTAINER_ORIGINAL_ORDER.lock().unwrap();
                        if let Some(orig_order) = container_map.get(&orig_parent) {
                            if let Some(pos) = orig_order.iter().position(|&id| id == skill_id) {
                                return (p_idx as usize, pos);
                            }
                        }
                        drop(container_map);

                        (p_idx as usize, t_idx as usize)
                    };

                    let key_a = get_sort_key(skill_a, a.1 as usize, is_hint_a);
                    let key_b = get_sort_key(skill_b, b.1 as usize, is_hint_b);

                    return key_a.cmp(&key_b);
                }

                if desc {
                    b.2.total_cmp(&a.2)
                } else {
                    a.2.total_cmp(&b.2)
                }
            });

            for (i, item) in group.iter().enumerate() {
                if i >= available_slots.len() { break; }
                let target_slot = available_slots[i];
                let item_transform = item.1;

                let current_parent = get_parent(item_transform);

                if current_parent != target_slot.0 {
                    set_parent(item_transform, target_slot.0, false);
                }

                set_sibling_index(item_transform, target_slot.2);
            }
        }
    } else {
        let mut grouped_items: std::collections::HashMap<usize, Vec<(*mut Il2CppObject, *mut Il2CppObject, f64, i32, i32)>> = std::collections::HashMap::new();

        for (item_ptr, transform, parent, _grandparent, score, skill_id, _is_hint) in active_items {
            if parent.is_null() { continue; }
            let orig_idx = get_sibling_index(transform);
            grouped_items.entry(parent as usize).or_default().push((item_ptr, transform, score, orig_idx, skill_id));
        }

        for (_, mut group) in grouped_items {
            group.sort_by_key(|&(_, _, _, idx, _)| idx);

            let mut runs: Vec<Vec<(*mut Il2CppObject, *mut Il2CppObject, f64, i32, i32)>> = Vec::new();
            let mut current_run = Vec::new();

            for item in group {
                if current_run.is_empty() {
                    current_run.push(item);
                } else if item.3 == current_run.last().unwrap().3 + 1 {
                    current_run.push(item);
                } else {
                    runs.push(current_run);
                    current_run = vec![item];
                }
            }
            if !current_run.is_empty() {
                runs.push(current_run);
            }

            for mut run in runs {
                let orig_indices: Vec<i32> = run.iter().map(|&(_, _, _, idx, _)| idx).collect();

                run.sort_by(|a, b| {
                    if !enable_scoring {
                        let order_map = crate::hooks::TRANSFORM_ORIGINAL_ORDER.lock().unwrap();
                        let a_slot = order_map.get(&(a.1 as usize, a.4)).unwrap_or(&(0, 0, 0));
                        let b_slot = order_map.get(&(b.1 as usize, b.4)).unwrap_or(&(0, 0, 0));
                        return a_slot.2.cmp(&b_slot.2);
                    }

                    if desc {
                        b.2.total_cmp(&a.2)
                    } else {
                        a.2.total_cmp(&b.2)
                    }
                });

                for (i, &(_, sort_transform, _, _, _)) in run.iter().enumerate() {
                    set_sibling_index(sort_transform, orig_indices[i]);
                }
            }
        }
    }
}

extern "C" fn event_system_update_hook(this: *mut c_void) {
    if NEEDS_REFRESH.swap(false, Ordering::SeqCst) {
        let vc = VC_INSTANCE.load(Ordering::Relaxed);
        let list = SKILL_LIST_PTR.load(Ordering::Relaxed);

        if !vc.is_null() && !list.is_null() {
            unsafe {
                sort_and_collect_skills(list);
                let orig_ptr = SETUP_SCROLL_LIST_ORIG.load(Ordering::Relaxed);
                if !orig_ptr.is_null() {
                    let is_omission = IS_OMISSION.load(Ordering::Relaxed);
                    let orig_fn: extern "C" fn(*mut c_void, *mut c_void, bool, f32) = std::mem::transmute(orig_ptr);
                    orig_fn(vc, list, is_omission, 0.0);
                }
            }
        }

        unsafe {
            let list_handle = ACTIVE_PARTS_LIST_HANDLE.load(Ordering::SeqCst);
            let param_handle = ACTIVE_SETUP_PARAM_HANDLE.load(Ordering::SeqCst);

            if list_handle != 0 && param_handle != 0 {
                let list_obj = get_valid_target(list_handle);
                let param_obj = get_valid_target(param_handle);

                if !list_obj.is_null() && !param_obj.is_null() {
                    let vtable = &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2);
                    sort_setup_parameter_list(param_obj, vtable);

                    let hash = ACTIVE_RESOURCE_HASH.load(Ordering::SeqCst);
                    let orig_ptr = PARTS_SINGLE_MODE_SKILL_LIST_SETUP_ORIG.load(Ordering::Relaxed);
                    if !orig_ptr.is_null() {
                        let orig_fn: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject, i32) = std::mem::transmute(orig_ptr);
                        orig_fn(list_obj, param_obj, hash);
                    }
                }
            }
        }

        unsafe {
            let factor_select_instance = FACTOR_SELECT_INSTANCE.load(Ordering::SeqCst);
            if !factor_select_instance.is_null() {
                let vtable = &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2);
                let factor_select_class = (*(factor_select_instance as *mut Il2CppObject)).klass;

                let on_value_changed_addr = (vtable.il2cpp_get_method_addr_cached)(factor_select_class, c"OnValueChangedInputName".as_ptr(), 0);
                if !on_value_changed_addr.is_null() {
                    let on_value_changed: extern "C" fn(*mut c_void) = std::mem::transmute(on_value_changed_addr);
                    on_value_changed(factor_select_instance);
                }
            }

            apply_live_ui_updates();
        }
    }

    let orig_ptr = EVENT_SYSTEM_UPDATE_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut c_void) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this);
    }
}

pub unsafe fn sort_and_collect_skills(skill_info_list: *mut c_void) {
    let vtable = &*(VTABLE_PTR.load(Ordering::Relaxed) as *const VtableV2);

    let list_class = *(skill_info_list as *mut *mut c_void);
    let size_field = (vtable.il2cpp_get_field_from_name)(list_class, c"_size".as_ptr());
    let items_field = (vtable.il2cpp_get_field_from_name)(list_class, c"_items".as_ptr());

    if size_field.is_null() || items_field.is_null() { return; }

    let mut size: i32 = 0;
    (vtable.il2cpp_get_field_value)(skill_info_list, size_field, &mut size as *mut _ as _);

    let mut items_array: *mut Il2CppArray = std::ptr::null_mut();
    (vtable.il2cpp_get_field_value)(skill_info_list, items_field, &mut items_array as *mut _ as _);

    if items_array.is_null() { return; }

    let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
    let db_opt = crate::data::SKILL_DB.as_ref();

    struct SkillSortItem {
        orig_idx: usize,
        ptr: *mut Il2CppObject,
        group_id: i32,
        primary_id: i32,
    }

    let mut sortable_infos = Vec::with_capacity(size as usize);
    let mut family_unique_skills: std::collections::HashMap<i32, std::collections::HashMap<i32, f64>> = std::collections::HashMap::new();
    let mut family_max_costs: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    for i in 0..size as usize {
        let obj = (*items_array).get_obj(i);
        if obj.is_null() { continue; }

        let skill_info_class = (*obj).klass;
        let inner_list_field = (vtable.il2cpp_get_field_from_name)(skill_info_class, c"<SkillList>k__BackingField".as_ptr());
        if inner_list_field.is_null() { continue; }

        let mut inner_list: *mut c_void = std::ptr::null_mut();
        (vtable.il2cpp_get_field_value)(obj as _, inner_list_field, &mut inner_list as *mut _ as _);
        if inner_list.is_null() { continue; }

        let inner_list_class = *(inner_list as *mut *mut c_void);
        let inner_size_field = (vtable.il2cpp_get_field_from_name)(inner_list_class, c"_size".as_ptr());
        let inner_items_field = (vtable.il2cpp_get_field_from_name)(inner_list_class, c"_items".as_ptr());

        if inner_size_field.is_null() || inner_items_field.is_null() { continue; }

        let mut inner_size: i32 = 0;
        (vtable.il2cpp_get_field_value)(inner_list, inner_size_field, &mut inner_size as *mut _ as _);

        let mut inner_items_array: *mut Il2CppArray = std::ptr::null_mut();
        (vtable.il2cpp_get_field_value)(inner_list, inner_items_field, &mut inner_items_array as *mut _ as _);

        if inner_items_array.is_null() { continue; }

        let mut container_cost = 0;
        let mut primary_id = i32::MAX;
        let mut temp_skills = Vec::new();

        let scores_lock = crate::data::SKILL_SCORES.lock().unwrap();

        for j in 0..inner_size as usize {
            let item_info = (*inner_items_array).get_obj(j);
            if item_info.is_null() { continue; }

            let item_class = (*item_info).klass;

            let skill_id_field = (vtable.il2cpp_get_field_from_name)(item_class, c"<SkillId>k__BackingField".as_ptr());
            let mut skill_id: i32 = 0;
            if !skill_id_field.is_null() {
                (vtable.il2cpp_get_field_value)(item_info as _, skill_id_field, &mut skill_id as *mut _ as _);
            }

            if primary_id == i32::MAX {
                primary_id = skill_id;
            }

            let is_acquired_field = (vtable.il2cpp_get_field_from_name)(item_class, c"<IsAcquired>k__BackingField".as_ptr());
            let mut is_acquired: bool = false;
            if !is_acquired_field.is_null() {
                (vtable.il2cpp_get_field_value)(item_info as _, is_acquired_field, &mut is_acquired as *mut _ as _);
            }

            let get_cost_addr = (vtable.il2cpp_get_method_addr_cached)(item_class, c"get_CurrentNeedPoint".as_ptr(), 0);
            let cost = if !get_cost_addr.is_null() {
                let get_cost: extern "C" fn(*mut c_void) -> i32 = std::mem::transmute(get_cost_addr);
                get_cost(item_info as _)
            } else { 0 };

            if !is_acquired {
                container_cost += cost;
                let score = crate::data::get_skill_score_from_map(&scores_lock, skill_id, state.target_strategy).unwrap_or(0.0);
                temp_skills.push((skill_id, score));
            }
        }

        let group_id = if let Some(db) = db_opt {
            *db.skill_to_group.get(&primary_id).unwrap_or(&primary_id)
        } else {
            primary_id
        };

        let fam_map = family_unique_skills.entry(group_id).or_insert_with(std::collections::HashMap::new);
        for (sid, score) in temp_skills {
            fam_map.insert(sid, score);
        }

        let current_max = family_max_costs.entry(group_id).or_insert(0);
        if container_cost > *current_max {
            *current_max = container_cost;
        }

        sortable_infos.push(SkillSortItem {
            orig_idx: i,
            ptr: obj,
            group_id,
            primary_id,
        });
    }

    let mut group_total_scores = std::collections::HashMap::new();
    for (g_id, skills) in &family_unique_skills {
        let t_score: f64 = skills.values().sum();
        group_total_scores.insert(*g_id, t_score);
    }

    {
        let mut cache = crate::data::FAMILY_STATS_CACHE.lock().unwrap();
        cache.clear();
        for (g_id, score) in &group_total_scores {
            let cost = family_max_costs.get(g_id).unwrap_or(&0);
            cache.insert(*g_id, crate::data::FamilyStats {
                total_score: *score,
                total_cost: *cost,
            });
        }
    }

    let mut orig_order_guard = LEARNING_LIST_ORIGINAL_ORDER.lock().unwrap();
    if orig_order_guard.is_empty() {
        for info in &sortable_infos {
            orig_order_guard.push(info.primary_id);
        }
    }
    let orig_order = orig_order_guard.clone();
    drop(orig_order_guard);

    let desc = state.sort_descending;
    let mode = state.sort_mode;
    let enable_scoring = state.enable_scoring;

    sortable_infos.sort_by(|a, b| {
        if !enable_scoring || mode == 3 {
            let idx_a = orig_order.iter().position(|&id| id == a.primary_id).unwrap_or(usize::MAX);
            let idx_b = orig_order.iter().position(|&id| id == b.primary_id).unwrap_or(usize::MAX);
            return idx_a.cmp(&idx_b);
        }

        let score_a = group_total_scores[&a.group_id];
        let score_b = group_total_scores[&b.group_id];

        let cost_a = family_max_costs[&a.group_id];
        let cost_b = family_max_costs[&b.group_id];

        let val_a = match mode {
            1 => if cost_a == 0 { f64::MAX } else { cost_a as f64 },
            2 => if cost_a == 0 { -1.0 } else { (score_a * 100.0) / (cost_a as f64) },
            _ => score_a,
        };

        let val_b = match mode {
            1 => if cost_b == 0 { f64::MAX } else { cost_b as f64 },
            2 => if cost_b == 0 { -1.0 } else { (score_b * 100.0) / (cost_b as f64) },
            _ => score_b,
        };

        let cmp = if desc {
            val_b.total_cmp(&val_a)
        } else {
            val_a.total_cmp(&val_b)
        };

        cmp.then_with(|| a.group_id.cmp(&b.group_id))
           .then_with(|| a.orig_idx.cmp(&b.orig_idx))
    });

    for (i, item) in sortable_infos.iter().enumerate() {
        (*items_array).set_obj(i, item.ptr);
    }
}

extern "C" fn play_out_view_hook(this: *mut c_void) -> *mut c_void {
    VC_INSTANCE.store(std::ptr::null_mut(), Ordering::SeqCst);
    SKILL_LIST_PTR.store(std::ptr::null_mut(), Ordering::SeqCst);

    unsafe {
        let free_handle: extern "C" fn(u32) = std::mem::transmute(IL2CPP_GCHANDLE_FREE.load(Ordering::Relaxed));
        if let Ok(mut items) = TRACKED_LEARNING_ITEMS.lock() {
            for &handle in items.iter() { free_handle(handle); }
            items.clear();
        }
        if let Ok(mut items) = TRACKED_INNER_ITEMS.lock() {
            for &(handle, _) in items.iter() { free_handle(handle); }
            items.clear();
        }
        if let Ok(mut items) = TRACKED_DECK_ITEMS.lock() {
            for &(handle, _) in items.iter() { free_handle(handle); }
            items.clear();
        }

        LEARNING_LIST_ORIGINAL_ORDER.lock().unwrap().clear();

        let old_list = ACTIVE_PARTS_LIST_HANDLE.swap(0, Ordering::SeqCst);
        if old_list != 0 { free_handle(old_list); }
        let old_param = ACTIVE_SETUP_PARAM_HANDLE.swap(0, Ordering::SeqCst);
        if old_param != 0 { free_handle(old_param); }
    }

    let orig_ptr = PLAY_OUT_VIEW_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut c_void) -> *mut c_void = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this)
    } else {
        std::ptr::null_mut()
    }
}

extern "C" fn setup_scroll_list_hook(
    this: *mut c_void,
    skill_info_list: *mut c_void,
    is_enable_skill_description_omission: bool,
    initial_anchored_position: f32
) {
    VC_INSTANCE.store(this, Ordering::SeqCst);
    SKILL_LIST_PTR.store(skill_info_list, Ordering::SeqCst);
    IS_OMISSION.store(is_enable_skill_description_omission, Ordering::SeqCst);

    if !skill_info_list.is_null() {
        unsafe { sort_and_collect_skills(skill_info_list); }
    }

    let orig_ptr = SETUP_SCROLL_LIST_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut c_void, *mut c_void, bool, f32) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this, skill_info_list, is_enable_skill_description_omission, initial_anchored_position);
    }
}

unsafe fn update_learning_item_text(this: *mut Il2CppObject, vtable: &VtableV2, state: &crate::data::OptimizerState) {
    let cached_ptr = *(this.cast::<u8>().add(0x10) as *const usize);
    if cached_ptr == 0 { return; }

    let image = (vtable.il2cpp_get_assembly_image)(c"umamusume".as_ptr());
    let class = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"PartsSingleModeSkillLearningListItem".as_ptr());

    let get_top_info: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject =
        std::mem::transmute((vtable.il2cpp_get_method_addr_cached)(class, c"GetTopInfo".as_ptr(), 0));
    let info_obj = get_top_info(this);
    if info_obj.is_null() { return; }

    let info_class = (*info_obj).klass;

    let get_skill_id_addr = (vtable.il2cpp_get_method_addr_cached)(info_class, c"get_SkillId".as_ptr(), 0);
    let skill_id = if !get_skill_id_addr.is_null() {
        let get_skill_id: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(get_skill_id_addr);
        get_skill_id(info_obj)
    } else { 0 };

    let get_cost_addr = (vtable.il2cpp_get_method_addr_cached)(info_class, c"get_CurrentNeedPoint".as_ptr(), 0);
    let cost = if !get_cost_addr.is_null() {
        let get_cost: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(get_cost_addr);
        get_cost(info_obj)
    } else { 0 };

    let name_text_obj: *mut Il2CppObject = std::ptr::null_mut();
    (vtable.il2cpp_get_field_value)(this as *mut c_void, (vtable.il2cpp_get_field_from_name)(class, c"_nameText".as_ptr()), &name_text_obj as *const _ as *mut c_void);
    if name_text_obj.is_null() { return; }

    let ui_image = (vtable.il2cpp_get_assembly_image)(c"UnityEngine.UI".as_ptr());
    let text_class = (vtable.il2cpp_get_class)(ui_image, c"UnityEngine.UI".as_ptr(), c"Text".as_ptr());
    let get_text: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppString = std::mem::transmute((vtable.il2cpp_get_method_addr_cached)(text_class, c"get_text".as_ptr(), 0));
    let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute((vtable.il2cpp_get_method_addr_cached)(text_class, c"set_text".as_ptr(), 1));

    let current_str = (*get_text(name_text_obj)).as_string();
    let base_str = current_str.split("<color=").next().unwrap_or(&current_str).trim_end();

    let strategy = state.target_strategy;
    let sort_mode = state.sort_mode;
    let enable_scoring = state.enable_scoring;

    let group_id = if let Some(db) = crate::data::SKILL_DB.as_ref() {
        *db.skill_to_group.get(&skill_id).unwrap_or(&skill_id)
    } else {
        skill_id
    };

    let (fam_score, fam_cost) = {
        let cache = crate::data::FAMILY_STATS_CACHE.lock().unwrap();
        if let Some(stats) = cache.get(&group_id) {
            (stats.total_score, stats.total_cost)
        } else {
            (0.0, 0)
        }
    };

    let individual_score = crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0);

    let new_text = if enable_scoring {
        if individual_score > 0.0 && cost > 0 {
            if sort_mode == 2 {
                let ind_eff = (individual_score * 100.0) / (cost as f64);
                if fam_cost > 0 && (fam_cost > cost || (fam_score - individual_score).abs() > 0.01) {
                    let fam_eff = (fam_score * 100.0) / (fam_cost as f64);
                    format!("{} <color=#ffb000>[{:.2}pt|{:.2}e]</color> <color=#bbbbbb>(T:{:.2}e)</color>", base_str, individual_score, ind_eff, fam_eff)
                } else {
                    format!("{} <color=#ffb000>[{:.2}pt|{:.2}e]</color>", base_str, individual_score, ind_eff)
                }
            } else {
                if fam_cost > 0 && (fam_cost > cost || (fam_score - individual_score).abs() > 0.01) {
                    format!("{} <color=#ffb000>[{:.2}pt]</color> <color=#bbbbbb>(T:{:.2}pt)</color>", base_str, individual_score, fam_score)
                } else {
                    format!("{} <color=#ffb000>[{:.2}pt]</color>", base_str, individual_score)
                }
            }
        } else {
            if cost == 0 && individual_score > 0.0 {
                format!("{} <color=#777777>[{:.2}pt (Acq)]</color>", base_str, individual_score)
            } else {
                format!("{} <color=#777777>[-- pt]</color>", base_str)
            }
        }
    } else {
        base_str.to_string()
    };

    if new_text != current_str {
        let string_new: extern "C" fn(*const c_char) -> *mut Il2CppString = std::mem::transmute(IL2CPP_STRING_NEW.load(Ordering::Relaxed));
        set_text(name_text_obj, string_new(CString::new(new_text).unwrap().as_ptr()));
    }
}

#[cfg(target_os = "windows")]
extern "C" fn update_current_hook(this: *mut Il2CppObject) {
    let orig_ptr = UPDATE_CURRENT_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppObject) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this);
    }

    unsafe {
        let new_handle: extern "C" fn(*mut Il2CppObject, bool) -> u32 = std::mem::transmute(IL2CPP_GCHANDLE_NEW.load(Ordering::Relaxed));
        let free_handle: extern "C" fn(u32) = std::mem::transmute(IL2CPP_GCHANDLE_FREE.load(Ordering::Relaxed));

        if let Ok(mut items) = TRACKED_LEARNING_ITEMS.lock() {
            items.retain(|&handle| {
                let obj = get_valid_target(handle);
                if obj.is_null() || obj == this { free_handle(handle); return false; }
                true
            });
            items.push(new_handle(this, false));
        }

        let vtable = &*(VTABLE_PTR.load(Ordering::Relaxed) as *const VtableV2);
        let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
        update_learning_item_text(this, vtable, &state);
    }
}

#[cfg(target_os = "android")]
extern "C" fn update_skill_name_hook(this: *mut Il2CppObject, info: *mut Il2CppObject) {
    let orig_ptr = UPDATE_SKILL_NAME_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this, info);
    }

    unsafe {
        let new_handle: extern "C" fn(*mut Il2CppObject, bool) -> u32 = std::mem::transmute(IL2CPP_GCHANDLE_NEW.load(Ordering::Relaxed));
        let free_handle: extern "C" fn(u32) = std::mem::transmute(IL2CPP_GCHANDLE_FREE.load(Ordering::Relaxed));

        if let Ok(mut items) = TRACKED_LEARNING_ITEMS.lock() {
            items.retain(|&handle| {
                let obj = get_valid_target(handle);
                if obj.is_null() || obj == this { free_handle(handle); return false; }
                true
            });
            items.push(new_handle(this, false));
        }

        let vtable = &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2);
        let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
        update_learning_item_text(this, vtable, &state);
    }
}

extern "C" fn dialog_skill_hint_open_hook(skill_data_array: *mut Il2CppArray) {
    if !skill_data_array.is_null() {
        unsafe {
            let arr_ptr = skill_data_array as usize;
            let mut order_map = HINT_ORIGINAL_ORDER.lock().unwrap();
            let slice = (*skill_data_array).get_i32_mut_slice();

            let mut needs_insert = true;
            if let Some(orig) = order_map.get(&arr_ptr) {
                let mut c_sorted = slice.to_vec();
                c_sorted.sort_unstable();
                let mut o_sorted = orig.clone();
                o_sorted.sort_unstable();
                if c_sorted == o_sorted { needs_insert = false; }
            }

            if needs_insert {
                order_map.insert(arr_ptr, slice.to_vec());
            }
        }
    }

    let orig_ptr = DIALOG_SKILL_HINT_OPEN_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppArray) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(skill_data_array);
    }

    crate::hooks::trigger_list_refresh();
}

#[cfg(target_os = "windows")]
extern "C" fn parts_skill_list_item_update_hook(
    this: *mut Il2CppObject,
    skill_info: *mut Il2CppObject,
    is_plate_effect_enable: bool,
    adjuster_data: *mut Il2CppObject,
    resource_hash: i32
) {
    let orig_ptr = PARTS_SKILL_LIST_ITEM_UPDATE_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject, bool, *mut Il2CppObject, i32) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this, skill_info, is_plate_effect_enable, adjuster_data, resource_hash);
    }

    if skill_info.is_null() || this.is_null() { return; }

    let vtable = unsafe { &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2) };
    unsafe {
        let info_class = (*skill_info).klass;
        let mut skill_id = 0;

        let get_id_addr = (vtable.il2cpp_get_method_addr_cached)(info_class, c"get_Id".as_ptr(), 0);
        if !get_id_addr.is_null() {
            let get_id: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(get_id_addr);
            skill_id = get_id(skill_info);
        } else {
            let backing_field = (vtable.il2cpp_get_field_from_name)(info_class, c"<Id>k__BackingField".as_ptr());
            if !backing_field.is_null() {
                (vtable.il2cpp_get_field_value)(skill_info as *mut c_void, backing_field, &mut skill_id as *mut _ as *mut c_void);
            }
        }

        if skill_id == 0 { return; }

        let new_handle: extern "C" fn(*mut Il2CppObject, bool) -> u32 = std::mem::transmute(IL2CPP_GCHANDLE_NEW.load(Ordering::Relaxed));
        let free_handle: extern "C" fn(u32) = std::mem::transmute(IL2CPP_GCHANDLE_FREE.load(Ordering::Relaxed));

        if let Ok(mut items) = TRACKED_INNER_ITEMS.lock() {
            items.retain(|&(handle, _)| {
                let obj = get_valid_target(handle);
                if obj.is_null() || obj == this { free_handle(handle); return false; }
                true
            });
            items.push((new_handle(this, false), skill_id));
        }

        let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
        let strategy = state.target_strategy;
        let enable_scoring = state.enable_scoring;
        let score = crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0);

        if score > 0.0 || !enable_scoring {
            let item_class = (*this).klass;
            let name_text_obj: *mut Il2CppObject = std::ptr::null_mut();
            (vtable.il2cpp_get_field_value)(this as *mut c_void, (vtable.il2cpp_get_field_from_name)(item_class, c"_nameText".as_ptr()), &name_text_obj as *const _ as *mut c_void);
            if name_text_obj.is_null() { return; }

            let ui_image = (vtable.il2cpp_get_assembly_image)(c"UnityEngine.UI".as_ptr());
            let text_class = (vtable.il2cpp_get_class)(ui_image, c"UnityEngine.UI".as_ptr(), c"Text".as_ptr());
            let get_text: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppString = std::mem::transmute((vtable.il2cpp_get_method_addr_cached)(text_class, c"get_text".as_ptr(), 0));
            let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute((vtable.il2cpp_get_method_addr_cached)(text_class, c"set_text".as_ptr(), 1));

            let current_str = (*get_text(name_text_obj)).as_string();
            let base_str = current_str.split("<color=").next().unwrap_or(&current_str).trim_end();

            let new_text = if score > 0.0 && enable_scoring {
                format!("{} <color=#ffb000>[{:.2}pt]</color>", base_str, score)
            } else {
                base_str.to_string()
            };

            if new_text != current_str {
                let string_new: extern "C" fn(*const c_char) -> *mut Il2CppString = std::mem::transmute(IL2CPP_STRING_NEW.load(Ordering::Relaxed));
                set_text(name_text_obj, string_new(CString::new(new_text).unwrap().as_ptr()));
            }
        }
    }

    unsafe { apply_live_ui_updates(); }
}

#[cfg(target_os = "android")]
extern "C" fn parts_skill_list_item_setup_need_skill_point_hook(this: *mut Il2CppObject) {
    let orig_ptr = PARTS_SKILL_LIST_ITEM_SETUP_NEED_SKILL_POINT_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppObject) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this);
    }

    if this.is_null() { return; }

    let vtable = unsafe { &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2) };
    unsafe {
        let item_class = (*this).klass;

        let info_field = (vtable.il2cpp_get_field_from_name)(item_class, c"_info".as_ptr());
        if info_field.is_null() { return; }

        let mut skill_info: *mut Il2CppObject = std::ptr::null_mut();
        (vtable.il2cpp_get_field_value)(this as *mut c_void, info_field, &mut skill_info as *mut _ as *mut c_void);
        if skill_info.is_null() { return; }

        let info_class = (*skill_info).klass;
        let mut skill_id = 0;

        let get_id_addr = (vtable.il2cpp_get_method_addr_cached)(info_class, c"get_Id".as_ptr(), 0);
        if !get_id_addr.is_null() {
            let get_id: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(get_id_addr);
            skill_id = get_id(skill_info);
        } else {
            let backing_field = (vtable.il2cpp_get_field_from_name)(info_class, c"<Id>k__BackingField".as_ptr());
            if !backing_field.is_null() {
                (vtable.il2cpp_get_field_value)(skill_info as *mut c_void, backing_field, &mut skill_id as *mut _ as *mut c_void);
            }
        }

        if skill_id == 0 { return; }

        let new_handle: extern "C" fn(*mut Il2CppObject, bool) -> u32 = std::mem::transmute(IL2CPP_GCHANDLE_NEW.load(Ordering::Relaxed));
        let free_handle: extern "C" fn(u32) = std::mem::transmute(IL2CPP_GCHANDLE_FREE.load(Ordering::Relaxed));

        if let Ok(mut items) = TRACKED_INNER_ITEMS.lock() {
            items.retain(|&(handle, _)| {
                let obj = get_valid_target(handle);
                if obj.is_null() || obj == this { free_handle(handle); return false; }
                true
            });
            items.push((new_handle(this, false), skill_id));
        }

        let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
        let strategy = state.target_strategy;
        let enable_scoring = state.enable_scoring;
        let score = crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0);

        if score > 0.0 || !enable_scoring {
            let name_text_obj: *mut Il2CppObject = std::ptr::null_mut();
            (vtable.il2cpp_get_field_value)(this as *mut c_void, (vtable.il2cpp_get_field_from_name)(item_class, c"_nameText".as_ptr()), &name_text_obj as *const _ as *mut c_void);
            if name_text_obj.is_null() { return; }

            let ui_image = (vtable.il2cpp_get_assembly_image)(c"UnityEngine.UI".as_ptr());
            let text_class = (vtable.il2cpp_get_class)(ui_image, c"UnityEngine.UI".as_ptr(), c"Text".as_ptr());
            let get_text: extern "C" fn(*mut Il2CppObject) -> *mut crate::il2cpp_types::Il2CppString = std::mem::transmute((vtable.il2cpp_get_method_addr_cached)(text_class, c"get_text".as_ptr(), 0));
            let set_text: extern "C" fn(*mut Il2CppObject, *mut crate::il2cpp_types::Il2CppString) = std::mem::transmute((vtable.il2cpp_get_method_addr_cached)(text_class, c"set_text".as_ptr(), 1));

            let current_str = (*get_text(name_text_obj)).as_string();
            let base_str = current_str.split("<color=").next().unwrap_or(&current_str).trim_end();

            let new_text = if score > 0.0 && enable_scoring {
                format!("{} <color=#ffb000>[{:.2}pt]</color>", base_str, score)
            } else {
                base_str.to_string()
            };

            if new_text != current_str {
                let string_new: extern "C" fn(*const std::ffi::c_char) -> *mut crate::il2cpp_types::Il2CppString = std::mem::transmute(IL2CPP_STRING_NEW.load(Ordering::Relaxed));
                set_text(name_text_obj, string_new(std::ffi::CString::new(new_text).unwrap().as_ptr()));
            }
        }
    }

    unsafe { apply_live_ui_updates(); }
}

extern "C" fn parts_skill_list_container_update_hook(
    this: *mut Il2CppObject,
    info_list: *mut Il2CppObject,
    resource_hash: i32
) {
    if !info_list.is_null() {
        let vtable = unsafe { &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2) };
        unsafe {
            let list_class = (*info_list).klass;
            let size_field = (vtable.il2cpp_get_field_from_name)(list_class, c"_size".as_ptr());
            let items_field = (vtable.il2cpp_get_field_from_name)(list_class, c"_items".as_ptr());

            if !size_field.is_null() && !items_field.is_null() {
                let mut size: i32 = 0;
                (vtable.il2cpp_get_field_value)(info_list as _, size_field, &mut size as *mut _ as _);

                let mut items_array: *mut crate::il2cpp_types::Il2CppArray = std::ptr::null_mut();
                (vtable.il2cpp_get_field_value)(info_list as _, items_field, &mut items_array as *mut _ as _);

                if !items_array.is_null() && size > 1 {
                    let mut managed_by_global = false;
                    let core_module = (vtable.il2cpp_get_assembly_image)(c"UnityEngine.CoreModule".as_ptr());
                    let component_class = (vtable.il2cpp_get_class)(core_module, c"UnityEngine".as_ptr(), c"Component".as_ptr());
                    let transform_class = (vtable.il2cpp_get_class)(core_module, c"UnityEngine".as_ptr(), c"Transform".as_ptr());
                    let object_class = (vtable.il2cpp_get_class)(core_module, c"UnityEngine".as_ptr(), c"Object".as_ptr());

                    let get_game_object_addr = (vtable.il2cpp_get_method_addr_cached)(component_class, c"get_gameObject".as_ptr(), 0);
                    let get_transform_addr = (vtable.il2cpp_get_method_addr_cached)(component_class, c"get_transform".as_ptr(), 0);
                    let get_parent_addr = (vtable.il2cpp_get_method_addr_cached)(transform_class, c"get_parent".as_ptr(), 0);
                    let get_name_addr = (vtable.il2cpp_get_method_addr_cached)(object_class, c"get_name".as_ptr(), 0);

                    let mut container_transform: *mut Il2CppObject = std::ptr::null_mut();

                    if !get_game_object_addr.is_null() && !get_transform_addr.is_null() && !get_parent_addr.is_null() && !get_name_addr.is_null() {
                        let get_game_object: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_game_object_addr);
                        let get_transform: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_transform_addr);
                        let get_parent: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_parent_addr);
                        let get_name: extern "C" fn(*mut Il2CppObject) -> *mut crate::il2cpp_types::Il2CppString = std::mem::transmute(get_name_addr);

                        container_transform = get_transform(this);
                        let mut curr = container_transform;
                        for _ in 0..8 {
                            if curr.is_null() { break; }
                            let curr_go = get_game_object(curr);
                            if !curr_go.is_null() {
                                let name_str = get_name(curr_go);
                                if !name_str.is_null() {
                                    let name = (*name_str).as_string();
                                    if name.contains("Loop") {
                                        managed_by_global = true;
                                        break;
                                    }
                                }
                            }
                            curr = get_parent(curr);
                        }
                    }

                    if !managed_by_global {
                        let mut current_ids = Vec::with_capacity(size as usize);

                        for i in 0..size as usize {
                            let item_info = (*items_array).get_obj(i);
                            if item_info.is_null() { continue; }

                            let item_class = (*item_info).klass;
                            let mut skill_id = 0;

                            let get_id_addr = (vtable.il2cpp_get_method_addr_cached)(item_class, c"get_Id".as_ptr(), 0);
                            if !get_id_addr.is_null() {
                                let get_id: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(get_id_addr);
                                skill_id = get_id(item_info);
                            } else {
                                let backing_field = (vtable.il2cpp_get_field_from_name)(item_class, c"<Id>k__BackingField".as_ptr());
                                if !backing_field.is_null() {
                                    (vtable.il2cpp_get_field_value)(item_info as *mut c_void, backing_field, &mut skill_id as *mut _ as *mut c_void);
                                }
                            }
                            current_ids.push(skill_id);
                        }

                        if current_ids.len() == size as usize {
                            let mut order_map = CONTAINER_ORIGINAL_ORDER.lock().unwrap();
                            let list_ptr = if !container_transform.is_null() { container_transform as usize } else { info_list as usize };

                            let mut needs_insert = true;
                            if let Some(orig_ids) = order_map.get(&list_ptr) {
                                let mut c_sorted = current_ids.clone();
                                c_sorted.sort_unstable();
                                let mut o_sorted = orig_ids.clone();
                                o_sorted.sort_unstable();
                                if c_sorted == o_sorted {
                                    needs_insert = false;
                                }
                            }

                            if needs_insert {
                                order_map.insert(list_ptr, current_ids);
                            }
                        }
                    }
                }
            }
        }
    }

    let orig_ptr = PARTS_SKILL_LIST_CONTAINER_UPDATE_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject, i32) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this, info_list, resource_hash);
    }

    unsafe { apply_live_ui_updates(); }
}

extern "C" fn setup_skill_content_hook(
    this: *mut Il2CppObject,
    support_card_list: *mut Il2CppObject,
    single_mode_start_card_id: i32,
) {
    unsafe {
        let free_handle: extern "C" fn(u32) = std::mem::transmute(IL2CPP_GCHANDLE_FREE.load(Ordering::Relaxed));
        if let Ok(mut items) = TRACKED_DECK_ITEMS.lock() {
            for &(handle, _) in items.iter() { free_handle(handle); }
            items.clear();
        }
        crate::hooks::TRANSFORM_ORIGINAL_ORDER.lock().unwrap().clear();
    }

    let orig_ptr = SETUP_SKILL_CONTENT_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject, i32) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this, support_card_list, single_mode_start_card_id);
    }

    unsafe { apply_live_ui_updates(); }
}

extern "C" fn deck_skill_item_update_hook(
    this: *mut Il2CppObject,
    info: *mut Il2CppObject,
    resource_hash: usize
) {
    let orig_ptr = DECK_SKILL_ITEM_UPDATE_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject, usize) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this, info, resource_hash);
    }

    if info.is_null() || this.is_null() { return; }

    let vtable = unsafe { &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2) };
    unsafe {
        let info_class = (*info).klass;
        let mut skill_id = 0;

        let skill_id_field = (vtable.il2cpp_get_field_from_name)(info_class, c"SkillId".as_ptr());
        if !skill_id_field.is_null() {
            (vtable.il2cpp_get_field_value)(info as *mut c_void, skill_id_field, &mut skill_id as *mut _ as *mut c_void);
        }

        if skill_id == 0 { return; }

        let new_handle: extern "C" fn(*mut Il2CppObject, bool) -> u32 = std::mem::transmute(IL2CPP_GCHANDLE_NEW.load(Ordering::Relaxed));
        let free_handle: extern "C" fn(u32) = std::mem::transmute(IL2CPP_GCHANDLE_FREE.load(Ordering::Relaxed));

        if let Ok(mut items) = TRACKED_DECK_ITEMS.lock() {
            items.retain(|&(handle, _)| {
                let obj = get_valid_target(handle);
                if obj.is_null() || obj == this { free_handle(handle); return false; }
                true
            });
            items.push((new_handle(this, false), skill_id));
        }

        let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
        let strategy = state.target_strategy;
        let enable_scoring = state.enable_scoring;
        let score = crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0);

        if score > 0.0 || !enable_scoring {
            let item_class = (*this).klass;
            let name_text_obj: *mut Il2CppObject = std::ptr::null_mut();

            (vtable.il2cpp_get_field_value)(this as *mut c_void, (vtable.il2cpp_get_field_from_name)(item_class, c"_name".as_ptr()), &name_text_obj as *const _ as *mut c_void);
            if name_text_obj.is_null() { return; }

            let ui_image = (vtable.il2cpp_get_assembly_image)(c"UnityEngine.UI".as_ptr());
            let text_class = (vtable.il2cpp_get_class)(ui_image, c"UnityEngine.UI".as_ptr(), c"Text".as_ptr());
            let get_text: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppString = std::mem::transmute((vtable.il2cpp_get_method_addr_cached)(text_class, c"get_text".as_ptr(), 0));
            let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute((vtable.il2cpp_get_method_addr_cached)(text_class, c"set_text".as_ptr(), 1));

            let current_str = (*get_text(name_text_obj)).as_string();
            let base_str = current_str.split("<color=").next().unwrap_or(&current_str).trim_end();

            let new_text = if score > 0.0 && enable_scoring {
                format!("{} <color=#ffb000>[{:.2}pt]</color>", base_str, score)
            } else {
                base_str.to_string()
            };

            if new_text != current_str {
                let string_new: extern "C" fn(*const c_char) -> *mut Il2CppString = std::mem::transmute(IL2CPP_STRING_NEW.load(Ordering::Relaxed));
                set_text(name_text_obj, string_new(CString::new(new_text).unwrap().as_ptr()));
            }
        }
    }
}

pub unsafe fn sort_setup_parameter_list(setup_parameter: *mut Il2CppObject, vtable: &crate::plugin_api::VtableV2) {
    let param_class = (*setup_parameter).klass;
    let list_field = (vtable.il2cpp_get_field_from_name)(param_class, c"<SkillInfoList>k__BackingField".as_ptr());
    if list_field.is_null() { return; }

    let mut info_list: *mut Il2CppObject = std::ptr::null_mut();
    (vtable.il2cpp_get_field_value)(setup_parameter as _, list_field, &mut info_list as *mut _ as _);
    if info_list.is_null() { return; }

    let list_class = (*info_list).klass;
    let size_field = (vtable.il2cpp_get_field_from_name)(list_class, c"_size".as_ptr());
    let items_field = (vtable.il2cpp_get_field_from_name)(list_class, c"_items".as_ptr());

    if size_field.is_null() || items_field.is_null() { return; }

    let mut size: i32 = 0;
    (vtable.il2cpp_get_field_value)(info_list as _, size_field, &mut size as *mut _ as _);

    let mut items_array: *mut crate::il2cpp_types::Il2CppArray = std::ptr::null_mut();
    (vtable.il2cpp_get_field_value)(info_list as _, items_field, &mut items_array as *mut _ as _);

    if items_array.is_null() || size <= 1 { return; }

    let mut sortable_infos = Vec::with_capacity(size as usize);
    let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
    let strategy = state.target_strategy;
    let desc = state.sort_descending;
    let mode = state.sort_mode;
    let enable_scoring = state.enable_scoring;

    for i in 0..size as usize {
        let item_info = (*items_array).get_obj(i);
        if item_info.is_null() { continue; }

        let item_class = (*item_info).klass;
        let mut skill_id = 0;

        let get_skill_id_addr = (vtable.il2cpp_get_method_addr_cached)(item_class, c"get_SkillId".as_ptr(), 0);
        if !get_skill_id_addr.is_null() {
            let get_id: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(get_skill_id_addr);
            skill_id = get_id(item_info);
        } else {
            let get_id_addr = (vtable.il2cpp_get_method_addr_cached)(item_class, c"get_Id".as_ptr(), 0);
            if !get_id_addr.is_null() {
                let get_id: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(get_id_addr);
                skill_id = get_id(item_info);
            } else {
                let bf1 = (vtable.il2cpp_get_field_from_name)(item_class, c"<SkillId>k__BackingField".as_ptr());
                if !bf1.is_null() {
                    (vtable.il2cpp_get_field_value)(item_info as _, bf1, &mut skill_id as *mut _ as _);
                } else {
                    let bf2 = (vtable.il2cpp_get_field_from_name)(item_class, c"<Id>k__BackingField".as_ptr());
                    if !bf2.is_null() {
                        (vtable.il2cpp_get_field_value)(item_info as _, bf2, &mut skill_id as *mut _ as _);
                    }
                }
            }
        }

        let score = if skill_id > 0 { crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0) } else { 0.0 };
        sortable_infos.push((item_info, score, skill_id));
    }

    if sortable_infos.len() == size as usize {
        let mut order_map = SETUP_LIST_ORIGINAL_ORDER.lock().unwrap();
        let list_ptr = info_list as usize;

        let current_ids: Vec<i32> = sortable_infos.iter().map(|&(_, _, id)| id).collect();
        let mut needs_insert = false;

        if let Some(orig_ids) = order_map.get(&list_ptr) {
            let mut c_sorted = current_ids.clone();
            c_sorted.sort_unstable();
            let mut o_sorted = orig_ids.clone();
            o_sorted.sort_unstable();
            if c_sorted != o_sorted {
                needs_insert = true;
            }
        } else {
            needs_insert = true;
        }

        if needs_insert {
            order_map.insert(list_ptr, current_ids);
        }

        let orig_order = order_map.get(&list_ptr).unwrap().clone();
        drop(order_map);

        sortable_infos.sort_by(|a, b| {
            if !enable_scoring || mode == 3 {
                let idx_a = orig_order.iter().position(|&id| id == a.2).unwrap_or(usize::MAX);
                let idx_b = orig_order.iter().position(|&id| id == b.2).unwrap_or(usize::MAX);
                return idx_a.cmp(&idx_b);
            }
            if desc {
                b.1.total_cmp(&a.1)
            } else {
                a.1.total_cmp(&b.1)
            }
        });

        for (i, (ptr, _, _)) in sortable_infos.iter().enumerate() {
            (*items_array).set_obj(i, *ptr);
        }
    }
}

extern "C" fn parts_single_mode_skill_list_setup_hook(
    this: *mut Il2CppObject,
    setup_parameter: *mut Il2CppObject,
    resource_hash: i32
) {
    if !setup_parameter.is_null() {
        let vtable = unsafe { &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2) };
        unsafe {
            let new_handle: extern "C" fn(*mut Il2CppObject, bool) -> u32 = std::mem::transmute(IL2CPP_GCHANDLE_NEW.load(Ordering::Relaxed));
            let free_handle: extern "C" fn(u32) = std::mem::transmute(IL2CPP_GCHANDLE_FREE.load(Ordering::Relaxed));

            let old_list = ACTIVE_PARTS_LIST_HANDLE.swap(0, Ordering::SeqCst);
            if old_list != 0 { free_handle(old_list); }
            let old_param = ACTIVE_SETUP_PARAM_HANDLE.swap(0, Ordering::SeqCst);
            if old_param != 0 { free_handle(old_param); }

            ACTIVE_PARTS_LIST_HANDLE.store(new_handle(this, false), Ordering::SeqCst);
            ACTIVE_SETUP_PARAM_HANDLE.store(new_handle(setup_parameter, false), Ordering::SeqCst);
            ACTIVE_RESOURCE_HASH.store(resource_hash, Ordering::SeqCst);

            sort_setup_parameter_list(setup_parameter, vtable);
        }
    }

    let orig_ptr = PARTS_SINGLE_MODE_SKILL_LIST_SETUP_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject, i32) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this, setup_parameter, resource_hash);
    }
}

extern "C" fn factor_list_item_setup_hook(
    this: *mut Il2CppObject,
    factor: *mut Il2CppObject,
    on_click: *mut c_void,
    on_long_tap: *mut c_void,
    is_selected: bool,
    is_enabled: bool,
    is_hide_race_fit: bool,
    adjuster_data: *mut c_void
) {
    let orig_ptr = FACTOR_LIST_ITEM_SETUP_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject, *mut c_void, *mut c_void, bool, bool, bool, *mut c_void) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this, factor, on_click, on_long_tap, is_selected, is_enabled, is_hide_race_fit, adjuster_data);
    }

    if factor.is_null() || this.is_null() { return; }

    let vtable = unsafe { &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2) };
    unsafe {
        let factor_class = (*factor).klass;

        let mut factor_type: i32 = 0;
        let type_field = (vtable.il2cpp_get_field_from_name)(factor_class, c"FactorType".as_ptr());
        if !type_field.is_null() {
            (vtable.il2cpp_get_field_value)(factor as *mut c_void, type_field, &mut factor_type as *mut _ as *mut c_void);
        } else {
            factor_type = *((factor as *const u8).add(0x28) as *const i32);
        }

        if factor_type != 4 { return; }

        let mut factor_group_id: i32 = 0;
        let group_field = (vtable.il2cpp_get_field_from_name)(factor_class, c"FactorGroupId".as_ptr());
        if !group_field.is_null() {
            (vtable.il2cpp_get_field_value)(factor as *mut c_void, group_field, &mut factor_group_id as *mut _ as *mut c_void);
        } else {
            factor_group_id = *((factor as *const u8).add(0x14) as *const i32);
        }

        let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
        let strategy = state.target_strategy;
        let enable_scoring = state.enable_scoring;

        if !enable_scoring { return; }

        if let Some(db) = crate::data::SKILL_DB.as_ref() {
            if let Some(&skill_id) = db.factor_to_skill.get(&factor_group_id) {

                let score = crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0);

                if score > 0.0 {
                    let item_class = (*this).klass;
                    let name_text_obj: *mut Il2CppObject = std::ptr::null_mut();

                    (vtable.il2cpp_get_field_value)(this as *mut c_void, (vtable.il2cpp_get_field_from_name)(item_class, c"_factorName".as_ptr()), &name_text_obj as *const _ as *mut c_void);

                    if !name_text_obj.is_null() {
                        let ui_image = (vtable.il2cpp_get_assembly_image)(c"UnityEngine.UI".as_ptr());
                        let text_class = (vtable.il2cpp_get_class)(ui_image, c"UnityEngine.UI".as_ptr(), c"Text".as_ptr());
                        let get_text: extern "C" fn(*mut Il2CppObject) -> *mut crate::il2cpp_types::Il2CppString = std::mem::transmute((vtable.il2cpp_get_method_addr_cached)(text_class, c"get_text".as_ptr(), 0));
                        let set_text: extern "C" fn(*mut Il2CppObject, *mut crate::il2cpp_types::Il2CppString) = std::mem::transmute((vtable.il2cpp_get_method_addr_cached)(text_class, c"set_text".as_ptr(), 1));

                        let current_str = (*get_text(name_text_obj)).as_string();
                        let base_str = current_str.split("<color=").next().unwrap_or(&current_str).trim_end();

                        let new_text = format!("{} <color=#ffb000>[{:.2}pt]</color>", base_str, score);

                        if new_text != current_str {
                            let string_new: extern "C" fn(*const std::ffi::c_char) -> *mut crate::il2cpp_types::Il2CppString = std::mem::transmute(crate::hooks::IL2CPP_STRING_NEW.load(Ordering::Relaxed));
                            set_text(name_text_obj, string_new(std::ffi::CString::new(new_text).unwrap().as_ptr()));
                        }
                    }
                }
            }
        }
    }
}

extern "C" fn get_filtered_factor_group_list_hook(this: *mut Il2CppObject) -> *mut Il2CppObject {
    let orig_ptr = GET_FILTERED_FACTOR_GROUP_LIST_ORIG.load(Ordering::Relaxed);
    let orig_fn: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = unsafe { std::mem::transmute(orig_ptr) };

    let list_obj = orig_fn(this);

    if list_obj.is_null() { return list_obj; }

    let vtable = unsafe { &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2) };

    unsafe {
        let list_class = (*list_obj).klass;
        let size_field = (vtable.il2cpp_get_field_from_name)(list_class, c"_size".as_ptr());
        let items_field = (vtable.il2cpp_get_field_from_name)(list_class, c"_items".as_ptr());

        if size_field.is_null() || items_field.is_null() { return list_obj; }

        let mut size: i32 = 0;
        (vtable.il2cpp_get_field_value)(list_obj as *mut c_void, size_field, &mut size as *mut _ as *mut c_void);

        if size <= 1 { return list_obj; }

        let mut items_array: *mut crate::il2cpp_types::Il2CppArray = std::ptr::null_mut();
        (vtable.il2cpp_get_field_value)(list_obj as *mut c_void, items_field, &mut items_array as *mut _ as *mut c_void);

        if items_array.is_null() { return list_obj; }

        let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
        if !state.enable_scoring { return list_obj; }

        let strategy = state.target_strategy;
        let sort_descending = state.sort_descending;
        let mode = state.sort_mode;

        let mut sortable_factors = Vec::with_capacity(size as usize);

        for i in 0..size as usize {
            let factor_obj = (*items_array).get_obj(i);
            if factor_obj.is_null() { continue; }

            let factor_class = (*factor_obj).klass;

            let mut factor_type: i32 = 0;
            let type_field = (vtable.il2cpp_get_field_from_name)(factor_class, c"FactorType".as_ptr());
            if !type_field.is_null() {
                (vtable.il2cpp_get_field_value)(factor_obj as *mut c_void, type_field, &mut factor_type as *mut _ as *mut c_void);
            } else {
                factor_type = *((factor_obj as *const u8).add(0x28) as *const i32);
            }

            let mut score = 0.0;

            if factor_type == 4 {
                let mut factor_group_id: i32 = 0;
                let group_field = (vtable.il2cpp_get_field_from_name)(factor_class, c"FactorGroupId".as_ptr());
                if !group_field.is_null() {
                    (vtable.il2cpp_get_field_value)(factor_obj as *mut c_void, group_field, &mut factor_group_id as *mut _ as *mut c_void);
                } else {
                    factor_group_id = *((factor_obj as *const u8).add(0x14) as *const i32);
                }

                if let Some(db) = crate::data::SKILL_DB.as_ref() {
                    if let Some(&skill_id) = db.factor_to_skill.get(&factor_group_id) {
                        score = crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0);
                    }
                }
            }

            sortable_factors.push((factor_obj, score, factor_type));
        }

        if sortable_factors.len() == size as usize {
            if mode != 3 {
                sortable_factors.sort_by(|a, b| {
                    let cmp = if sort_descending {
                        b.1.total_cmp(&a.1)
                    } else {
                        a.1.total_cmp(&b.1)
                    };
                    cmp.then_with(|| a.2.cmp(&b.2))
                });

                for (i, (ptr, _, _)) in sortable_factors.iter().enumerate() {
                    (*items_array).set_obj(i, *ptr);
                }
            }
        }
    }

    list_obj
}

extern "C" fn factor_select_show_hook(this: *mut c_void) {
    FACTOR_SELECT_INSTANCE.store(this, Ordering::SeqCst);
    let orig_ptr = FACTOR_SELECT_SHOW_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut c_void) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this);
    }
}

extern "C" fn factor_select_hide_hook(this: *mut c_void, force: bool) {
    FACTOR_SELECT_INSTANCE.store(std::ptr::null_mut(), Ordering::SeqCst);
    let orig_ptr = FACTOR_SELECT_HIDE_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut c_void, bool) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this, force);
    }
}