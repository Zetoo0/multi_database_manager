use super::prose_node::ProseMirrorNode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProseMirrorDoc {
    #[serde(rename = "type", default)]
    pub doc_type: String,
    pub content: Vec<ProseMirrorNode>,
}

impl Default for ProseMirrorDoc {
    fn default() -> Self {
        ProseMirrorDoc {
            doc_type: "doc".to_string(),
            content: vec![ProseMirrorNode::Paragraph { content: vec![] }],
        }
    }
}
