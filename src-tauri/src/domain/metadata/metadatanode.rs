use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MetadataNode {
    name: String,
    children: Vec<MetadataNode>,
}

impl MetadataNode {
    pub fn new(name: &str, children: Vec<MetadataNode>) -> Self {
        MetadataNode {
            name: name.to_string(),
            children,
        }
    }
}
