use std::path::PathBuf;

use crate::domain::{
    parser::{docx_parser, md_parser, rtf_parser},
    serializable::response::ResponseBody,
};
use log::debug;

/*
#[tauri::command]
pub fn get_parsed_md() -> ResponseBody {
    debug!("Getting parsed md");

    let res = include_str!("../../resources/sample.md");
    // let file = match res {
    //     Ok(file) => file,
    //     Err(e) => {
    //         error!("Error reading file: {:?}", e);
    //         let error_body = ErrorBody {
    //             code: 1,
    //             message: "Error reading file".to_string(),
    //         };
    //         return ResponseBody::new_with_error(error_body);
    //     }
    // };

    return md_parser::parse(&res);
}

#[tauri::command]
pub fn get_parsed_rtf() -> ResponseBody {
    debug!("Getting parsed rtf");

    let res = include_str!("../../resources/file-sample_100kB.rtf");
    // let file = match res {
    //     Ok(file) => file,
    //     Err(e) => {
    //         error!("Error reading file: {:?}", e);
    //         let error_body = ErrorBody {
    //             code: 1,
    //             message: "Error reading file".to_string(),
    //         };
    //         return ResponseBody::new_with_error(error_body);
    //     }
    // };
    return rtf_parser::parse(&res);
}*/

#[tauri::command]
pub fn get_parsed_docx() -> ResponseBody {
    debug!("Getting parsed docx");
    let mut path = PathBuf::from(".");

    path.push("src");
    path.push("resources");
    path.push("file-sample_1MB.docx");
    return docx_parser::parse(path);
}

// #[tauri::command]
// pub fn get_parsed_doc() -> ResponseBody {
//     debug!("Getting parsed doc");
//     return rtf_parser::parse(&res);
// }
