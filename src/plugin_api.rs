use std::ffi::{c_char, c_void};

pub type HachimiGetApiFn = extern "C" fn(name: *const c_char) -> *mut c_void;

#[repr(i32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum InitResult {
    Error = 0,
    Ok = 1,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VtableV2 {
    pub hachimi_instance: unsafe extern "C" fn() -> *const c_void,
    pub hachimi_get_interceptor: unsafe extern "C" fn(this: *const c_void) -> *const c_void,

    pub interceptor_hook: unsafe extern "C" fn(
        this: *const c_void,
        orig_addr: *mut c_void,
        hook_addr: *mut c_void,
    ) -> *mut c_void,
    pub interceptor_hook_vtable: unsafe extern "C" fn(
        this: *const c_void,
        vtable: *mut *mut c_void,
        vtable_index: usize,
        hook_addr: *mut c_void,
    ) -> *mut c_void,
    pub interceptor_get_trampoline_addr: unsafe extern "C" fn(
        this: *const c_void,
        hook_addr: *mut c_void
    ) -> *mut c_void,
    pub interceptor_unhook: unsafe extern "C" fn(
        this: *const c_void,
        hook_addr: *mut c_void
    ) -> *mut c_void,

    pub il2cpp_resolve_symbol: unsafe extern "C" fn(name: *const c_char) -> *mut c_void,
    pub il2cpp_get_assembly_image: unsafe extern "C" fn(assembly_name: *const c_char) -> *const c_void,
    pub il2cpp_get_class: unsafe extern "C" fn(
        image: *const c_void,
        namespace: *const c_char,
        class_name: *const c_char,
    ) -> *mut c_void,

    pub il2cpp_get_method: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        args_count: i32,
    ) -> *const c_void,
    pub il2cpp_get_method_overload: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        params: *const c_void,
        param_count: usize,
    ) -> *const c_void,
    pub il2cpp_get_method_addr: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        args_count: i32,
    ) -> *mut c_void,
    pub il2cpp_get_method_overload_addr: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        params: *const c_void,
        param_count: usize,
    ) -> *mut c_void,
    pub il2cpp_get_method_cached: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        args_count: i32,
    ) -> *const c_void,
    pub il2cpp_get_method_addr_cached: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        args_count: i32,
    ) -> *mut c_void,

    pub il2cpp_find_nested_class: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char
    ) -> *mut c_void,
    pub il2cpp_get_field_from_name: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char
    ) -> *mut c_void,
    pub il2cpp_get_field_value: unsafe extern "C" fn(
        obj: *mut c_void,
        field: *mut c_void,
        out_value: *mut c_void,
    ),
    pub il2cpp_set_field_value: unsafe extern "C" fn(
        obj: *mut c_void,
        field: *mut c_void,
        value: *const c_void,
    ),
    pub il2cpp_get_static_field_value: unsafe extern "C" fn(
        field: *mut c_void,
        out_value: *mut c_void
    ),
    pub il2cpp_set_static_field_value: unsafe extern "C" fn(
        field: *mut c_void,
        value: *const c_void
    ),

    pub il2cpp_unbox: unsafe extern "C" fn(obj: *mut c_void) -> *mut c_void,
    pub il2cpp_get_main_thread: unsafe extern "C" fn() -> *mut c_void,
    pub il2cpp_get_attached_threads: unsafe extern "C" fn(out_size: *mut usize) -> *mut *mut c_void,
    pub il2cpp_schedule_on_thread: unsafe extern "C" fn(thread: *mut c_void, callback: unsafe extern "C" fn()),
    pub il2cpp_create_array: unsafe extern "C" fn(
        element_type: *mut c_void,
        length: usize
    ) -> *mut c_void,
    pub il2cpp_get_singleton_like_instance: unsafe extern "C" fn(class: *mut c_void) -> *mut c_void,

    pub log: unsafe extern "C" fn(level: i32, target: *const c_char, message: *const c_char),
    pub gui_register_menu_item: unsafe extern "C" fn(
        label: *const c_char,
        callback: Option<extern "C" fn(*mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_register_menu_section: unsafe extern "C" fn(
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_show_notification: unsafe extern "C" fn(message: *const c_char) -> bool,

    pub gui_ui_heading: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_label: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_small: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_separator: unsafe extern "C" fn(ui: *mut c_void) -> bool,
    pub gui_ui_button: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_small_button: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_checkbox: unsafe extern "C" fn(
        ui: *mut c_void,
        text: *const c_char,
        value: *mut bool
    ) -> bool,
    pub gui_ui_text_edit_singleline: unsafe extern "C" fn(
        ui: *mut c_void,
        buffer: *mut c_char,
        buffer_len: usize,
    ) -> bool,
    pub gui_ui_horizontal: unsafe extern "C" fn(
        ui: *mut c_void,
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_ui_grid: unsafe extern "C" fn(
        ui: *mut c_void,
        id: *const c_char,
        columns: usize,
        spacing_x: f32,
        spacing_y: f32,
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_ui_end_row: unsafe extern "C" fn(ui: *mut c_void) -> bool,
    pub gui_ui_colored_label: unsafe extern "C" fn(
        ui: *mut c_void,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        text: *const c_char,
    ) -> bool,
    pub gui_register_menu_item_icon: unsafe extern "C" fn(
        label: *const c_char,
        icon_uri: *const c_char,
        icon_ptr: *const u8,
        icon_len: usize,
    ) -> bool,
    pub gui_register_menu_section_with_icon: unsafe extern "C" fn(
        title: *const c_char,
        icon_uri: *const c_char,
        icon_ptr: *const u8,
        icon_len: usize,
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,

    pub android_dex_load: unsafe extern "C" fn(
        dex_ptr: *const u8,
        dex_len: usize,
        class_name: *const c_char,
    ) -> u64,
    pub android_dex_unload: unsafe extern "C" fn(handle: u64) -> bool,
    pub android_dex_call_static_noargs: unsafe extern "C" fn(
        handle: u64,
        method: *const c_char,
        sig: *const c_char
    ) -> bool,
    pub android_dex_call_static_string: unsafe extern "C" fn(
        handle: u64,
        method: *const c_char,
        sig: *const c_char,
        arg: *const c_char,
    ) -> bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VtableV3 {
    pub base: VtableV2,

    pub gui_ui_searchable_combobox: unsafe extern "C" fn(
        ui: *mut c_void,
        id_salt: *const c_char,
        selected_value: *mut i32,
        item_values: *const i32,
        item_labels: *const *const c_char,
        item_count: usize
    ) -> bool,
    pub gui_get_menu_width: unsafe extern "C" fn() -> f32,
    pub gui_set_menu_width: unsafe extern "C" fn(width: f32),
    pub hachimi_get_base_dir: unsafe extern "C" fn() -> *const c_char,
    pub hachimi_get_data_path: unsafe extern "C" fn() -> *const c_char,
}

impl VtableV3 {
    pub unsafe fn from_get_api(get_api: HachimiGetApiFn) -> Self {
        macro_rules! load {
            ($name:expr) => {
                std::mem::transmute(get_api($name.as_ptr()))
            };
        }

        Self {
            base: VtableV2 {
                hachimi_instance: load!(c"hachimi_instance"),
                hachimi_get_interceptor: load!(c"hachimi_get_interceptor"),
                interceptor_hook: load!(c"interceptor_hook"),
                interceptor_hook_vtable: load!(c"interceptor_hook_vtable"),
                interceptor_get_trampoline_addr: load!(c"interceptor_get_trampoline_addr"),
                interceptor_unhook: load!(c"interceptor_unhook"),
                il2cpp_resolve_symbol: load!(c"il2cpp_resolve_symbol"),
                il2cpp_get_assembly_image: load!(c"il2cpp_get_assembly_image"),
                il2cpp_get_class: load!(c"il2cpp_get_class"),
                il2cpp_get_method: load!(c"il2cpp_get_method"),
                il2cpp_get_method_overload: load!(c"il2cpp_get_method_overload"),
                il2cpp_get_method_addr: load!(c"il2cpp_get_method_addr"),
                il2cpp_get_method_overload_addr: load!(c"il2cpp_get_method_overload_addr"),
                il2cpp_get_method_cached: load!(c"il2cpp_get_method_cached"),
                il2cpp_get_method_addr_cached: load!(c"il2cpp_get_method_addr_cached"),
                il2cpp_find_nested_class: load!(c"il2cpp_find_nested_class"),
                il2cpp_get_field_from_name: load!(c"il2cpp_get_field_from_name"),
                il2cpp_get_field_value: load!(c"il2cpp_get_field_value"),
                il2cpp_set_field_value: load!(c"il2cpp_set_field_value"),
                il2cpp_get_static_field_value: load!(c"il2cpp_get_static_field_value"),
                il2cpp_set_static_field_value: load!(c"il2cpp_set_static_field_value"),
                il2cpp_unbox: load!(c"il2cpp_unbox"),
                il2cpp_get_main_thread: load!(c"il2cpp_get_main_thread"),
                il2cpp_get_attached_threads: load!(c"il2cpp_get_attached_threads"),
                il2cpp_schedule_on_thread: load!(c"il2cpp_schedule_on_thread"),
                il2cpp_create_array: load!(c"il2cpp_create_array"),
                il2cpp_get_singleton_like_instance: load!(c"il2cpp_get_singleton_like_instance"),
                log: load!(c"log"),
                gui_register_menu_item: load!(c"gui_register_menu_item"),
                gui_register_menu_section: load!(c"gui_register_menu_section"),
                gui_show_notification: load!(c"gui_show_notification"),
                gui_ui_heading: load!(c"gui_ui_heading"),
                gui_ui_label: load!(c"gui_ui_label"),
                gui_ui_small: load!(c"gui_ui_small"),
                gui_ui_separator: load!(c"gui_ui_separator"),
                gui_ui_button: load!(c"gui_ui_button"),
                gui_ui_small_button: load!(c"gui_ui_small_button"),
                gui_ui_checkbox: load!(c"gui_ui_checkbox"),
                gui_ui_text_edit_singleline: load!(c"gui_ui_text_edit_singleline"),
                gui_ui_horizontal: load!(c"gui_ui_horizontal"),
                gui_ui_grid: load!(c"gui_ui_grid"),
                gui_ui_end_row: load!(c"gui_ui_end_row"),
                gui_ui_colored_label: load!(c"gui_ui_colored_label"),
                gui_register_menu_item_icon: load!(c"gui_register_menu_item_icon"),
                gui_register_menu_section_with_icon: load!(c"gui_register_menu_section_with_icon"),
                android_dex_load: load!(c"android_dex_load"),
                android_dex_unload: load!(c"android_dex_unload"),
                android_dex_call_static_noargs: load!(c"android_dex_call_static_noargs"),
                android_dex_call_static_string: load!(c"android_dex_call_static_string"),
            },
            gui_ui_searchable_combobox: load!(c"gui_ui_searchable_combobox"),
            gui_get_menu_width: load!(c"gui_get_menu_width"),
            gui_set_menu_width: load!(c"gui_set_menu_width"),
            hachimi_get_base_dir: load!(c"hachimi_get_base_dir"),
            hachimi_get_data_path: load!(c"hachimi_get_data_path"),
        }
    }
}