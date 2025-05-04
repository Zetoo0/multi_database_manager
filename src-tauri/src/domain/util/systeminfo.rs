use os_info;
use os_info::Type;
use serde_json::{Map, Value};
use sys_locale::get_locale;

fn get_system_info() -> Map<String, Value> {
    let info = os_info::get();
    let mut map = Map::new();
    Map::insert(
        &mut map,
        "info".to_string(),
        Value::String(info.to_string()),
    );
    Map::insert(
        &mut map,
        "type".to_string(),
        Value::String(info.os_type().to_string()),
    );
    Map::insert(
        &mut map,
        "version".to_string(),
        Value::String(info.version().to_string()),
    );
    Map::insert(
        &mut map,
        "bitness".to_string(),
        Value::String(info.bitness().to_string()),
    );
    Map::insert(
        &mut map,
        "architecture".to_string(),
        Value::String(info.architecture().unwrap().to_string()),
    );
    return map;
}

fn get_locale_info() -> String {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
    format!("{}", locale)
}

fn get_monitor_info(monitor: tauri::Monitor) -> Map<String, Value> {
    let mut monitor_map = Map::new();

    if let Some(name) = monitor.name() {
        monitor_map.insert("name".to_string(), Value::String(name.to_string()));
    }

    let physical_size = monitor.size();
    monitor_map.insert(
        "width".to_string(),
        Value::Number(physical_size.width.into()),
    );
    monitor_map.insert(
        "height".to_string(),
        Value::Number(physical_size.height.into()),
    );

    let dpi = format!("{:.6}", monitor.scale_factor())
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();
    monitor_map.insert("scale".to_string(), Value::String(dpi));

    return monitor_map;
}

fn get_monitors(window: tauri::WebviewWindow) -> Map<String, Value> {
    let mut map = Map::new();
    let available_monitors = window.available_monitors().unwrap();

    for (i, monitor) in available_monitors.into_iter().enumerate() {
        map.insert(i.to_string(), Value::Object(get_monitor_info(monitor)));
    }

    if let Some(primary_monitor) = window.primary_monitor().unwrap() {
        map.insert(
            "primary".to_string(),
            Value::Object(get_monitor_info(primary_monitor)),
        );
    } else {
        map.insert("primary".to_string(), Value::Null);
    }
    // let primary_monitor = window.primary_monitor().unwrap().unwrap();
    // map.insert("primary".to_string(), Value::Object(get_monitor_info(primary_monitor)));

    return map;
}

pub fn get_os_info(window: tauri::WebviewWindow) -> Map<String, Value> {
    let mut map = Map::new();
    let os_info = get_system_info();
    Map::insert(&mut map, "os".to_string(), Value::Object(os_info));
    let locale = get_locale_info();
    Map::insert(&mut map, "locale".to_string(), Value::String(locale));
    let monitor_size = get_monitors(window);
    Map::insert(
        &mut map,
        "monitors".to_string(),
        Value::Object(monitor_size),
    );
    return map;
}

fn all_os_types() -> Vec<Type> {
    vec![
        Type::AIX,
        Type::AlmaLinux,
        Type::Alpaquita,
        Type::Alpine,
        Type::Amazon,
        Type::Android,
        Type::Arch,
        Type::Artix,
        Type::CentOS,
        Type::Debian,
        Type::DragonFly,
        Type::Emscripten,
        Type::EndeavourOS,
        Type::Fedora,
        Type::FreeBSD,
        Type::Garuda,
        Type::Gentoo,
        Type::HardenedBSD,
        Type::Illumos,
        Type::Kali,
        Type::Linux,
        Type::Mabox,
        Type::Macos,
        Type::Manjaro,
        Type::Mariner,
        Type::MidnightBSD,
        Type::Mint,
        Type::NetBSD,
        Type::NixOS,
        Type::OpenBSD,
        Type::OpenCloudOS,
        Type::openEuler,
        Type::openSUSE,
        Type::OracleLinux,
        Type::Pop,
        Type::Raspbian,
        Type::Redhat,
        Type::RedHatEnterprise,
        Type::Redox,
        Type::RockyLinux,
        Type::Solus,
        Type::SUSE,
        Type::Ubuntu,
        Type::Ultramarine,
        Type::Void,
        Type::Unknown,
        Type::Windows,
    ]
}

pub fn get_os_types() -> serde_json::Map<String, Value> {
    let mut map = serde_json::Map::new();
    let os_types = all_os_types();
    serde_json::Map::insert(
        &mut map,
        "os_types".to_string(),
        serde_json::Value::Array(
            os_types
                .iter()
                .map(|t| serde_json::Value::String(t.to_string()))
                .collect(),
        ),
    );
    return map;
}
