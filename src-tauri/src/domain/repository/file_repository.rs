use log::debug;
use std::{error::Error, io::Read, path::PathBuf};

// pub fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
//     debug!("reading file: {}", path);
//     let contents = std::fs::read_to_string(path)?;
//     return Ok(contents);
// }

pub fn read_file(path: PathBuf) -> Result<String, Box<dyn Error>> {
    debug!("reading file: {:?}", path);
    let contents = std::fs::read_to_string(path)?;
    return Ok(contents);
}

// pub fn write_file(path: &str, content: &str) -> Result<(), Box<dyn Error>> {
//     debug!("writing file: {}", path);
//     std::fs::write(path, content)?;
//     return Ok(());
// }

pub fn write_file(path: PathBuf, content: &str) -> Result<(), Box<dyn Error>> {
    debug!("writing file: {:?}", path);
    std::fs::write(path, content)?;
    return Ok(());
}

pub fn get_config_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    if cfg!(debug_assertions) {
        path = PathBuf::from(".");
    }
    path.push(".officepad");
    path.push("writepad");

    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }

    path
}

pub fn read_to_vec(path: PathBuf) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    std::fs::File::open(path)?.read_to_end(&mut buf)?;
    Ok(buf)
}
