use crate::domain::serializable::response::{
    DataType, FileResponseBody, FileType, ParseType, ResponseBody,
};
use comrak::{markdown_to_html, Options};
use serde_json::Value;

pub fn parse(file: &str) -> ResponseBody {
    let html = markdown_to_html(&file, &Options::default());
    let file_response_body = FileResponseBody {
        file_type: FileType::Md,
        parsed_to: Some(ParseType::Html),
        content: Value::String(html),
    };
    return ResponseBody::new(DataType::FileResponse(file_response_body));
}
