use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct ResponseBody {
    pub data: DataType,
    pub error: Option<ErrorBody>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DataType {
    FileResponse(FileResponseBody),
    Object(Value),
    None,
}

impl ResponseBody {
    pub fn new(data: DataType) -> Self {
        ResponseBody {
            data,
            error: Default::default(),
        }
    }

    pub fn new_with_error(error: ErrorBody) -> Self {
        ResponseBody {
            data: DataType::None,
            error: Some(error),
        }
    }
}

impl Default for ResponseBody {
    fn default() -> Self {
        ResponseBody {
            data: DataType::None,
            error: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct ErrorBody {
    pub code: u32,
    pub message: String,
}

impl ErrorBody {
    pub fn new(code: u32, message: String) -> Self {
        ErrorBody { code, message }
    }
}

impl Default for ErrorBody {
    fn default() -> Self {
        ErrorBody {
            code: 0,
            message: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct FileResponseBody {
    pub file_type: FileType,
    pub parsed_to: Option<ParseType>,
    pub content: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Rtf,
    Md,
    Docx,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParseType {
    Text,
    Html,
    ProseJson,
    Unknown,
}

impl Default for FileResponseBody {
    fn default() -> Self {
        FileResponseBody {
            file_type: FileType::Unknown,
            parsed_to: None,
            content: Value::Null,
        }
    }
}
