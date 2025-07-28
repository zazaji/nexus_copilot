// src-tauri/src/knowledge_base/parser.rs
use crate::error::{AppError, Result};
// Removed unused 'Docx' import
use docx_rs::{read_docx, ParagraphChild, RunChild};
use std::path::Path;

pub fn parse_file(path: &Path) -> Result<String> {
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    match extension.to_lowercase().as_str() {
        "txt" | "md" | "rs" | "js" | "ts" | "py" | "html" | "css" => {
            std::fs::read_to_string(path).map_err(|e| e.into())
        }
        "pdf" => {
            pdf_extract::extract_text(path).map_err(|e| AppError::Parse(e.to_string()))
        }
        "docx" => {
            let content = std::fs::read(path)?;
            let docx = read_docx(&content).map_err(|e| AppError::Parse(e.to_string()))?;
            let text_content = docx
                .document
                .children
                .iter()
                .filter_map(|child| {
                    if let docx_rs::DocumentChild::Paragraph(p) = child {
                        let text: String = p.children.iter().filter_map(|p_child| {
                            if let ParagraphChild::Run(r) = p_child {
                                Some(r.children.iter().filter_map(|r_child| {
                                    if let RunChild::Text(t) = r_child {
                                        Some(t.text.clone())
                                    } else { None }
                                }).collect::<String>())
                            } else { None }
                        }).collect();
                        Some(text)
                    } else { None }
                })
                .collect::<Vec<_>>()
                .join("\n");
            Ok(text_content)
        }
        _ => Err(AppError::Parse(format!("Unsupported file type: {}", extension))),
    }
}