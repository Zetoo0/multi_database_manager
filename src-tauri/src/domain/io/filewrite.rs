//use fast_log::plugin::file;
use std::path::{Path, PathBuf};
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use tokio::io::Result;
use tokio::task::JoinSet;

///params:lines,file name, line limit, dml
pub async fn write_out_with_chunks(
    stuffs: Vec<String>,
    file_name: String,
    limit: usize,
    is_dml: bool,
) -> Result<()> {
    log::info!("Writing out {} with chunks: {:?}", file_name, limit);
    let file_num = (stuffs.len() as f64 / limit as f64).ceil() as usize;
    let main_folder = if is_dml { "DML" } else { "DDL" };
    let base_path = Path::new(main_folder).join(&file_name);

    create_dir_all(&base_path).await?;

    let mut iter = stuffs.chunks(limit);
    let mut tasks = JoinSet::new();

    for i in 1..=file_num {
        let file_chunk = iter.next().unwrap().to_vec();
        let file_name = format!("{}-{}.sql", file_name, i);
        let file_path = base_path.join(&file_name);

        tasks.spawn(write_out_into_one_file(file_chunk, file_path));
    }

    while let Some(res) = tasks.join_next().await {
        res??;
    }

    Ok(())
}

pub async fn write_out_all_into_one_file_at_once(
    stuffs: Vec<String>,
    file_name: String,
    is_dml: bool,
) -> Result<()> {
    log::info!("Writing out all into one file at once: {:?}", file_name);
    // If the number of data is more than 10,000 write out with chunks
    if stuffs.len() > 10_000 {
        log::info!("Number of data is more than 10,000, write out with chunks");
        write_out_with_chunks(stuffs, file_name, 10_000, is_dml).await?;
    } else {
        log::info!("Number of data is less than 10,000, write out into one file");
        let main_folder = if is_dml { "DML" } else { "DDL" };
        let path = Path::new(main_folder).join(&file_name);

        create_dir_all(&path).await?;

        // Write all data into one file if the number of data is less than 10,000
        write_out_into_one_file(stuffs, path).await?;
    }

    Ok(())
}

pub async fn write_out_into_one_file(stuffs: Vec<String>, file_path: PathBuf) -> Result<()> {
    log::info!("Writing out into one file: {:?}", file_path);
    let mut file = File::create(&file_path).await?;
    file.write_all(stuffs.join("\n").as_bytes()).await?;
    Ok(())
}
