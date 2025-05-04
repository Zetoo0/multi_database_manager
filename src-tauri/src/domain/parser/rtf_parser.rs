use crate::domain::serializable::{
    prosemirror::{
        prose_doc::ProseMirrorDoc, prose_mark::ProseMirrorMark, prose_node::ProseMirrorNode,
    },
    response::{DataType, FileResponseBody, FileType, ParseType, ResponseBody},
};
use log::debug;
use rtf_parser::{Lexer, Parser, RtfDocument, Token};
use serde_json;
use std::error::Error;

pub fn parse(file: &str) -> ResponseBody {
    let doc: RtfDocument = parse_rtf(&file).unwrap();

    debug!("{:#?}", doc.header);
    let document = convert_rtf_to_prosemirror(doc);

    let file_response = FileResponseBody {
        file_type: FileType::Rtf,
        parsed_to: Some(ParseType::ProseJson),
        // content: serde_json::to_value(doc).unwrap(),
        content: serde_json::to_value(document).unwrap(),
    };

    return ResponseBody::new(DataType::FileResponse(file_response));
}
fn convert_rtf_to_prosemirror(rtf_doc: RtfDocument) -> ProseMirrorDoc {
    let mut content = Vec::new();

    for block in rtf_doc.body {
        let mut paragraph_content = Vec::new();
        let mut marks = Vec::new();

        // Handle bold, italic, underline
        if block.painter.bold {
            marks.push(ProseMirrorMark {
                mark_type: "bold".to_string(),
                attrs: None,
            });
        }
        if block.painter.italic {
            marks.push(ProseMirrorMark {
                mark_type: "italic".to_string(),
                attrs: None,
            });
        }
        if block.painter.underline {
            marks.push(ProseMirrorMark {
                mark_type: "underline".to_string(),
                attrs: None,
            });
        }

        marks.push(ProseMirrorMark {
            mark_type: "textStyle".to_string(),
            attrs: Some({
                let mut attrs = std::collections::HashMap::new();
                if let Some(color) = rtf_doc.header.color_table.get(&block.painter.color_ref) {
                    let color_hex =
                        format!("#{:02X}{:02X}{:02X}", color.red, color.green, color.blue);
                    attrs.insert("color".to_string(), color_hex);
                } else {
                    eprintln!("Color not found for color_ref: {}", block.painter.color_ref);
                }

                if let Some(font) = rtf_doc.header.font_table.get(&block.painter.font_ref) {
                    let font_family = font.name.clone();
                    attrs.insert("fontFamily".to_string(), font_family);
                } else {
                    eprintln!("Font not found for font_ref: {}", block.painter.font_ref);
                }

                attrs.insert(
                    "fontSize".to_string(),
                    format!("{}pt", block.painter.font_size),
                );
                attrs
            }),
        });

        // Create a text node with marks
        let text_node = ProseMirrorNode::Text {
            text: block.text.clone(),
            marks: if marks.is_empty() { None } else { Some(marks) },
        };

        // Add the text node to paragraph content
        paragraph_content.push(text_node);

        // Create a paragraph node
        let paragraph_node = ProseMirrorNode::Paragraph {
            content: paragraph_content,
        };

        // Add paragraph node to document content
        content.push(paragraph_node);
    }

    // Return the final ProseMirror document
    ProseMirrorDoc {
        doc_type: "doc".to_string(),
        content,
    }
}

fn parse_rtf(content: &str) -> Result<RtfDocument, Box<dyn Error>> {
    let tokens: Vec<Token> = Lexer::scan(content)?;
    let mut parser = Parser::new(tokens);
    let doc: RtfDocument = parser.parse()?;

    debug!("{:?}", doc);

    return Ok(doc);
}
