use std::ffi::{c_char, c_void, CString};
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::il2cpp_types::{Il2CppArray, Il2CppObject, Il2CppString};
use crate::plugin_api::VtableV2;
use crate::{get_real_target_addr, VTABLE_PTR};

pub static UPDATE_CURRENT_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static SETUP_SCROLL_LIST_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static PLAY_OUT_VIEW_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static EVENT_SYSTEM_UPDATE_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static DIALOG_SKILL_HINT_OPEN_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static PARTS_SKILL_LIST_ITEM_UPDATE_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static PARTS_SKILL_LIST_CONTAINER_UPDATE_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static DECK_SKILL_ITEM_UPDATE_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
pub static SETUP_SKILL_CONTENT_ORIG: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
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

    let class_item = (vtable.il2cpp_get_class)(image, c"Gallop".as_ptr(), c"PartsSingleModeSkillLearningListItem".as_ptr());
    let update_current_addr = (vtable.il2cpp_get_method_addr)(class_item, c"UpdateCurrent".as_ptr(), 0);
    UPDATE_CURRENT_ORIG.store((vtable.interceptor_hook)(
        interceptor, get_real_target_addr(update_current_addr as *mut u8), update_current_hook as *mut c_void
    ), Ordering::SeqCst);

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
    let update_item_addr = (vtable.il2cpp_get_method_addr)(class_skill_list_item, c"UpdateItem".as_ptr(), 3);
    if !update_item_addr.is_null() {
        PARTS_SKILL_LIST_ITEM_UPDATE_ORIG.store((vtable.interceptor_hook)(
            interceptor, crate::get_real_target_addr(update_item_addr as *mut u8), parts_skill_list_item_update_hook as *mut c_void
        ), Ordering::SeqCst);
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
}

pub fn trigger_list_refresh() {
    NEEDS_REFRESH.store(true, Ordering::SeqCst);
}

unsafe fn apply_live_ui_updates() {
    let vtable = &*(crate::VTABLE_PTR.load(Ordering::Relaxed) as *const crate::plugin_api::VtableV2);

    let state = { crate::data::OPTIMIZER_STATE.lock().unwrap().clone() };
    let strategy = state.target_strategy;
    let desc = state.sort_descending;

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

    update_and_sort_item_group(inner_items, strategy, desc, vtable, c"PartsSingleModeSkillListItem", c"_nameText");

    update_and_sort_item_group(deck_items, strategy, desc, vtable, c"PartsSupportCardDeckSkillListItem", c"_name");
}

unsafe fn update_and_sort_item_group(
    items: Vec<(usize, i32)>,
    strategy: i32,
    desc: bool,
    vtable: &crate::plugin_api::VtableV2,
    _class_name: &std::ffi::CStr,
    text_field_name: &std::ffi::CStr
) {
    if items.is_empty() { return; }

    let core_module = (vtable.il2cpp_get_assembly_image)(c"UnityEngine.CoreModule".as_ptr());
    let component_class = (vtable.il2cpp_get_class)(core_module, c"UnityEngine".as_ptr(), c"Component".as_ptr());
    let transform_class = (vtable.il2cpp_get_class)(core_module, c"UnityEngine".as_ptr(), c"Transform".as_ptr());
    let gameobject_class = (vtable.il2cpp_get_class)(core_module, c"UnityEngine".as_ptr(), c"GameObject".as_ptr());

    let get_game_object_addr = (vtable.il2cpp_get_method_addr_cached)(component_class, c"get_gameObject".as_ptr(), 0);
    let get_active_addr = (vtable.il2cpp_get_method_addr_cached)(gameobject_class, c"get_activeInHierarchy".as_ptr(), 0);
    let get_transform_addr = (vtable.il2cpp_get_method_addr_cached)(component_class, c"get_transform".as_ptr(), 0);
    let get_parent_addr = (vtable.il2cpp_get_method_addr_cached)(transform_class, c"get_parent".as_ptr(), 0);
    let get_sibling_index_addr = (vtable.il2cpp_get_method_addr_cached)(transform_class, c"GetSiblingIndex".as_ptr(), 0);
    let set_sibling_index_addr = (vtable.il2cpp_get_method_addr_cached)(transform_class, c"SetSiblingIndex".as_ptr(), 1);
    let get_child_count_addr = (vtable.il2cpp_get_method_addr_cached)(transform_class, c"get_childCount".as_ptr(), 0);

    let ui_image = (vtable.il2cpp_get_assembly_image)(c"UnityEngine.UI".as_ptr());
    let text_class = (vtable.il2cpp_get_class)(ui_image, c"UnityEngine.UI".as_ptr(), c"Text".as_ptr());
    let get_text_addr = (vtable.il2cpp_get_method_addr_cached)(text_class, c"get_text".as_ptr(), 0);
    let set_text_addr = (vtable.il2cpp_get_method_addr_cached)(text_class, c"set_text".as_ptr(), 1);

    let string_new: extern "C" fn(*const std::ffi::c_char) -> *mut Il2CppString = std::mem::transmute(IL2CPP_STRING_NEW.load(std::sync::atomic::Ordering::Relaxed));

    if get_transform_addr.is_null() || get_parent_addr.is_null() || get_sibling_index_addr.is_null()
        || set_sibling_index_addr.is_null() || get_child_count_addr.is_null() || get_text_addr.is_null()
        || set_text_addr.is_null() || get_game_object_addr.is_null() || get_active_addr.is_null() {
        return;
    }

    let get_game_object: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_game_object_addr);
    let get_active: extern "C" fn(*mut Il2CppObject) -> bool = std::mem::transmute(get_active_addr);
    let get_transform: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_transform_addr);
    let get_parent: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_parent_addr);
    let get_sibling_index: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(get_sibling_index_addr);
    let set_sibling_index: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(set_sibling_index_addr);
    let get_child_count: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(get_child_count_addr);
    let get_text: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppString = std::mem::transmute(get_text_addr);
    let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(set_text_addr);

    let mut active_items = Vec::new();

    for &(item_val, skill_id) in items.iter() {
        let item_ptr = item_val as *mut Il2CppObject;

        let go = get_game_object(item_ptr);
        if go.is_null() || !get_active(go) { continue; }

        let score = crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0);

        let item_class = (*item_ptr).klass;
        let name_field = (vtable.il2cpp_get_field_from_name)(item_class, text_field_name.as_ptr());

        if !name_field.is_null() {
            let mut name_text_obj: *mut Il2CppObject = std::ptr::null_mut();
            (vtable.il2cpp_get_field_value)(item_ptr as *mut std::ffi::c_void, name_field, &mut name_text_obj as *const _ as *mut std::ffi::c_void);

            if !name_text_obj.is_null() {
                let current_str = (*get_text(name_text_obj)).as_string();
                let base_str = current_str.split("<color=").next().unwrap_or(&current_str).trim_end();

                let new_text = if score > 0.0 {
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
            active_items.push((item_ptr, transform, parent, grandparent, score));
        }
    }

    let mut parent_counts: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    let mut grandparent_counts: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();

    for &(_, _, p, gp, _) in active_items.iter() {
        if !p.is_null() { *parent_counts.entry(p as usize).or_insert(0) += 1; }
        if !gp.is_null() { *grandparent_counts.entry(gp as usize).or_insert(0) += 1; }
    }

    let mut grouped_items: std::collections::HashMap<usize, Vec<(*mut Il2CppObject, *mut Il2CppObject, f64, i32)>> = std::collections::HashMap::new();

    for (item_ptr, transform, parent, grandparent, score) in active_items {
        if parent.is_null() { continue; }

        let p_count = *parent_counts.get(&(parent as usize)).unwrap_or(&0);
        let gp_count = if grandparent.is_null() { 0 } else { *grandparent_counts.get(&(grandparent as usize)).unwrap_or(&0) };

        let (sort_transform, group_parent) = if p_count > 1 {
            (transform, parent)
        } else if gp_count > 1 {
            (parent, grandparent)
        } else {
            let mut st = transform;
            let mut pr = parent;
            if get_child_count(pr) == 1 && !grandparent.is_null() {
                st = pr;
                pr = grandparent;
            }
            (st, pr)
        };

        let orig_idx = get_sibling_index(sort_transform);
        grouped_items.entry(group_parent as usize).or_default().push((item_ptr, sort_transform, score, orig_idx));
    }

    for (_, mut group) in grouped_items {
        group.sort_by_key(|&(_, _, _, idx)| idx);

        let mut runs: Vec<Vec<(*mut Il2CppObject, *mut Il2CppObject, f64, i32)>> = Vec::new();
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
            let orig_indices: Vec<i32> = run.iter().map(|&(_, _, _, idx)| idx).collect();

            run.sort_by(|a, b| {
                if desc {
                    b.2.total_cmp(&a.2)
                } else {
                    a.2.total_cmp(&b.2)
                }
            });

            for (i, &(_, sort_transform, _, _)) in run.iter().enumerate() {
                set_sibling_index(sort_transform, orig_indices[i]);
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

        unsafe { apply_live_ui_updates(); }
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

    let desc = state.sort_descending;
    let mode = state.sort_mode;

    sortable_infos.sort_by(|a, b| {
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

    let new_text = if individual_score > 0.0 && cost > 0 {
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
    };

    if new_text != current_str {
        let string_new: extern "C" fn(*const c_char) -> *mut Il2CppString = std::mem::transmute(IL2CPP_STRING_NEW.load(Ordering::Relaxed));
        set_text(name_text_obj, string_new(CString::new(new_text).unwrap().as_ptr()));
    }
}

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
            let mut found = false;
            items.retain(|&handle| {
                let obj = get_valid_target(handle);
                if obj.is_null() { free_handle(handle); return false; }
                if obj == this { found = true; }
                true
            });
            if !found { items.push(new_handle(this, false)); }
        }

        let vtable = &*(VTABLE_PTR.load(Ordering::Relaxed) as *const VtableV2);
        let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
        update_learning_item_text(this, vtable, &state);
    }
}

extern "C" fn dialog_skill_hint_open_hook(skill_data_array: *mut Il2CppArray) {
    if !skill_data_array.is_null() {
        unsafe {
            let slice = (*skill_data_array).get_i32_mut_slice();

            let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
            let strategy = state.target_strategy;
            let desc = state.sort_descending;

            slice.sort_by(|a, b| {
                let score_a = crate::data::get_skill_score(*a, strategy).unwrap_or(0.0);
                let score_b = crate::data::get_skill_score(*b, strategy).unwrap_or(0.0);

                if desc {
                    score_b.total_cmp(&score_a)
                } else {
                    score_a.total_cmp(&score_b)
                }
            });
        }
    }

    let orig_ptr = DIALOG_SKILL_HINT_OPEN_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppArray) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(skill_data_array);
    }
}

extern "C" fn parts_skill_list_item_update_hook(
    this: *mut Il2CppObject,
    skill_info: *mut Il2CppObject,
    is_plate_effect_enable: bool,
    resource_hash: i32
) {
    let orig_ptr = PARTS_SKILL_LIST_ITEM_UPDATE_ORIG.load(Ordering::Relaxed);
    if !orig_ptr.is_null() {
        let orig_fn: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject, bool, i32) = unsafe { std::mem::transmute(orig_ptr) };
        orig_fn(this, skill_info, is_plate_effect_enable, resource_hash);
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
            let mut found = false;
            items.retain(|&(handle, _)| {
                let obj = get_valid_target(handle);
                if obj.is_null() { free_handle(handle); return false; }
                if obj == this { found = true; }
                true
            });
            if !found { items.push((new_handle(this, false), skill_id)); }
        }

        let strategy = crate::data::OPTIMIZER_STATE.lock().unwrap().target_strategy;
        let score = crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0);

        if score > 0.0 {
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

            let new_text = format!("{} <color=#ffb000>[{:.2}pt]</color>", base_str, score);

            if new_text != current_str {
                let string_new: extern "C" fn(*const c_char) -> *mut Il2CppString = std::mem::transmute(IL2CPP_STRING_NEW.load(Ordering::Relaxed));
                set_text(name_text_obj, string_new(CString::new(new_text).unwrap().as_ptr()));
            }
        }
    }
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
                    let mut sortable_infos = Vec::with_capacity(size as usize);
                    let state = crate::data::OPTIMIZER_STATE.lock().unwrap();
                    let strategy = state.target_strategy;
                    let desc = state.sort_descending;

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

                        let score = if skill_id > 0 { crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0) } else { 0.0 };
                        sortable_infos.push((item_info, score));
                    }

                    if sortable_infos.len() == size as usize {
                        sortable_infos.sort_by(|a, b| {
                            if desc {
                                b.1.total_cmp(&a.1)
                            } else {
                                a.1.total_cmp(&b.1)
                            }
                        });

                        for (i, (ptr, _)) in sortable_infos.iter().enumerate() {
                            (*items_array).set_obj(i, *ptr);
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
            let mut found = false;
            items.retain(|&(handle, _)| {
                let obj = get_valid_target(handle);
                if obj.is_null() { free_handle(handle); return false; }
                if obj == this { found = true; }
                true
            });
            if !found { items.push((new_handle(this, false), skill_id)); }
        }

        let strategy = crate::data::OPTIMIZER_STATE.lock().unwrap().target_strategy;
        let score = crate::data::get_skill_score(skill_id, strategy).unwrap_or(0.0);

        if score > 0.0 {
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

            let new_text = format!("{} <color=#ffb000>[{:.2}pt]</color>", base_str, score);

            if new_text != current_str {
                let string_new: extern "C" fn(*const c_char) -> *mut Il2CppString = std::mem::transmute(IL2CPP_STRING_NEW.load(Ordering::Relaxed));
                set_text(name_text_obj, string_new(CString::new(new_text).unwrap().as_ptr()));
            }
        }
    }
}