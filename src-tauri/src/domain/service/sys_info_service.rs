use crate::domain::repository::file_repository::get_config_dir;
use log::debug;
use serde_json::{Map, Value};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[cfg(target_os = "android")]
fn get_unique_device_id() -> String {
    // Try to get Android's hardware ID first, fallback to UUID if necessary
    use std::process::Command;
    if let Ok(output) = Command::new("getprop").arg("ro.serialno").output() {
        let serial = String::from_utf8_lossy(&output.stdout);
        let serial = serial.trim();
        if !serial.is_empty() {
            return serial.to_string();
        }
    }
    // Fallback to a generated UUID
    generate_or_read_uuid().unwrap_or_else(|| "unknown-android-id".to_string())
}

#[cfg(target_os = "ios")]
fn get_unique_device_id() -> String {
    // iOS does not allow access to hardware serial numbers; use fallback UUID
    generate_or_read_uuid().unwrap_or_else(|| "unknown-ios-id".to_string())
}

#[cfg(target_os = "windows")]
fn get_unique_device_id() -> String {
    // Try to get the motherboard serial, fallback to UUID
    use std::process::Command;
    if let Ok(output) = Command::new("wmic")
        .args(&["baseboard", "get", "SerialNumber"])
        .output()
    {
        let serial = String::from_utf8_lossy(&output.stdout);
        if let Some(serial_num) = serial.trim().lines().nth(1) {
            if !serial_num.is_empty() {
                return serial_num.to_string();
            }
        }
    }
    // Fallback to a generated UUID
    generate_or_read_uuid().unwrap_or_else(|| "unknown-windows-id".to_string())
}

#[cfg(target_os = "linux")]
fn get_unique_device_id() -> String {
    // Try to get the motherboard serial, fallback to UUID
    if let Ok(serial) = fs::read_to_string("/sys/class/dmi/id/board_serial") {
        if !serial.trim().is_empty() {
            return serial.trim().to_string();
        }
    }
    // Fallback to a generated UUID
    generate_or_read_uuid().unwrap_or_else(|| "unknown-linux-id".to_string())
}

#[cfg(target_os = "macos")]
fn get_unique_device_id() -> String {
    use std::process::Command;
    if let Ok(output) = Command::new("ioreg")
        .args(&["-rd1", "-c", "IOPlatformExpertDevice"])
        .output()
    {
        let serial = String::from_utf8_lossy(&output.stdout);
        if let Some(uuid_str) = serial
            .split("\"IOPlatformUUID\" = ")
            .nth(1)
            .map(|s| s.split("\"").nth(1))
        {
            return uuid_str.unwrap_or("").to_string();
        } else {
            println!("err");
        }
    }
    // Fallback to a generated UUID
    generate_or_read_uuid().unwrap_or_else(|| "unknown-macos-id".to_string())
}

// Fallback UUID generator and storage
fn generate_or_read_uuid() -> Option<String> {
    let file_path = get_uuid_file_path();
    if file_path.exists() {
        // If the UUID file exists, read it
        if let Ok(uuid) = fs::read_to_string(&file_path) {
            debug!("Read UUID from file: {}", uuid.trim());
            return Some(uuid.trim().to_string());
        }
    }

    // If the UUID file doesn't exist, generate a new UUID and store it
    let new_uuid = Uuid::new_v4().to_string();
    if fs::write(&file_path, &new_uuid).is_ok() {
        debug!("Generated new UUID: {}", new_uuid);
        return Some(new_uuid);
    }
    debug!("Failed to generate UUID");
    None
}

// Get the file path where the UUID is stored
fn get_uuid_file_path() -> PathBuf {
    let mut path = get_config_dir();
    path.push("id");
    path
}

fn get_sysinfo_file_path() -> PathBuf {
    let mut path = get_config_dir();
    path.push("sysinfo.json");
    path
}

fn xor_cipher(data: &str, key: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ key).collect()
}

fn encrypt_data(plaintext: &str) -> String {
    let key: u8 = 42; // Simple fixed key for XOR, can be any number
    let encrypted_bytes = xor_cipher(plaintext, key);
    base64::encode(encrypted_bytes) // Encode the result in Base64 for easy transmission
}

fn decrypt_data(ciphertext: &str) -> String {
    let key: u8 = 42; // Use the same fixed key for decryption
    let decoded_bytes = base64::decode(ciphertext).expect("Invalid base64 string");
    let decrypted_bytes: Vec<u8> = xor_cipher(&String::from_utf8_lossy(&decoded_bytes), key);
    String::from_utf8(decrypted_bytes).expect("Invalid UTF-8 data")
}

fn get_os_info() -> String {
    let os = std::env::consts::OS;
    let version = std::env::consts::ARCH; // Architecture for a rough version approximation
    format!("{} {}", os, version)
}

fn get_current_datetime() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    format!("{}", since_the_epoch.as_secs()) // Unix timestamp for simplicity
}

fn concatenate_data() -> String {
    let datetime = get_current_datetime();
    let os_info = get_os_info();
    let unique_id = get_unique_device_id();

    format!("{}#{}#{}", datetime, os_info, unique_id)
}

fn get_data() {
    // Concatenate the data (datetime#os type and version#unique id)
    let data = concatenate_data();
    println!("Original Data: {}", data);

    // Encrypt the data
    let encrypted_data = encrypt_data(&data);
    println!("Encrypted Data: {}", encrypted_data);

    // Decrypt the data
    let decrypted_data = decrypt_data(&encrypted_data);
    println!("Decrypted Data: {}", decrypted_data);
}

pub fn get_encrypted_data() -> String {
    let data = concatenate_data();
    encrypt_data(&data)
}

pub fn save_sys_info(window: tauri::WebviewWindow) {
    let mut map = Map::new();
    map.insert("sysinfo".to_string(), Value::String(get_encrypted_data()));
    map.append(&mut crate::domain::util::systeminfo::get_os_info(window));

    let file_path = get_sysinfo_file_path();
    if !file_path.exists() {
        fs::write(file_path, serde_json::to_string(&map).unwrap()).expect("Unable to write file");
    }
}
