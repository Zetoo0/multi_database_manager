use crate::domain::{
    serializable::response::{DataType, ResponseBody},
    util::systeminfo,
};
use font_loader::system_fonts;
use log::debug;
use serde_json;
use sysinfo::{Pid, System};

#[tauri::command]
pub fn get_os_information(window: tauri::WebviewWindow) -> ResponseBody {
    debug!("Getting OS information");
    return ResponseBody::new(DataType::Object(
        serde_json::to_value(systeminfo::get_os_info(window)).unwrap(),
    ));
}

#[tauri::command]
pub fn get_vram_usage() -> u64 {
    let pid: u32 = std::process::id();
    let s = System::new_all();

    if let Some(process) = s.process(Pid::from(pid as usize)) {
        let vm: u64 = process.virtual_memory();
        // debug!("Memory: {} MB", process.memory() / 8 / 1024 / 1024 / 1024);
        // debug!("Virtual memory: {} MB", vm / 8 / 1024 / 1024 / 1024);
        return vm;
    }

    return 0;
}

#[tauri::command]
pub fn get_os_types() -> String {
    debug!("Getting OS types");
    return serde_json::to_string(&systeminfo::get_os_types()).unwrap();
}

#[tauri::command]
pub fn get_system_fonts() -> Vec<String> {
    let available_fonts = system_fonts::query_all();
    available_fonts
}
