use super::prose_mark::ProseMirrorMark;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProseMirrorNode {
    Paragraph {
        content: Vec<ProseMirrorNode>,
    },
    Heading {
        content: Vec<ProseMirrorNode>,
        attrs: HeadingAttrs,
    },
    Blockquote {
        content: Vec<ProseMirrorNode>,
    },
    CodeBlock {
        content: Vec<ProseMirrorNode>,
    },

    OrderedList {
        content: Vec<ProseMirrorNode>,
        attrs: ListAttrs,
    },
    BulletList {
        content: Vec<ProseMirrorNode>,
    },
    ListItem {
        content: Vec<ProseMirrorNode>,
    },

    Text {
        text: String,
        marks: Option<Vec<ProseMirrorMark>>,
    },
    Image {
        attrs: ImageAttrs,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeadingAttrs {
    pub level: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageAttrs {
    pub src: String,
    pub alt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListAttrs {
    pub order: u32,
}
